#!/usr/bin/env bash
set -euo pipefail
host="${HOST:-http://localhost:8080}"
echo "[1] Sem unit_ref -> ASK + PoI"
curl -s "$host/v1/run" -H 'Content-Type: application/json' -d '{"input":{"actor":{"role":"rp"}}}' | jq '{decision, poi, runtime_used}'

echo "[2] Exigir runtime sem cert -> NACK + PoI(runtime_cert)"
unset RUNTIME_CERT_PATH || true
curl -s "$host/v1/run" -H 'Content-Type: application/json' -d '{"unit_ref":"cid:b3:UNIT...","input":{},"options":{"require_certified_runtime":true}}' | jq '{decision, poi, runtime_used}'

echo "[3] Sem exigir runtime -> recibo válido sem EER"
export RUNTIME_CERT_PATH=${RUNTIME_CERT_PATH:-/etc/tdln/runtime.cert.v1.json}
curl -s "$host/v1/run" -H 'Content-Type: application/json' -d '{"unit_ref":"cid:b3:UNIT...","input":{},"options":{"require_certified_runtime":false}}' | jq '{decision, runtime_used, has_runtime: .runtime != null}'

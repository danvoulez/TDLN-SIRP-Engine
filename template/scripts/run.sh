#!/usr/bin/env bash
set -euo pipefail
: "${TDLN_HOST:?TDLN_HOST must be set, e.g. https://engine.example.com}"
BODY="${1:-./examples/run.json}"
curl -s -X POST "$TDLN_HOST/v1/run" -H 'Content-Type: application/json' --data-binary "@${BODY}" | jq '.'

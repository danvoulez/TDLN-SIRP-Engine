#!/usr/bin/env bash
set -euo pipefail

: "${TDLN_HOST:?set TDLN_HOST=https://<engine-host>}"
APP_NAME="${1:-my-app}"

ROOT="$(cd "$(dirname "$0")"/.. && pwd)"
TDLN_DIR="$ROOT/tdln"
mkdir -p "$TDLN_DIR"

# Keygen (prefer Node noble, fallback to Python)
if command -v node >/dev/null 2>&1; then
  OUT_JSON=$(node "$ROOT/scripts/keys.js")
else
  OUT_JSON=$(python3 "$ROOT/scripts/keys.py")
fi

SEED_B64=$(echo "$OUT_JSON" | jq -r '.seed_b64')
PUB_B64=$(echo "$OUT_JSON" | jq -r '.pub_b64')
if [ -z "$PUB_B64" ] || [ "$PUB_B64" = "bnVsbA==" ]; then
  echo "WARN: missing pubkey (fallback mode). Install node + @noble/curves for full signing."
fi

# DID from BLAKE3(pub)
PUB_HEX=$(echo "$PUB_B64" | base64 -d | openssl dgst -blake2b256 | awk '{print $2}')
APP_DID="did:tdln:app:${PUB_HEX}"

# Write tdln.json
jq -n --arg host "$TDLN_HOST" --arg name "$APP_NAME" --arg did "$APP_DID" --arg pub "$PUB_B64" --arg seed "$SEED_B64" '{
  host: $host,
  app: { name: $name, did: $did, pubkey_ed25519_b64: $pub, seed_ed25519_b64: $seed }
}' > "$TDLN_DIR/tdln.json"

# .env
ENVF="$ROOT/.env"
grep -v '^TDLN_HOST=' "$ROOT/.env.example" > "$ENVF" 2>/dev/null || cp "$ROOT/.env.example" "$ENVF"
echo "TDLN_HOST=$TDLN_HOST" >> "$ENVF"

# Optional register
STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$TDLN_HOST/v1/apps/register"   -H 'Content-Type: application/json'   --data "{"name":"$APP_NAME","did":"$APP_DID","pubkey_b64":"$PUB_B64"}")
echo "register app -> HTTP $STATUS (ignored if 404)"

echo "OK: provisioned for $APP_NAME"
cat "$TDLN_DIR/tdln.json"

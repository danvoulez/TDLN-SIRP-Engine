#!/usr/bin/env bash
set -euo pipefail

BASE_DIR="$(cd "$(dirname "$0")" && pwd)"
TPL_DIR="$BASE_DIR/template_base"
WRAP_DIR="$BASE_DIR/wrappers"

read -rp "Nome do produto (slug, ex: audit-api): " SLUG
read -rp "Domínio (api|docs|ml|supply|compliance|billing|sla): " DOMAIN
read -rp "Policy pack (RID/preset, ex: rid:policy/fin/v1): " POLICY_PACK
read -rp "Runtime (none|WASM|TEE): " RUNTIME
read -rp "Exposição (SDK|CLI|HTTP): " EXPOSURE
read -rp "Brand/Tenant (texto curto): " BRAND

DEST="$WRAP_DIR/$SLUG"
if [ -e "$DEST" ]; then
  echo "Destino já existe: $DEST" >&2
  exit 1
fi

mkdir -p "$DEST"
cp -a "$TPL_DIR/." "$DEST/"

# Patch README title
sed -i "1s|^# .*|# ${SLUG//-/ } (template v3) |" "$DEST/README.md"

# Policy minimal aligns to pack
jq --arg name "${SLUG}_policy_v1" '.name=$name' "$DEST/policies/product.policy.json" > "$DEST/policies/tmp.json" && mv "$DEST/policies/tmp.json" "$DEST/policies/product.policy.json"

# Examples/run.json: embed runtime + metadata
jq --arg rt "$RUNTIME" '.options.require_certified_runtime = ($rt!="none")' "$DEST/examples/run.json" > "$DEST/examples/run.tmp" && mv "$DEST/examples/run.tmp" "$DEST/examples/run.json"

# Add generator.json (traceability)
cat > "$DEST/generator.json" <<JSON
{
  "slug": "$SLUG",
  "domain": "$DOMAIN",
  "policy_pack": "$POLICY_PACK",
  "runtime": "$RUNTIME",
  "exposure": "$EXPOSURE",
  "brand": "$BRAND",
  "generated_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
JSON

echo "OK: $DEST"


# Auto-provision if TDLN_HOST is set
if [[ -n "${TDLN_HOST:-}" ]]; then
  echo "Auto-provisioning..."
  ( cd "$DEST" && ./scripts/provision.sh "$SLUG" )
else
  echo "TIP: export TDLN_HOST to auto-provision the app binding."
fi

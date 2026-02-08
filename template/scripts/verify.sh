#!/usr/bin/env bash
set -euo pipefail
CARD="${1:?pass card.json path}"
# RREF check: links.card_url (preferred) or links.url (legacy) pattern
URL=$(jq -r '(.links.card_url // .links.url // "")' "$CARD")
if [[ ! "$URL" =~ ^https://cert.tdln.foundry/r/b3:[0-9a-f]{16,}$ ]]; then
  echo "RREF FAIL: links.card_url/url inválido"; exit 1; fi
# SIRP presence (optional, warn-only)
S=$(jq -r '[.refs[]?|select(.kind|startswith("sirp."))|.kind]|join(",")' "$CARD")
echo "RREF PASS. SIRP refs: ${S:-none}"

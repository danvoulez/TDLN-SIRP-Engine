#!/usr/bin/env bash
set -euo pipefail
BUNDLE="$1"
if command -v b3sum >/dev/null 2>&1; then
  B3=$(b3sum "$BUNDLE" | awk '{print $1}')
else
  B3=$(python3 - <<'PY' "$BUNDLE"
import sys, hashlib
p=sys.argv[1]
h=hashlib.blake2s()
with open(p,'rb') as f:
  while True:
    c=f.read(1<<20)
    if not c: break
    h.update(c)
print(h.hexdigest())
PY
  )
fi
echo "b3:${B3}  $BUNDLE"
echo "cargo run -p engine-cli -- registry-put --name engine-foundation-suite --version 25.0.1 --cid b3:${B3} --regdir ./registry"

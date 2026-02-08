#!/usr/bin/env python3
import sys, json
data = json.load(open(sys.argv[1]))
kinds = [r.get("kind","") for r in (data.get("refs") or []) if isinstance(r, dict)]
need = {"sirp.capsule.v1", "sirp.receipt.delivery.v1"}
have = set(kinds)
missing = [k for k in need if k not in have]
if missing:
    raise SystemExit(f"SIRP WARN/FAIL: faltando {missing}; encontrados={sorted(have)}")
print("SIRP OK:", sorted(have))

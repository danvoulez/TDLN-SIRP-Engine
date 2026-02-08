# RREF v1.1 Conformance Pack

This pack reflects the **short handle** (`links.card_url = https://cert.tdln.foundry/r/<run_cid>`) and the **CID-first** rule:

- **Public refs** must include at least one portable resolver (`https://registry.../objects/<cid>` or `tdln://objects/<cid>`).
- **Private refs** may start with presigned, but **must** also include a portable resolver somewhere.

Files:
- `rref-v1.1.schema.json` — JSON Schema of the minimal receipt.card.v1 (links/proof/refs).
- `rref11_verify.py` — offline verifier (structure + href policy).
- `rref-v1.1-test-vectors.json` — PASS/WARN/FAIL vectors.

## Usage

Validate a card:
```bash
python3 rref11_verify.py /path/to/card.json
```

Run vectors:
```bash
python3 - <<'PY'
import json, subprocess, tempfile, os
base = "/mnt/data/rref_v1_1_pack"
vectors = json.load(open(base + "/rref-v1.1-test-vectors.json", "r"))
ok=0; total=0
for v in vectors:
  total += 1
  tmp = tempfile.NamedTemporaryFile(delete=False, suffix=".json")
  tmp.write(json.dumps(v["card"]).encode("utf-8")); tmp.close()
  out = subprocess.check_output(["python3", base + "/rref11_verify.py", tmp.name])
  res = json.loads(out.decode("utf-8"))
  os.unlink(tmp.name)
  exp = v["expected"]
  print(("✔" if res["result"]==exp else "✘"), v["name"], "expected", exp, "got", res["result"], res)
print(f"Passed {ok}/{total}")
PY
```


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.


### SIRP Signatures & Resolver
- Engine initializes an Ed25519 signer from `ENGINE_SIGNING_KEY_ED25519` (base64 seed) or `ENGINE_SIGNING_KEY_ED25519_FILE`.
- If neither provided, a new seed is generated at `var/keys/ed25519.seed`.
- Route `/r/:run`:
  - `Accept: application/json` → returns the Card JSON.
  - otherwise → `303` to `/<realm>/<did>#<run_cid>`.

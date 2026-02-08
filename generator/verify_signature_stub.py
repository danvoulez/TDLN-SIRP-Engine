#!/usr/bin/env python3
import sys, json, base64, hashlib, re
# Stub: só recomputa canônico (chaves ordenadas) e confere que o header existe.
# A verificação Ed25519 real roda no engine; aqui só conferimos formato.
def sort_keys(o):
    if isinstance(o, dict):
        return {k: sort_keys(o[k]) for k in sorted(o.keys())}
    if isinstance(o, list):
        return [sort_keys(x) for x in o]
    return o
req = json.load(open(sys.argv[1]))
headers = open(sys.argv[2]).read()
pub_b64 = open(sys.argv[3]).read().strip()
canon = json.dumps(sort_keys(req), separators=(',',':'))
sig = None
for line in headers.splitlines():
    if line.lower().startswith("x-app-signature:"):
        sig = line.split(":",1)[1].strip()
assert sig and sig.startswith("ed25519:"), "Signature header ausente ou inválido"
print("SIGNATURE HEADER OK (formato). Pubkey.b64 len=", len(pub_b64))
print("CANON SHA256:", hashlib.sha256(canon.encode()).hexdigest())

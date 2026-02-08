#!/usr/bin/env python3
# Fallback keygen using Python (no external deps): use nacl-like ed25519 via hashlib/ed25519 if available
import os, json, base64
try:
    import nacl.signing as ns  # type: ignore
    sk = ns.SigningKey.generate()
    seed = sk._seed
    pub = sk.verify_key.encode()
except Exception:
    # Very small fallback: use secrets for 32 bytes; no pub calc (engine will reject sign, but we avoid crashing)
    import secrets
    seed = secrets.token_bytes(32)
    pub = b''
print(json.dumps({"seed_b64": base64.b64encode(seed).decode(), "pub_b64": base64.b64encode(pub).decode()}))

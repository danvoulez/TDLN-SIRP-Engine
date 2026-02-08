# TDLN CI Pack — RREF + SIRP + Signature

Gates de CI para garantir:
1) `links.url` curto (`https://cert.tdln.foundry/r/b3:...`)
2) `refs` com SIRP (`sirp.intent|delivery|result|execution` quando aplicável)
3) Assinatura de app no `/v1/run` (`X-App-DID`, `X-App-Signature` sobre o corpo JSON✯Atomic)

## Uso local
```bash
# card.json -> verificação RREF+SIRP
python3 ci/check_rref.py card.json
python3 ci/check_sirp.py card.json

# request.json + headers.txt -> verificação assinatura (stub)
python3 ci/verify_signature_stub.py request.json headers.txt pubkey.b64
```


## CI Gates (RREF/SIRP/Signature)
- `make ci` roda validações locais.
- GitHub Actions: `.github/workflows/tdln-ci.yml`.

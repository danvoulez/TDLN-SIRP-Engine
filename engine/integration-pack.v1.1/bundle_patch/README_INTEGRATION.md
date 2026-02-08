# Integração — Kernel Branco + Certified Runtime + RREF

Este pacote completa a integração no **SEU** bundle:

1) **/run exige `unit_ref`** — ver `01-unitref-required.patch.diff` (aplicado).
2) **Certified Runtime obrigatório** — defina `RUNTIME_CERT_PATH` apontando para um `runtime.cert.v1` válido (schema incluso).
3) **RREF v1.1** — cards com `links.card_url` e refs CID-first.

## Passos
- Coloque um certificado válido em JSON e exporte:
  ```bash
  export RUNTIME_CERT_PATH=/etc/tdln/runtime.cert.v1.json
  python3 runtime_cert_verify.py $RUNTIME_CERT_PATH  # PASS
  ```
- Suba o engine e teste:
  - Sem `unit_ref` ⇒ `ASK + PoI(missing:["unit_ref"])`
  - Sem runtime cert ⇒ `NACK + PoI(missing:["runtime_cert"])`
  - Com ambos ⇒ ACK/NACK conforme a UNIT

## SDKs
- Rust workspace atualizado com **ed25519**: `tdln-rust-workspace.v1.1-ed25519.zip`
- Certified Runtime WASM v1.1: `certified-runtime-wasm-v1.1-kit.zip`

Released at: 2026-02-07T16:11:17Z


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

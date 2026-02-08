# Certified Runtime — WASM v1 (Kit)

**Inclui:**
- `specs/certified-runtime-wasm-v1.md` — contrato técnico (determinismo, sandbox, metering, CID-first, EER).
- `schemas/eer-wasm-v1.schema.json` — schema do EER (hash do runtime, config, digests, wasmtime).
- `rust/` — workspace com crates:
  - `tdln-certified-runtime` (traits/tipos)
  - `tdln-runtime-wasm` (impl com wasmtime, determinístico + fuel)
  - `tdln-runner` (CLI)

**Uso (local):**
```bash
cd rust
cargo build

# executar unit wasm com input.json e produzir card.json
cargo run -p tdln-runner -- run --wasm ./unit.wasm --input ./input.json --out ./card.json
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

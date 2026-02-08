
# guest-wrapper-deterministic

Wrapper mínimo que expõe a ABI determinística do Engine:
- `alloc(len)`, `dealloc(ptr,len)`, `run(ptr,len)->(ptr,len)`

A lógica fica em `logic::execute(Json) -> Json` (pura/determinística).

## Build
```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
# artifact:
# target/wasm32-unknown-unknown/release/guest_wrapper_deterministic.wasm
```

## Teste rápido com /run-wasm
```bash
WASM=target/wasm32-unknown-unknown/release/guest_wrapper_deterministic.wasm
WASM_B64=$(base64 -w0 "$WASM")
curl -sS http://localhost:8080/run-wasm -H 'content-type: application/json'   -d "{"wasm_b64":"$WASM_B64","input":{"hello":"world"}}"
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

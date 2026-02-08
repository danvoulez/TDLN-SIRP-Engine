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

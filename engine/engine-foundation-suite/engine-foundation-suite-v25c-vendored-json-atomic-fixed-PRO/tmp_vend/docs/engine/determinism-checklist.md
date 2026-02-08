
# Determinism Checklist (Engine + Guest)

## Guest (WASM)
- [x] Compilar para `wasm32-unknown-unknown`
- [x] Nenhuma dependência de WASI / wasm-bindgen / imports (tempo, rand, fs, net)
- [x] Função pura (sem I/O), toda entrada vem no JSON; toda saída no JSON
- [x] Evitar floats NaN/Inf estranhos; host já canonicaliza NaN, mas simplifique a lógica
- [x] Sem timestamps/UUIDs aleatórios; se precisar de IDs, derive de conteúdo (CID)

## Host (Engine)
- [x] `consume_fuel(true)` e `store.add_fuel(limit)`
- [x] `memory_limit_bytes` e checagens de OOB (alloc e retorno)
- [x] Rejeitar imports em `validate_module` (unless `allow_imports=true`)
- [x] Canonização JSON in/out (JSON✯Atomic)
- [x] Sumarizar meta (fuel_limit, mem_limit) no receipt


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

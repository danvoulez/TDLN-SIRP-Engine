# Testing (Engine)

- Unit tests: expression evaluation, wiring aggregation, PoI generation.
- Golden tests: stable receipts for fixed inputs.
- Chaos: presign TTL/verb/byte-range scenarios should audit and fail-closed.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

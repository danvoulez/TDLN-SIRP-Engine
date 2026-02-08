# Determinism Guide

- Avoid non-deterministic sources (time.now, random). Inject timestamps only in metadata outside decision path.
- Sort keys consistently; use canonical JSON (JSON✯Atomic) before hashing.
- Validate that equal inputs produce equal `hash_chain` in CI.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

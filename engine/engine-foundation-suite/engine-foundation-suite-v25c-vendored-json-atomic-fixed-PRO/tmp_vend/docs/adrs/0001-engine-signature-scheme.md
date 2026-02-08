
# ADR-0001: Engine signature scheme (ed25519 over blake3(json))

- Status: Accepted
- Date: 2026-02-05
- Decision Owners: Engine Team

## Context
We need a stable, deterministic signature scheme for receipts and grants with minimal dependencies and strong crypto agility.

## Decision
Use **ed25519 over blake3(message)** for:
- `access.grant.v1.seal.sig`
- PASETO v4.public variant (payload = JSON grant)

## Consequences
- Positive: uniform verifier; fast; small binaries.
- Trade-off: diverges from pure PASETO spec (we note in docs and SDK mirrors behavior).
- Future: can introduce `alg: ed25519` (pure) and run both in parallel; deprecate blake3 variant later.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

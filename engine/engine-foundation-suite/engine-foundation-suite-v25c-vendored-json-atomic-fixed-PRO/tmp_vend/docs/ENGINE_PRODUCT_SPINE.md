
# Engine — Product Spine

**Mission**: Deterministic, auditable, product‑agnostic execution with receipts (ACK/ASK/NACK + PoI).  
**Tenets**: JSON✯Atomic • Content‑addressed (CID) • Fail‑closed • Audit‑by‑default • Zero custody.

## Core Surfaces
- **/v1/run**: execute unit → returns DID/CID URL, status RUNNING, final receipt + offline bundle.
- **/registry/presign**: constrained pre‑signed URLs (ACK/ASK/NACK + PoI) + sealed Grant (+ PASETO).
- **/s3/proxy**: grant‑verified streaming with constraints & revocation; ASK (PoI) on violations.

## Invariants
- Determinism: same input ⇒ identical receipt/hash‑chain.
- No HITL: indecision ⇒ **ASK** with PoI (never manual escalation).
- Observability: every call emits `audit.report.v1` with digests and timing.

## SLOs (G1)
- P95 `/v1/run` <= 200ms (no WASM), <= 1s (WASM).
- Proxy availability >= 99.9%.

## Roadmap Glimpse
- v21: SDK TS + CLI verify (musl).
- v22: Grant Revocation List (signed), SDK auto-refresh helpers.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.


# ADR-NNNN: Title

- Status: Proposed | Accepted | Deprecated
- Date: YYYY-MM-DD
- Decision Owners: ...

## Context
...

## Decision
...

## Consequences
- Positive: ...
- Negative: ...
- Neutral/Trade-offs: ...


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

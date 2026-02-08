# Runbook (Engine)

- On-call checks: /ready, /metrics rates, audit emit success rate.
- Incident: high NACK → examine rules & PoI; high presign denies → RBAC drift.
- Kill-switch: PRESIGN_DISABLE=true (deny all); audit span still created.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

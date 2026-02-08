# Engine HTTP API (Generic Surface)

## Health
- `GET /health` → 200
- `GET /ready` → 200
- `GET /version` → semver string
- `GET /metrics` → Prometheus text

## Execute
- `POST /run` → `{receipt, bundle_url?}`
- `POST /submit-data`, `POST /submit-code` → same receipt contract

## Registry
- `POST /registry/put` → `{ok:true}`

## Presign
- `POST /acquire_presigned_url` → `{url, grant, expires_at}`


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

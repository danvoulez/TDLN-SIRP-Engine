# Pre-signed Access Spec

## Grant (`access.grant.v1`)
```
{ kind, grant_id, sub, tenants[], resource{ store, bucket, prefix, object?, verbs[], constraints? }, exp, iat, nonce, seal{alg,kid,sig} }
```
- Seal: `ed25519-blake3` (default).

## Intention: `acquire_presigned_url`
- Request includes resource and verb; engine evaluates RBAC; emits grant + URL; audits outcome.
- TTL defaults 5 minutes; verbs: GET/PUT.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

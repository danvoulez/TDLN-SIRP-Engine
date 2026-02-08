# Audit Report Spec (`audit.report.v1`)

## Required Fields
- `audit_id` (ULID), `ts`, `who`, `tenant_scope[]`
- `policy`: version, rules_matched[], decision, redactions[]
- `plan`: planner_version, engine{ name, version }, sql_emitted?, operators[], sources[]
- `limits`: row_cap, time_cap_ms, object_cap
- `runtime`: elapsed_ms, rows_returned, attempts, breaker, fallback_to
- `proofs`: inputs_merkle_root, result_digest, dv25_seal{pubkey,sig,signed_fields[]}
- `observability`: traceId, client_ip_hash, user_agent_hash

## Storage Layout (S3-compatible, dual emit)
```
tenants/{tenant}/audit/YYYY/MM/DD/{audit_id}.json
tenants/{tenant}/audit/monthly/YYYY/MM/audit-YYYY-MM.ndjson.zst
tenants/{tenant}/audit/manifests/audit-YYYY-MM.json
```


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

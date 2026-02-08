# Runbook

## Readiness
- GET /ready → 200 OK
- GET /version → string
- GET /metrics → Prometheus text

## Deploy
- Edit .env (PORT, BRAND_NAME, BRAND_COLOR, RATE_QPS, UNITS_DIR)
- docker build && docker run -p 8080:8080

## Common Incidents
- High ASK rate: check data completeness; PoI hints.
- Presign errors: validate RBAC & TTL; audit.report.v1 for grant failures.
- Determinism issues: ensure units & inputs are canonical; verify hash_chain.

## SLOs
- p95 < 200ms; 0 crashes; audit spans 100%.

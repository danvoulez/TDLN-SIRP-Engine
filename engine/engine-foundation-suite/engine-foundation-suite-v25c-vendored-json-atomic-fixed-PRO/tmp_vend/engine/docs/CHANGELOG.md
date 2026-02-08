# Engine Changelog

## [0.1.0] - 2026-02-05
- Initial engine docs: spine, invariants, specs (receipt/audit/presign/registry), API, security, determinism, ADRs, runbook, contributing, testing.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

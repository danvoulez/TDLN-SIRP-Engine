
# Changelog

## v21 — 2026-02-05
- SDK **@logline/verify** (TS): verify PASETO v4.public + grant JSON seal; PoI helpers; audit digest helpers.
- CLI **receipt-verify** (Rust, musl‑ready): offline validation of bundle.zip + issuer signature check.
- Docs kit: Product Spine, ADR template + ADR‑0001, Changelog bootstrapped.



## v22 — 2026-02-05
- SDK: `verifyPasetoWith()` valida **exp** e **kid**; helper `ipHash()`.
- CLI: `--strict` compara `bundle_hash` com recomputado e emite **JSON report** (`--out_report`).



## v23 — 2026-02-05
- SDK: `validateIpConstraint()` e `rangeHint()` (hints de uso de constraints).
- CLI: `--prove` gera prova assinada (`verify.proof.v1`) com ed25519.
- Engine: **GRL assinada** (`revoked_grants/manifest.json`) + **/health** auditável.



## v24 — 2026-02-05
- SDK: `fetchHealth()`, `parseHealth()`, `startHealthMonitor()` — monitor leve do Engine.
- Engine: `/.well-known/logline/grl.json` (merge local+remoto com cache TTL via `GRL_REMOTE_URL`, `GRL_TTL_MS`).
- Wrapper-gen: nota de uso do Health Widget (opt-in).



## v25 — 2026-02-05
- SDK: `fetchGrl()`, `isRevoked()` e cache in-memory com `makeGrlCache()`.
- Engine: GRL com **ETag** + doc de **If-None-Match**; métrica `metrics/grl_merge_applied.count`.
- Wrapper-gen: opção `--with-verify-cli` para incluir o binário **receipt-verify** no container de admin.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

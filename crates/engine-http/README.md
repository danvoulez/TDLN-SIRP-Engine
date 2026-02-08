
# engine-http

## Endpoints
- `GET /health` → "ok"
- `GET /ready` → "ok"
- `GET /version` → pkg version
- `POST /run` → stub
- `POST /run-wasm` → deterministic WASM execution (JSON ABI)

### /run-wasm

Request:
```json
{
  "wasm_b64": "<base64 do .wasm>",
  "input": { "msg": "hi" }
}
```

Response:
```json
{
  "output": { "msg": "hi", "echo": true },
  "meta": { "fuel_limit": 50000000, "memory_limit_bytes": 33554432, "deterministic": true }
}
```

Mount:
```rust
use engine_http::{engine_router_with_wasm, EngineHttpConfig};
let app = engine_router_with_wasm(EngineHttpConfig{ enable_metrics: true });
```


---
### /registry/presign  (R2 / MinIO / FS)

Request:
```json
{
  "backend": "s3",        // "s3" (R2/MinIO) ou "fs"
  "bucket": "vv-ledger-prod",
  "key": "tenants/acme/audit/2026/02/test.json",
  "verb": "GET",
  "ttl_secs": 300,
  "who": "dan@voulezvous"
}
```

Response:
```json
{
  "url": "...signed...",
  "meta": { "backend":"s3","bucket":"vv-ledger-prod","key":"...","verb":"GET","ttl_secs":300 }
}
```

**Auditoria**: cada emissão gera `audit.report.v1` (intent=`presign`).
**Ambiente R2/MinIO** (exemplo):
```bash
export AWS_ACCESS_KEY_ID=...
export AWS_SECRET_ACCESS_KEY=...
export AWS_REGION=auto       # R2 usa "auto"; MinIO pode usar "us-east-1"
export AWS_ENDPOINT_URL=https://<account>.r2.cloudflarestorage.com   # ou https://s3.lab512.example
```


#### Decisões (TDLN-like) em `/registry/presign`
- **ACK**: URL emitida
- **ASK**: sem URL; retorna `poi` com `violations` e `hints`
- **NACK**: rejeitado (p.ex., `verb_not_allowed`)

**Regras padrão (demo policy):**
- `verb ∈ {GET, PUT}`
- `ttl_secs ≤ 600`
- `byte_range_max ≤ 100MB` (se informado)
- `key` sem `..`
- `backend ∈ {s3, fs}`


### Grant selado + Proxy verificador

**Fluxo**: cliente pede `/registry/presign` → se **ACK**, recebe `url` e `grant` (no campo `meta.grant`).  
Em seguida, baixa pelo nosso proxy:
```
GET /s3/proxy?bucket=vv-ledger-prod&key=tenants/acme/audit/2026/02/test.json
X-LogLine-Grant: <base64(JSON do access.grant.v1)>
```
O proxy:
1) verifica assinatura (ed25519-blake3) e expiração  
2) confere `object` autorizado == `key` solicitado  
3) lê direto do S3 (R2/MinIO) e **streaming** para o cliente  
4) emite `audit.report.v1` (intent=`proxy_get`)

**Obs.** Chaves dev em `secrets/dev/ed25519.*.b64`. Em produção, injete `KMS/HSM`.


#### Revogação e Enforcement no Proxy
- **Revogação**: coloque um arquivo vazio em `./revoked_grants/<grant_id>.json` para invalidar imediatamente.
- **Constraints** aplicados pelo proxy:
  - `ip_hash`: compara com `X-Client-IP` (ou `X-Forwarded-For`) → se não bater, **ASK** com PoI.
  - `byte_range_max`: valida cabeçalho `Range` (limite por resposta) → se exceder, **ASK** com PoI.
  - `object` binding: `key` solicitado deve ser **igual** ao do grant (senão `401`).
- Toda violação gera `audit.report.v1` (`intent="proxy_get"`) com a **decisão**.

**Exemplo de revogação:**
```bash
touch revoked_grants/01HXYZ...ULID.json
# próxima chamada ao proxy usando este grant retorna 401
```


### PASETO v4.public para Grants
- `/registry/presign` (ACK) agora devolve também `paseto` (token `v4.public.`) ao lado do `grant` JSON.
- O proxy aceita **ou** `X-LogLine-Paseto: <token>` **ou** `X-LogLine-Grant: <base64(JSON)>`.

### Chunking automático no proxy
- Se o cliente pedir `Range` acima de `byte_range_max`, o proxy **capará** o intervalo e responderá **206 Partial** com `Content-Range` ajustado.
- Tudo auditado com `range_start/range_end`.

### Kill-switches operacionais
```bash
# desliga emissão de presigned (serviço em manutenção/abuso)
export PRESIGN_DISABLE=1
# desliga proxy (bloqueio de emergência)
export PROXY_DISABLE=1
```


### Health auditable
- `GET /health` → `{ ok, presign_enabled, proxy_enabled, ts }` e emite `audit.report.v1` (`intent="health"`).

### Grant Revocation List (manifest assinada)
- Arquivo: `revoked_grants/manifest.json`
```json
{
  "kind": "revocation.manifest.v1",
  "updated_at": "2026-02-05T12:00:00Z",
  "grants": ["01H...ULID", "01J...ULID"],
  "sig": "<ed25519(base64) sobre blake3(json sem sig)>"
}
```
- O proxy verifica **primeiro** a manifest assinada; se ausente ou inválida, cai no modo `per-file` (`revoked_grants/<grant_id>.json`).


### GRL caching semantics
- `GET /.well-known/logline/grl.json` sets **ETag** = `b3:<body>`.
- Clients podem enviar **If-None-Match** para 304 (template sugere; ajuste no handler real).
- Métrica de merge aplicada em `metrics/grl_merge_applied.count`.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.


### SIRP Signatures & Resolver
- Engine initializes an Ed25519 signer from `ENGINE_SIGNING_KEY_ED25519` (base64 seed) or `ENGINE_SIGNING_KEY_ED25519_FILE`.
- If neither provided, a new seed is generated at `var/keys/ed25519.seed`.
- Route `/r/:run`:
  - `Accept: application/json` → returns the Card JSON.
  - otherwise → `303` to `/<realm>/<did>#<run_cid>`.

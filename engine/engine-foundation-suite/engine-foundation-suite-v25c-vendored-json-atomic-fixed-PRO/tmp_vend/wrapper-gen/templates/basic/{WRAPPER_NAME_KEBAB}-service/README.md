# {{WRAPPER_NAME}} (preset: {{PRESET}})

**Premium wrapper** gerado por `wrapper-gen` — API + UI estática, CORS, logs, OpenAPI, Dockerfile e `.env.example`.

## Quickstart
```bash
# gerar este wrapper (exemplo)
cargo run -p wrapper-gen -- new --dir ./wrappers/{{WRAPPER_NAME_KEBAB}} --name "{{WRAPPER_NAME}}" --color "{{THEME_COLOR}}" --flavors code,data --preset premium

# rodar
cd wrappers/{{WRAPPER_NAME_KEBAB}}
cp .env.example .env
cargo run -p {{WRAPPER_NAME_KEBAB}}-service
open http://localhost:8080
```

## Endpoints
- `GET /health`, `GET /ready`, `GET /version`
- `POST /run`
- `POST /submit-data`, `POST /submit-code`
- `POST /registry/put`
- `POST /acquire_presigned_url`

## Config (env)
- `PORT`, `BRAND_NAME`, `BRAND_COLOR`, `CORS_ORIGINS`, `RATE_QPS`
- `UNITS_DIR` (carrega units JSON✯Atomic em HOT-RELOAD)
- (opcional, com `--features s3`) `S3_ENDPOINT`, `S3_REGION`, `S3_ACCESS_KEY`, `S3_SECRET_KEY`, `S3_BUCKET_DEFAULT`

## OpenAPI
Edite `openapi.yaml` e publique no teu portal de devs.

## Deploy
- `Dockerfile` pronto
- `Makefile` com alvos `dev/build/run/docker`
- CI Github Actions básico (build+test)


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

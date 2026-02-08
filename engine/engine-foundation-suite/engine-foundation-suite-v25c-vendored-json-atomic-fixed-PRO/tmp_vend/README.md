
# Engine Foundation Suite — Generic, Modular, Product‑Agnostic 🧩

**Goal:** a strong base from which any product wrapper can emerge, without brand nouns.

## Modules
- **engine-core**: JSON✯Atomic unit engine with pluggable providers (canon, CID, time, signer, aggregation, sink).
- **engine-extras**: Ed25519 signer, K‑of‑N aggregator, expression registry, filesystem sink, optional **S3‑compatible** sink (feature‑gated).
- **engine-auth**: generic AccessGrant (JSON + Rust types) with a seal (alg/kid/sig).
- **engine-registry**: neutral JSON registry (file‑based impl for dev).
- **engine-audit**: tiny audit report struct + FS sink (NDJSON/JSON; extend as needed).
- **engine-cli**: runs a sample chip ⇒ emits **receipt.card.v1** + **audit.report.v1**; puts entries in the registry.

## Quickstart
```bash
cargo build

# Run sample chip (Allow path)
cargo run -p engine-cli -- run --input ./input_allow.json --outdir ./out --k 2

# Doubt path (graceful missing info)
cargo run -p engine-cli -- run --input ./input_doubt.json --outdir ./out_doubt --k 2

# Add registry entry (agnostic JSON)
cargo run -p engine-cli -- registry-put --name example --version 1.0.0 --cid b3:deadbeef --regdir ./registry
```

## Notes
- **No product storage layout** is imposed. S3‑compatible sink is feature‑gated and provider‑neutral.
- **Clean layering:** core engine ↔ auth primitives ↔ registry ↔ audit. Wrappers can be added later without touching the base.


---

## HTTP neutro (engine-http)
```bash
cargo run -p engine-http
# POST /run, /registry/put, /acquire_presigned_url
curl -s localhost:8088/run -X POST -H 'content-type: application/json' -d '{"input":{"actor":{"role":"admin","quota":5},"resource":{"restricted":false}}}'
```

## Gerador de Wrapper (engine-wrapper-gen)
```bash
# Gera um wrapper pronto a partir do template neutro
cargo run -p engine-wrapper-gen -- --name acme-trust --outdir ./wrappers

# Estrutura gerada:
# wrappers/acme-trust/
#   Cargo.toml (workspace)
#   acme-trust-svc/ (Axum service com healthz; cole os handlers que quiser)
#
# Para subir:
cd wrappers/acme-trust && cargo run -p acme-trust-svc
```

### Ideia de evolução
- Presets do gerador: `--preset api-only | full-http | cli-only`
- Flags: `--with-registry`, `--with-audit`, `--with-presign`
- Substituição de placeholders de branding em README e rotas, mantendo o núcleo 100% neutro.


## HTTP neutro — endpoints
- `POST /run` — executa unidade JSON✯Atomic com input genérico
- `POST /submit-code` — aceita código/URL e executa (placeholder); devolve receipt/card
- `POST /submit-data` — aceita JSON e executa; devolve receipt/card
- `POST /registry/put` — cadastra entry no registry
- `POST /acquire_presigned_url` — presigner abstrato; `s3` real via feature/env


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

# Docs Index

- `product-spine.md` — visão curto e prático (TL;DR primeiro).
- `CHANGELOG.md` — mudanças notáveis.
- `architecture.md` — visão técnica do wrapper sobre o engine.
- `runbook.md` — operar, incidentes comuns, SLO.
- `glossary.md` — termos rápidos.
- `adrs/` — decisões arquiteturais.

> Estes docs são neutros em domínio e amigáveis a LLMs (títulos claros, seções curtas).

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

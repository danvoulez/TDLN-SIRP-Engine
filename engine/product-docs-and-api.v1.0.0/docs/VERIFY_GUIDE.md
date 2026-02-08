# Guia de Verificação (Offline)

- Card público: `receipt.card.v1` com `links.card_url = https://cert.tdln.foundry/r/<run_cid>`
- Refs: `cid:b3:<...>` e `hrefs[]` contendo **canônico** (`/v1/objects/<cid>`) ou `tdln://` (portátil).
- Para privados, inclua `presign` **e** um resolver portátil.
- Conformance: `conformance/rref-v1.1/*`


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

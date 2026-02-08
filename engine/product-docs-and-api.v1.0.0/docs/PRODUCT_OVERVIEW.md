# LogLine Trust — v1.0.0 (Produto Oficial)

**Uma forma, dois reinos.** O LogLine Trust entrega verificação *NHE-by-default* com recibos públicos auditáveis (JSON✯Atomic), `links.card_url` (QR/curto) e refs **CID-first** com resolvers portáteis.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

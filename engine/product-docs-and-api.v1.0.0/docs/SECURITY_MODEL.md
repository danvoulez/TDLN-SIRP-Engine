# Modelo de Segurança (v1)

- **NHE**: sem escalada humana; indecisões retornam PoI máquina-legível.
- **CID-first**: identidade do conteúdo é o hash (BLAKE3).
- **Seal**: `ed25519-blake3` sobre bytes canônicos (implementação no runtime).
- **Bundles privados**: sempre via presign/ACL + resolver portátil para auditabilidade.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

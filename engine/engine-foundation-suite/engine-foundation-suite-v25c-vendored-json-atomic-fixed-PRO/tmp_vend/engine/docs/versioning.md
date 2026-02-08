# Versioning & Compatibility

- **Engine**: SemVer on APIs and receipt fields (`engine/receipt@1` stable within major).
- **Docs**: maintained with engine version; changes recorded in CHANGELOG.
- **Compat Matrix**: wrappers define minimal engine version; verify at startup.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

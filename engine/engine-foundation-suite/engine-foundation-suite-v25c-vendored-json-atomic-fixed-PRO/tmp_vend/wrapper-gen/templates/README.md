
# Wrapper Templates — Extras

## Health Widget (optional)
- Import `@logline/verify` and call `startHealthMonitor(baseUrl, onUpdate, 15_000)`.
- Show a small banner in the admin UI when `issues.length > 0`.
- This is opt-in; remove at generation time with `--no-health-widget`.


## Bundle `receipt-verify` into admin container (opt-in)
- Pass `--with-verify-cli` at generation time.
- The Dockerfile adds a build stage for `tools/receipt-verify` and copies the static MUSL binary to `/usr/local/bin/receipt-verify`.


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

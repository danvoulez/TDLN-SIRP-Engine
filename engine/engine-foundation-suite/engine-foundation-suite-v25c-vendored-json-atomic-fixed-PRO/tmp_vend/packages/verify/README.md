
# @logline/verify

TypeScript SDK to verify LogLine grants and receipts offline.

## Features
- Verify **PASETO v4.public** tokens (Ed25519, ed25519(blake3(msg)) compatibility layer to match Engine v20).
- Verify **access.grant.v1** JSON seal (`ed25519-blake3`).
- Parse and summarize **PoI** (Proof of Indecision) responses.
- Helper to validate **audit.report.v1** digests.

> NOTE: this SDK mirrors the v20 Engine behavior (Ed25519 over `blake3(json)` for both PASETO and JSON seals).


## Extra validation
```ts
import { verifyPasetoWith } from '@logline/verify';
const grant = await verifyPasetoWith(token, { publicKeyB64: PK, requiredKid: 'prod-2026-key-1' });
```
- Checks **exp** and **kid** in addition to signature.
- Helper `ipHash(ip)` to pre-compute constraint.


### Constraints helpers
```ts
import { validateIpConstraint, rangeHint } from '@logline/verify';
const ipOk = validateIpConstraint(grant, '203.0.113.10');
const rangeOk = rangeHint(1048576, 'bytes=0-2097152');
```


### Health monitor
```ts
import { startHealthMonitor } from '@logline/verify';
const stop = startHealthMonitor('https://api.example.com', h => console.log('health', h), 15000);
```    


### GRL (Grant Revocation List)
```ts
import { fetchGrl, isRevoked, makeGrlCache } from '@logline/verify';
const grl = await fetchGrl('https://api.example.com');
console.log(isRevoked('01H...ULID', grl));
const cache = makeGrlCache();
await cache.fetchAndSet('https://api.example.com');
```    


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

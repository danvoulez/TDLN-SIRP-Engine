# Receipt Specification (Generic)

## Object
- `schema`: `engine/receipt@1`
- `did`, `card_id`, `issued_at`
- `decision`: `ACK|ASK|NACK`
- `refs.inputs[]`: `{name, kind, cid, bytes?}`
- `runtime`: `{engine_version, profile?, duration_ms, input_cid, output_cid}`
- `proof`: `{hash_chain[], signature?}`
- `poi` (when ASK): `{missing_fields[], missing_evidence[], hint}`
- `signatures.issuer` (optional): DV25-like `{alg,kid,sig}`

## Determinism
- Canonicalization → CID
- Hash-chain includes: input CID, per-step CID(s), output CID.
- No wall-clock/entropy in decision path.

## Bundle (offline)
- `receipt.json`, `verification-instructions.md`, `signatures.sig` (optional).
- Verification recomputes CIDs and compares to receipt.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

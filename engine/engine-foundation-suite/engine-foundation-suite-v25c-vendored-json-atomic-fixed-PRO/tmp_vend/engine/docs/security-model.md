# Security Model

- **TDLN RBAC** for grants & presign; default DENY.
- **Short-lived presign** with scope (bucket/prefix/object, verb, TTL) and optional IP-hash / byte-range.
- **No credentials** on clients; only URLs.
- **PQ-agility** (optional): dual-sig (Ed25519 + Dilithium3) via pluggable signers.
- **Fail-closed** on missing attestations (EER, SLSA manifests).
- **Audit fences**: every grant and presign is audited (grant_id, exp, resource, proofs).


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

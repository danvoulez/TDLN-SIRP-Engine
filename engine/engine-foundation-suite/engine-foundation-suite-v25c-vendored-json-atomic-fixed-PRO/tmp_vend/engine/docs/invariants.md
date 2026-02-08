# Engine Invariants

- **Canonical Input**: requests are normalized to JSON✯Atomic before execution.
- **Deterministic Receipts**: `hash_chain`, `input_cid`, `output_cid` are stable for same inputs & unit.
- **NHE / NEI**: No Human Escalation, No Exposure Invariants. All outputs are receipts or PoI.
- **Audit-on-By-Default**: every run creates `audit.report.v1` span(s).
- **Pre-signed Access**: data IO via short-lived URLs only (RBAC-governed). No provider credentials to clients.
- **Fail-Closed**: missing attestations or policy → ASK/NACK with PoI; never silent allow.
- **Caps**: row/time/object caps enforced and recorded.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

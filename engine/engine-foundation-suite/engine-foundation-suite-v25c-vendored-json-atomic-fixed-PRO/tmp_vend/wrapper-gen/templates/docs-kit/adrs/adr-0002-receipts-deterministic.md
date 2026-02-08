# ADR-0002: Receipts are deterministic & offline-verifiable

## Status
Accepted

## Context
Regulators and partners need proofs without network reliance.

## Decision
Receipts include hash_chain + input/output CIDs; bundles contain all needed artifacts; verifier recomputes CIDs.

## Consequences
- Pros: Strong trust, easy audits.
- Cons: Stricter constraints on nondeterminism.

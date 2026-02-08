# ADR-0001: Engine stays generic; wrapper carries UX

## Status
Accepted

## Context
We want one engine powering many products. Coupling UX or domain logic into engine reduces reuse.

## Decision
Keep engine 100% generic (JSON✯Atomic, receipts, audit, presign). Put UX, color, texts in wrappers.

## Consequences
- Pros: Reuse, security boundaries, faster SKU generation.
- Cons: Requires wrapper-gen discipline and docs.

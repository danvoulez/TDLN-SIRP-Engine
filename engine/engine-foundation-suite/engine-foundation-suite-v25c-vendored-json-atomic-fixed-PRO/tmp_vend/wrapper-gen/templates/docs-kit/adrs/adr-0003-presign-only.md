# ADR-0003: Data access via pre-signed URLs only

## Status
Accepted

## Context
Clients cannot hold provider credentials; minimize blast radius.

## Decision
All data access is via short-lived pre-signed links issued after RBAC (TDLN). No raw keys on clients.

## Consequences
- Pros: Principle of least privilege; auditable; easy revocation.
- Cons: Extra hop; link management.

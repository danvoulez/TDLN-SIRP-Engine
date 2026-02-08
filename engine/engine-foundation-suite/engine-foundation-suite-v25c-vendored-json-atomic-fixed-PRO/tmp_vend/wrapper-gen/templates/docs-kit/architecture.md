# Architecture Overview

## Context
Wrapper on top of a generic engine. HTTP + static UI. Optional S3-compatible presigner.

## Components
- Wrapper Service (this repo): routing, static UI, config, CORS, logging, metrics.
- Engine (dependency): receipts, audit, registry, presign integration, units loader (hot-reload).
- Storage (optional): S3-compatible endpoints for presigned access.

## Flows
1) /run → engine executes → receipt.json + bundle.zip → audit span.
2) /acquire_presigned_url → RBAC → short URL → audit span.

## Non-Functional
- Determinism, NHE/NEI, ASK/NACK with PoI, p95 latency, caps.

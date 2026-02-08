---
title: Engine Spine
version: 0.1.0
generated: 2026-02-05
---

# Engine Spine

## TL;DR
- Execute **JSON✯Atomic units** and emit **deterministic receipts**.
- First-class **Audit** and **Pre-signed Access**; zero HITL/custody.
- Stable **/run** contract; offline-verifiable **bundle.zip**.

## 1. Core Problem → Outcome
- Problem: Prove decisions/work **without exposing raw data**.
- Outcome: Machine-verifiable **ACK/ASK/NACK** with **Proof-of-Indecision (PoI)** and reproducibility.

## 2. Core Interfaces
- HTTP: `/run`, `/registry/put`, `/acquire_presigned_url`, health/ready/version/metrics.
- CLI helpers: verifier, conform tools (outside core).

## 3. Non-Goals
- No business logic or domain policies inside the engine.
- No secrets for data providers exposed to clients.

## 4. Success Criteria
- Determinism (same input → same CIDs).
- ASK always actionable (PoI useful).
- 0 crashes; all failures map to **ASK** or **NACK** with reasons.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.

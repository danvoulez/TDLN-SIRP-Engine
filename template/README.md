# Wrapper Template v3 — TDLN + SIRP (PROD)

Este template gera **cascas** padronizadas que conversam com o **TDLN Engine** (link único `/r/<run_cid>`) e saem com **RREF + SIRP** documentados.
Sem placeholders. Pronto p/ produção.

## O que vem pronto
- `openapi.yaml` — contrato estável com `/v1/run`.
- `schemas/` — RREF v1.2.2 e SIRP v1 (capsule, delivery, execution).
- `scripts/` — `run.sh` (curl) e `verify.sh` (checagens rápidas).
- `examples/` — `run.json` válido.
- `policies/` — `product.policy.json` (mínima, válida).
- `.env.example` — exige `TDLN_HOST` explícito.
- `generator/` — **5 perguntas** (JSON) para instanciar o wrapper.
- `src/sdk/` — cliente HTTP fino com `verifyCard()` e `verifySirp()` (stubs chamáveis).

## Uso
```bash
cp .env.example .env
# edite .env e defina:
# TDLN_HOST=https://seu-engine (ex.: https://cert.engine.prod)

./scripts/run.sh ./examples/run.json | jq .
./scripts/verify.sh ./examples/card.sample.json
```

## Conformidade obrigatória (CI)
- **RREF PASS**: `links.url` == `https://cert.tdln.foundry/r/b3:*`.
- **SIRP Kinds**: `sirp.intent|delivery|result|execution` (quando presentes).
- **CID-first**: `refs[].cid` em `b3:*`; 2+ resolvers quando aplicável.
- **Sem localhost**: `.env.example` não contém valores default.

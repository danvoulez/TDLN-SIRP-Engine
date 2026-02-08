# LogLine / TDLN / SIRP — Release v1.3.2 + wrap3.2 + template3 (2026-02-07)

## TL;DR
- ✅ `./test_all.sh` passa com `FILELIST.json` como fonte única de integridade.
- ✅ Handle curto no formato: `https://cert.tdln.foundry/r/<run_cid>`.
- ✅ RREF v1.1 conformance vetores alinhados ao regex hex.
- ✅ Template alinhado aos schemas SIRP v1 (capsule + delivery + execution).

## Novidades
- `test_all.sh` como runner unificado (integridade + conformance + checks locais).
- Vetores RREF corrigidos para `links.card_url` hex válido.
- Samples do template/generator alinhados aos kinds versionados (`sirp.*.v1`).

## Quebras / Compat
- Handles são **CID-first**; URLs são resolvers/mirrors.
- Presigned é resolver temporário (não-identidade).

## Como reproduzir
```bash
./test_all.sh | tee reports/test_all.$(date -u +%Y%m%dT%H%M%SZ).log
bash make_release_artifacts.sh
```

## Conformidade
- RREF v1.1: `conformance/rref-pack/rref11_verify.py` sobre `rref-v1.1-test-vectors.json`
- SIRP v1: presença de `sirp.capsule.v1`, `sirp.receipt.delivery.v1`, `sirp.receipt.execution.v1`

## Itens abertos
- SBOM consolidado (Rust + JS) ainda stub/minimal.
- Documentar resolver `tdln://` para airgap/mirror local.


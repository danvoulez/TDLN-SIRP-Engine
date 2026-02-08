# Merge (root): TDLN Blueprint v1.1.0 → repo

Arquivos/dirs do Blueprint foram integrados **no root** do repositório, com proteção de conflito.

## Pastas integradas
- `schemas/`
- `acts/`
- `docs/`
- `products/`
- `test_vectors/`
- `crates/tdln-permit/` (novo crate)

## Conflitos
Quando um arquivo já existia e era diferente, gravamos a cópia como `*.bp-v1.1.0<ext>` para evitar sobrescrever.

## Workspace
Cargo.toml patchado: nenhum (verifique o root Cargo.toml)

## Próximos passos
1) `cargo check -p tdln-permit`
2) Integrar `tdln-permit` no executor e exigir `verify_permit()` antes de executar ação
3) `wrapper-gen from-manifest blueprint/products/api-receipt-gateway.json` (quando disponível)

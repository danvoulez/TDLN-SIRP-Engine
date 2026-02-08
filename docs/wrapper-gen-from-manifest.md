# wrapper-gen — from-manifest (spec v1.1)

## Input
- `product.json` validated contra `schemas/product.schema.json`

## Steps
1. Parse + validate manifest.
2. Acts → copiar templates `/acts/{attest|evaluate|transact}/` (schema.rs, mapper.rs)
3. Policies → resolver `/policy-packs/<pack>@<version>/` → copiar `units/`
4. Proof level → ligar sirp/bundle em `hooks.rs`
5. Enrichments → ativar módulos/rotas
6. Billing → hook pós-run
7. Consent → persistir seletores k-of-n (Control Plane)
8. Bindings → aplicar no mapper.rs
9. Routes → registrar

## Output
```
wrappers/<name>/
  src/domain/schema.rs
  src/domain/mapper.rs
  src/enrichment.rs
  src/hooks.rs
  units/*.json
  Cargo.toml
  README.md
```

## CLI
```
wrapper-gen from-manifest ./product.json --out ./wrappers/<name>
```

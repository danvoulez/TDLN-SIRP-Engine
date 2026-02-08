# Repo Hardening Pack (v1)

O que contém
- Cargo workspace (root)
- rust-toolchain, .editorconfig, .gitignore additions
- Pre-commit config
- CI (Rust + Python)
- Scaffold para consolidar CLIs Python em `tools/python`
- Script de migração para mover crates do engine (vendidos) para `crates/`

Como usar (no seu repo)
1) Copie o conteúdo deste pacote para a raiz do repo (sem sobrescrever o que já existe; revise `.gitignore.additions`).
2) Opcional: rode `./scripts/02-crates-relayout.sh` para mover crates do motor para `./crates`.
3) Instale pre-commit: `pipx install pre-commit && pre-commit install`.
4) Valide: `just check` (ou `cargo check --workspace`).

Observação
- O script de migração é "best-effort" — revise os caminhos reais do seu repo antes de executar.

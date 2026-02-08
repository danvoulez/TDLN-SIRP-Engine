
#!/usr/bin/env bash
set -euo pipefail

# Use from repo root. Moves vendored engine crates to ./crates.
# NOTE: safe-by-default: no automatic rewriting of Cargo.toml `path = "..."`
# dependencies is performed. After moving, this script reports any remaining
# references to the old locations so you can patch them deterministically.

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

echo "[1/4] Criando pastas..."
mkdir -p crates tools/python scripts

echo "[2/4] Movendo crates do engine-foundation-suite (tmp_vend) para crates/ ..."
# Ajuste estes caminhos conforme os diretórios existentes no seu repo
for c in engine/engine-foundation-suite/*/tmp_vend/*; do
  name=$(basename "$c")
  if [ -f "$c/Cargo.toml" ]; then
    git mv "$c" "crates/$name" || mv "$c" "crates/$name"
  fi
done

echo "[3/4] Movendo crates do certified-runtime-wasm para crates/ ..."
for c in engine/certified-runtime-wasm.v1.1/rust/crates/*; do
  name=$(basename "$c")
  if [ -f "$c/Cargo.toml" ]; then
    git mv "$c" "crates/$name" || mv "$c" "crates/$name"
  fi
done

echo "[4/4] Verificando referencias antigas em path deps..."
echo "Se aparecerem resultados abaixo, ajuste os `path = ...` manualmente e rode `cargo check --workspace`."
find . -name Cargo.toml -type f -print0 | xargs -0 grep -nH -E 'path\\s*=\\s*\"(\\.{1,2}/)+engine/(engine-foundation-suite|certified-runtime-wasm\\.v1\\.1)' || true

echo "Feito. Agora rode:"
echo "  cargo check --workspace"

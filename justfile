default: check

check:
    cargo fmt --all -- --check
    cargo check --workspace
    cargo clippy --workspace

py.lint:
    uv run ruff check tools/python

py.fmt:
    uv run ruff format tools/python

tree4:
    find . -maxdepth 4 -print | sed 's|^./||'

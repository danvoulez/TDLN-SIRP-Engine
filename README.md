# LogLine / TDLN / SIRP — MEGA Bundle (expanded)

This repository is an expanded (no nested zips) distribution bundle for inspection and internal use.

**License:** All rights reserved. Internal distribution only. See `LICENSE`.

## What’s inside

- `engine/` — engine artifacts, SDKs, docs, conformance packs
- `generator/` — wrapper generator (legacy v3.x)
- `tools/wrapper-gen/` — wrapper-gen v4.1.1 (vendored) + template workspace
- `wrappers/` — generated wrappers (e.g. `wrappers/auditoria-api/`)
- `template/` — wrapper template v3 + schemas + scripts
- `kits/` — certified runtime kit(s)
- `conformance/` — conformance vectors/packs
- `_archives/` — original zip archives (for reference)

## Quick checks

```bash
./test_all.sh --all
```

## Generate a new wrapper (v4.1.1)

```bash
tools/wrapper-gen/wrapper-gen create \
  --name auditoria-api \
  --domain api \
  --policy-pack compliance,v2 \
  --runtime wasm \
  --expose http,grpc \
  --output wrappers/auditoria-api
```

## Run the wrapper locally (optional)

```bash
cd wrappers/auditoria-api/service
cp .env.example .env
# set TDLN_API_KEY=...
cargo run
```


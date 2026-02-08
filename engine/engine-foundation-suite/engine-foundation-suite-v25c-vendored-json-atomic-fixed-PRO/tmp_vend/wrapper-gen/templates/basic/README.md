
# {{WRAPPER_NAME}} — wrapper gerado

```bash
# no diretório criado
cargo run -p {{WRAPPER_NAME_KEBAB}}-service
# abrir health
curl http://localhost:8080/health
# executar
curl -X POST http://localhost:8080/run -H "content-type: application/json" -d '{"input":{"actor":{"role":"admin","quota":5},"resource":{"restricted":false}}}'
```


## Presigner S3-compat (opcional)
Compile o serviço com `--features s3` e configure:
```
export S3_ENDPOINT="https://s3.example.com"   # opcional
export S3_REGION="auto"
export S3_ACCESS_KEY="..."
export S3_SECRET_KEY="..."
export S3_BUCKET_DEFAULT="demo-bucket"
cargo run -p {{WRAPPER_NAME_KEBAB}}-service --features s3
```
Se não setar, o wrapper usa o presigner *stub* que retorna URLs sintéticas.


## Carregar units via arquivos (JSON/YAML) com hot-reload
Crie sua pasta e aponte via env:
```
export UNITS_DIR=./units
# exemplo de unit (JSON) está em ../../units/allow_admin_quota.json
cargo run -p {{WRAPPER_NAME_KEBAB}}-service
```
Quando mudar/adição de arquivo ocorrer no `UNITS_DIR`, o serviço recarrega a lista automaticamente.


## Manifesto (idea.manifest.v1)
Você pode gerar o wrapper a partir de um manifesto JSON:
```bash
cat > product.json <<'JSON'
{ "kind":"idea.manifest.v1", "name":"My Product", "brand_color":"#3B82F6", "flavors":["code","data"], "limits":{"row_cap":100000,"time_cap_ms":15000}, "rate_qps":20 }
JSON

cargo run -p wrapper-gen -- new --dir ./wrappers/my-product --manifest ./product.json --preset premium
```
O `--manifest` preenche automaticamente `--name`, `--color` e `--flavors`.


### Unified Link Behavior (v1.2.2)
- `links.url` is the single handle: `https://cert.tdln.foundry/r/<run_cid>`.
- `GET /r/<run_cid>`:
  - `Accept: application/json` → returns Card JSON.
  - browser (default) → 303 to `/<realm>/<did>#<run_cid>`.
- `card_url` is deprecated; kept only for backward compatibility in deserialization.


### SIRP Signatures & Resolver
- Engine initializes an Ed25519 signer from `ENGINE_SIGNING_KEY_ED25519` (base64 seed) or `ENGINE_SIGNING_KEY_ED25519_FILE`.
- If neither provided, a new seed is generated at `var/keys/ed25519.seed`.
- Route `/r/:run`:
  - `Accept: application/json` → returns the Card JSON.
  - otherwise → `303` to `/<realm>/<did>#<run_cid>`.

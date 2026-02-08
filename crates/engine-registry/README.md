
# engine-registry

Backend-agnostic RegistryProvider com impls:
- `fs` (local, para dev/testes)
- `s3` (S3-compatível → Cloudflare R2, MinIO/LAB512, etc.)

## R2/MinIO (feature `s3`)
Env vars usuais:
- `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`
- `AWS_REGION` (ex.: `auto` para R2, ou `us-east-1`)
- `AWS_ENDPOINT_URL` (ex.: `https://<account_id>.r2.cloudflarestorage.com` ou `https://s3.lab512.example`)

> Presign real pode exigir presigners específicos; aqui usamos placeholder (`s3://bucket/key`) para manter o core agnóstico.

## FS (feature `fs`)
Diretório raiz configurável; gera `cid_b3` ao gravar bytes.

## Exemplo (FS)
```rust
use engine_registry::fs_registry::FsRegistry;
use engine_registry::RegistryProvider;

# #[tokio::main] async fn main() -> anyhow::Result<()> {
let reg = FsRegistry::new("./.dev-registry");
reg.put_bytes("artifacts", "runs/01/receipt.json", br#"{"ok":true}"#).await?;
let got = reg.get_bytes("artifacts", "runs/01/receipt.json").await?;
println!("{}", String::from_utf8(got)?);
# Ok(()) }
```


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


# Smoke: /run-wasm (deterministic)

1) Compile o guest-wrapper:
```bash
cd engine-exec-wasm/examples/guest-wrapper-deterministic
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

2) Suba o servidor com `engine_router_with_wasm`:
```rust
use engine_http::{engine_router_with_wasm, EngineHttpConfig};
use axum::{Router};
use std::net::SocketAddr;
#[tokio::main] async fn main() {
  let app = engine_router_with_wasm(EngineHttpConfig{ enable_metrics: true });
  axum::Server::bind(&"0.0.0.0:8080".parse::<SocketAddr>().unwrap())
    .serve(app.into_make_service()).await.unwrap();
}
```

3) Chame /run-wasm:
```bash
WASM=engine-exec-wasm/examples/guest-wrapper-deterministic/target/wasm32-unknown-unknown/release/guest_wrapper_deterministic.wasm
WASM_B64=$(base64 -w0 "$WASM")
curl -sS localhost:8080/run-wasm -H 'content-type: application/json' -d "{"wasm_b64":"$WASM_B64","input":{"msg":"hi"}}"
```

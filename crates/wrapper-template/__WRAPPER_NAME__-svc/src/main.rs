
use axum::{Router, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  let app = Router::new()
     .route("/healthz", get(|| async { "ok" }));
  let addr = SocketAddr::from(([0,0,0,0], 8090));
  println!("__WRAPPER_NAME__ service on {}", addr);
  axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

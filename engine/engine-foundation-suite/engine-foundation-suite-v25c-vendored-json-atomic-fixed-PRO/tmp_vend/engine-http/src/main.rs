
#[tokio::main]
async fn main() {
    use engine_http::server::build_router_with_flavors;
    use engine_http::presign::StubPresigner;

    #[cfg(feature = "s3")]
    {
        use engine_http::presign_s3::S3Presigner;
        match S3Presigner::from_env().await {
            Ok(p) => {
                let app = build_router_with_flavors("./out", "./registry", 2, p).await;
                let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
                println!("🚀 engine-http (S3 presigner) on :8080");
                axum::serve(listener, app).await.unwrap();
                return;
            }
            Err(e) => eprintln!("S3 presigner init error: {e}. Falling back to stub..."),
        }
    }

    let app = build_router_with_flavors("./out", "./registry", 2, StubPresigner).await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 engine-http (stub presigner) on :8080");
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = engine_http::engine_router_with_wasm(engine_http::EngineHttpConfig {
        enable_metrics: true,
    });
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("engine-http listening on 0.0.0.0:8080");
    axum::serve(listener, app).await?;
    Ok(())
}

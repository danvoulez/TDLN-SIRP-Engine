use axum::{body::Body, http::Request, middleware::Next, response::Response};

pub async fn api_key(req: Request<Body>, next: Next) -> Response {
    // TODO: verify `x-api-key`; pass-through for template
    next.run(req).await
}

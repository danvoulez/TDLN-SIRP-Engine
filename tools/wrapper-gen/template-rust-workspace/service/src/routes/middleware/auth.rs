use axum::{http::Request, middleware::Next, response::Response};

pub async fn api_key<B>(req: Request<B>, next: Next<B>) -> Response {
    // TODO: verify `x-api-key`; pass-through for template
    next.run(req).await
}

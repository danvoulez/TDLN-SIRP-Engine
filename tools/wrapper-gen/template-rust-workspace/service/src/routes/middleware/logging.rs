pub fn layer() -> tower_http::trace::TraceLayer<tower_http::classify::ServerErrorsAsFailures> {
    tower_http::trace::TraceLayer::new_for_http()
}

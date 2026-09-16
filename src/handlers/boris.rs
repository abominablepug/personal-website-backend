use axum::response::Json;
use axum::{Router, routing::get};

async fn boris_check() -> Json<String> {
    Json("Boris is alive!".to_string())
}

pub fn boris_routes() -> Router {
    Router::new().route("/", get(boris_check))
}

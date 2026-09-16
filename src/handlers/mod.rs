pub mod boris;
use axum::Router;
use std::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(boris::boris_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030")
        .await
        .expect("Failed to bind to address");

    println!("Server running on http://localhost:3030");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

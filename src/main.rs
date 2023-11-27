use axum::{routing::get, Router};
use mitblob::config;

#[tokio::main]
async fn main() {
    let config = config::get();
    let port = config.port;
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

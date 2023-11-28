use axum::{routing::get, Router};
use mitblob::config;

#[tokio::main]
async fn main() {
    let config = config::get();
    let port = config.port;
    // build our application with a single route
    let app = Router::new().route(
        "/",
        get(|| async {
            config
                .git_repo
                .latest_commit(config.git_branch.as_str())
                .await
                .unwrap_or_else(|e| e.to_string())
        }),
    );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

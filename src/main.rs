use axum::{routing::get, Router};
use mitblob::{
    blob::{get_sha, put_sha},
    config,
};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = config::get();
    let port = config.port;
    info!("port = {port}");
    // build our application with a single route
    let app = Router::new()
        .route(
            "/",
            get(|| async {
                config
                    .git_repo
                    .latest_commit(config.git_branch.as_str())
                    .await
                    .unwrap_or_else(|e| e.to_string())
            }),
        )
        .route(
            "/:sha",
            // TODO https://docs.rs/axum/latest/axum/#using-the-state-extractor
            get(get_sha).put(put_sha),
        );

    let bind_address = "0.0.0.0";
    info!("Binding on {bind_address}:{port}");
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("{bind_address}:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

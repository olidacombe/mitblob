use axum::{extract::Path, Json};
use serde_json::Value;
use tracing::info;

pub async fn get_sha(Path(sha): Path<String>) -> String {
    sha
}

pub async fn put_sha(Path(sha): Path<String>, Json(payload): Json<Value>) {
    info!("PUT /{sha} : {payload}")
}

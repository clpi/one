pub mod admin;
use std::sync::Arc;

use axum::{
    response::IntoResponse,
    routing::{get, put, delete, Router}, extract::{State, Path, DefaultBodyLimit}, body::Bytes, http::StatusCode,
};
use tower_http::{compression::CompressionLayer, limit::RequestBodyLimitLayer};

use crate::SharedState;

pub(crate) fn routes() -> Router<SharedState> {
    Router::new()
        // .route("/:key", get(kv_get.layer(CompressionLayer::new()))
        //     .post_service(kv_set.layer((
        //         DefaultBodyLimit::disable(),
        //         RequestBodyLimitLayer::new(1024 * 5000) // 5 MB
        //     ))
        //     .with_state(Arc::clone(&shared_state))))
}

async fn kv_get(
    Path(key): Path<String>,
    State(state): State<SharedState>
) -> Result<Bytes, StatusCode> {
    let db = &state.read().unwrap().db;
    if let Some(val) = db.get(&key) {
        return Ok(val.clone())
    }
    Err(StatusCode::NOT_FOUND)
}

async fn kv_set(
    Path(key): Path<String>,
    State(state): State<SharedState>,
    bytes: Bytes,
) {
    state.write().unwrap().db.insert(key, bytes);
}

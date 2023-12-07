use axum::{
    Router,
    routing::{get, post, delete}, extract::{State, Path},
};
use tower_http::validate_request::ValidateRequestHeaderLayer;

use crate::SharedState;


pub(crate) fn router() -> Router<SharedState> {
    Router::new()
        .route("/keys", delete(clear_keys))
        .route("/keys/:key", delete(clear_key))
        .layer(ValidateRequestHeaderLayer::bearer("secret"))
}

async fn clear_keys(State(state): State<SharedState>) {
    state.write().unwrap().db.clear()
}

async fn clear_key(Path(key): Path<String>, State(state): State<SharedState>) {
    state.write().unwrap().db.remove(&key);
}

pub mod routes;
use axum::{
    body::Bytes,
    error_handling::HandleErrorLayer,
    extract::{DefaultBodyLimit, Path, State},
    handler::Handler,
    response::{Html, IntoResponse, Json},
    routing::{get, post, delete},
    http::StatusCode,
    Router,
};
use tower::{ServiceBuilder, BoxError};
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
    time::Duration,
};
use tower_http::{
    compression::CompressionLayer,
    limit::RequestBodyLimitLayer,
    trace::TraceLayer,
    validate_request::ValidateRequestHeaderLayer,
};
use serde::{Deserialize, Serialize};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[derive(Default)]
struct AppState {
    db: HashMap<String, Bytes>
}
type SharedState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "example_key_value_store=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let shared_state = SharedState::default();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/dashboard", get(dashboard))
        .route("/login", get(get_login))
        .route("/signup", get(get_signup))
        .route("/users", get(|| async { "Hello, World!" }))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/groups", post(create_group))
        .route("/users/:id", get(|params: axum::extract::Path<(u64, String)>| async move {
            let (id, username) = params.0;
            format!("Hello, {}! Your id is {}", username, id)
        }))
        .nest("/admin", routes::admin::router())
        .layer(ServiceBuilder::new()
            .layer(HandleErrorLayer::new(handle_error))
            .load_shed()
            .concurrency_limit(1024)
            .timeout(Duration::from_secs(10))
            .layer(TraceLayer::new_for_http()))
        .with_state(Arc::clone(&shared_state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// Page for logging in
async fn get_login() -> &'static str {
    "Loggin"
}

// Page for signing up
async fn get_signup() -> &'static str {
    "Signup"
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// user dashboard get handler
async fn dashboard() -> &'static str {
    "Hi there world"
}

async fn create_group(
    Json(payload): Json<CreateGroup>,
) -> (StatusCode, Json<Group>) {
    let group = Group {
        id: 1,
        user_id: payload.user_id,
        name: payload.name,
    };
    (StatusCode::CREATED, Json(group))
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}
// the input to our 'create_group' handler
#[derive(Deserialize)]
struct CreateGroup {
    user_id: u64,
    name: String
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

// the output to our 'create_group' handler
#[derive(Serialize)]
struct Group {
    id: u64,
    user_id: u64,
    name: String,
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("Request timed out"));
    } else if error.is::<tower::load_shed::error::Overloaded>() {
        return (StatusCode::SERVICE_UNAVAILABLE, Cow::from("Service overloaded, try again later"));
    }
    (StatusCode::INTERNAL_SERVER_ERROR, Cow::from(format!("Unhandled error: {error}")))
}

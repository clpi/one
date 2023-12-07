use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

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
        }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
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
    (StatusCode::CREATED, group)
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

#[derive(Serialize)]
struct Group {
    id: u64,
    user_id: u64,
    name: String,
}

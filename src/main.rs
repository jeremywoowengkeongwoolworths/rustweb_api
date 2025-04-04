mod employee;

use axum::{
    Json,
    routing::get,
    Router,
    response::IntoResponse
};

use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    // build our application with a single route
    
    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/json", get(getEmployee));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, root!"
}

async fn hello() -> impl IntoResponse {
    let user = String::from("World").to_uppercase();
     let message = format!("Hello, Axum! {user}");
    return message;
}

async fn getEmployee() -> Json<employee::Employee> {
    let user = String::from("World").to_uppercase();
    let message = format!("Hello, Axum! {user}");

    let employee = employee::Employee {
        name: String::from("John Doe"),
        age: 30,
        position: String::from("Software Engineer"),
    };

    Json(employee)
}

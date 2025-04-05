mod employee;
mod car;

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
        .route("/employee", get(get_employee))
        .route("/car", get(get_car));

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

async fn get_employee() -> Json<Vec<employee::Employee>> {
    let user = String::from("World").to_uppercase();
    let message = format!("Hello, Axum! {user}");
    let n: u16 = 10;
    let mut emplist: Vec<employee::Employee> = Vec::new();

    for i in 0..n {

        // let employee = employee::Employee {
        //     name: String::from("John Doe"),
        //     age: i as u32,
        //     position: String::from("Software Engineer"),
        // };

        let employee = employee::Employee::new(
            String::from("John Doe"),
            i as u32,
            String::from("Software Engineer"),
        );

        emplist.push(employee);    
    }

    Json(emplist)
}

async fn get_car() -> Json<car::Car> {
    let car = car::Car {
        name: String::from("Toyota"),
        age: 5,
    };
    Json(car)
}
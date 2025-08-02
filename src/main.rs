use std::net::SocketAddr;
use dotenv::dotenv;

// Import the app module
mod rest;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = rest::create_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
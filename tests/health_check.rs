use std::net::SocketAddr;

use reqwest::Client;
use tokio::net::TcpListener;
use tokio::task;

use tm::create_router;

/// Starts the Axum server using a TcpListener and returns the bound address
async fn run_server(listener: TcpListener) -> SocketAddr {
    let addr = listener.local_addr().expect("Failed to get local address");

    let app = create_router();

    // Start the Axum server in a background task
    task::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    addr
}

#[tokio::test]
async fn test_health_endpoint() {
    // Configure the listener to use a dynamic port (127.0.0.1:0)
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind to address");

    // Start the server and get the bound address
    let addr = run_server(listener).await;

    // Create a reqwest client
    let client = Client::new();

    // Define the health check endpoint URL
    let url = format!("http://{}/health", addr);

    // Send a GET request
    let response = client
        .get(&url)
        .send()
        .await
        .expect("Failed to send request");

    // Validate the status code
    assert_eq!(response.status(), 200, "Expected 200 OK");
}

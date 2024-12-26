use std::net::SocketAddr;

use reqwest::redirect::Policy;
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
async fn test_create_okr_with_valid_dates() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind to address");
    let addr = run_server(listener).await;

    let url = format!("http://{}/okrs", addr);

    let form_data = [
        ("objective", "Improve team efficiency"),
        ("owner", "Engineering Team"),
        ("start_date", "2024-01-01"),
        ("end_date", "2024-12-31"),
    ];

    let client = Client::builder()
        // Set the maximum number of redirects to follow
        .redirect(Policy::none())
        .build()
        .unwrap();

    let response = client
        .post(&url)
        .form(&form_data)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 303, "Expected 303 See Other");
    let location = response
        .headers()
        .get("location")
        .expect("No Location header found");
    assert_eq!(location, "/okrs/1/add-key-results");
}

// #[tokio::test]
// async fn test_create_okr_with_invalid_dates() {
//     let listener = TcpListener::bind("127.0.0.1:0")
//         .await
//         .expect("Failed to bind to address");
//     let addr = run_server(listener).await;

//     let client = Client::new();
//     let url = format!("http:{}/okrs", addr);

//     let form_data = [
//         ("objective", "Improve team efficiency"),
//         ("owner", "Engineering Team"),
//         ("start_date", "2024-12-31"),
//         ("end_date", "2024-01-01"), // Invalid: end_date is before start_date
//     ];

//     let response = client
//         .post(&url)
//         .form(&form_data)
//         .send()
//         .await
//         .expect("Failed to send request");

//     assert_eq!(
//         response.status(),
//         400,
//         "Expected 400 Bad Request for invalid dates"
//     );

//     // let body: serde_json::Value = response
//     //     .json()
//     //     .await
//     //     .expect("Failed to parse response JSON");
//     // assert_eq!(
//     //     body["error"], "start_date must be earlier than end_date.",
//     //     "Unexpected error message"
//     // );
// }

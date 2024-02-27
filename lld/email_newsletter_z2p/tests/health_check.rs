use email_newsletter_z2p::run;
use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    // we retriever the port assigned to us by the os
    let port = listener.local_addr().unwrap().port();
    let server =
        run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // we return the application address to the
    // caller!
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        // use the returned application address
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

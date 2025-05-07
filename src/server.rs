use once_cell::sync::{Lazy, OnceCell};
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio_postgres::NoTls;

// Store the database client
pub static DB_CLIENT: OnceCell<Arc<Mutex<tokio_postgres::Client>>> = OnceCell::new();

// Store the websocket URL
pub static WEBSOCKET_URL: Lazy<Arc<String>> = Lazy::new(|| {
    Arc::new("127.0.0.1:8080".to_string())
});

// Initialize the database connection
pub async fn connect_db() {
    // If the database client is already initialized
    // pass
    if DB_CLIENT.get().is_some() {
        return;
    }
    // Create a connection to the PostgreSQL database
    let (client, connection) = tokio_postgres::connect("host=localhost user=root dbname=ClusterX", NoTls).await.expect("Failed to connect to the database");

    // Spawn a new task to run the connection in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Set the client in the OnceCell
    DB_CLIENT.set(Arc::new(Mutex::new(client))).expect("Failed to set the database client");
}
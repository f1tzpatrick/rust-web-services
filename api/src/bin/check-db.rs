use std::env;

use mongodb::{bson::doc, options::ClientOptions, Client};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let username = env::var("DB_USER").unwrap();
    let password = env::var("DB_PASSWORD").unwrap();
    let host = env::var("DB_HOST").unwrap_or(String::from("localhost"));
    let port = env::var("DB_PORT").unwrap_or(String::from("27017"));
    let connection_string = format!("mongodb://{}:{}@{}:{}/", username, password, host, port);

    // Parse your connection string into an options struct
    let client_options = ClientOptions::parse(connection_string).await?;

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");

    Ok(())
}

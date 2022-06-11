use std::env;

use anyhow::{Context, Result};

use mongodb::{options::ClientOptions, Client};

#[allow(dead_code)]
pub const ASCENDING_ORDER: i32 = 1;
pub const DESCENDING_ORDER: i32 = -1;

pub async fn get_client() -> Result<Client> {
    let username = env::var("DB_USER").unwrap();
    let password = env::var("DB_PASSWORD").unwrap();
    let host = env::var("DB_HOST").unwrap_or(String::from("localhost"));
    let port = env::var("DB_PORT").unwrap_or(String::from("27017"));

    let connection_string = format!("mongodb://{}:{}@{}:{}/", username, password, host, port);
    let client_options = ClientOptions::parse(connection_string)
        .await
        .with_context(|| format!("Failed to make client_options"))?;
    let client =
        Client::with_options(client_options).with_context(|| format!("Failed to make client"))?;

    Ok(client)
}

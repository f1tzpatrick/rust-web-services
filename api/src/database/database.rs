use futures::stream::{StreamExt, TryStreamExt};
use std::env;

use anyhow::{Context, Result};

use mongodb::{
    bson::doc,
    options::{ClientOptions, FindOptions},
    results::{InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::product::product::{Id, Product};

const DESCENDING_ORDER: i32 = -1;

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

pub async fn get_product_collection() -> Collection<Product> {
    let db = get_client().await.unwrap().database("inventorydb");
    db.collection::<Product>("products")
}

pub async fn get_product(id: Id) -> Option<Product> {
    let collection = get_product_collection().await;
    let filter = doc! { "productId": id };
    let options = FindOptions::builder()
        .sort(doc! { "_id": DESCENDING_ORDER })
        .limit(1)
        .build();

    let mut cursor = collection.find(filter, options).await.unwrap();
    let product = cursor.next().await;
    let product = match product {
        Some(product_result) => match product_result {
            Ok(product) => Some(product),
            Err(_) => None,
        },
        None => None,
    };

    product
}

pub async fn list_products() -> Result<Vec<Product>> {
    let collection = get_product_collection().await;
    let options = FindOptions::builder()
        .sort(doc! { "productId": DESCENDING_ORDER })
        .build();
    let mut cursor = collection.find(None, options).await.unwrap();

    let mut products = vec![];
    while let Some(product) = cursor.try_next().await? {
        products.push(product);
    }
    Ok(products)
}

pub async fn insert_product(product: &Product) -> Result<InsertOneResult> {
    let collection = get_product_collection().await;
    let result = collection
        .insert_one(product, None)
        .await
        .with_context(|| format!("Failed to insert product {:?}", product))?;

    Ok(result)
}

pub async fn replace_product(product: &Product) -> Result<UpdateResult> {
    let collection = get_product_collection().await;
    let filter = doc! { "productId": product.product_id };
    let options = mongodb::options::ReplaceOptions::builder()
        .upsert(Some(true))
        .build();
    let result = collection
        .replace_one(filter, product, options)
        .await
        .with_context(|| format!("Failed to replace product {:?}", product))?;

    Ok(result)
}

pub async fn insert_or_overwrite_product(product: &Product) -> Result<(), ()> {
    match get_product(product.product_id).await {
        Some(product) => match replace_product(&product).await {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        },
        None => match insert_product(&product).await {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        },
    }
}

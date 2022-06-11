use futures::stream::{StreamExt, TryStreamExt};

use anyhow::{Context, Result};

use mongodb::{
    bson::doc,
    options::FindOptions,
    results::{InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::database::database::{get_client, DESCENDING_ORDER};
use crate::product::product::{Id, Product};

pub async fn get_product_collection(client: Option<Client>) -> Collection<Product> {
    let client = match client {
        Some(client) => client,
        None => get_client().await.unwrap(),
    };
    let db = client.database("inventorydb");
    db.collection::<Product>("products")
}

pub async fn get_product(client: Option<Client>, id: Id) -> Option<Product> {
    let collection = get_product_collection(client).await;
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

pub async fn list_products(client: Option<Client>) -> Result<Vec<Product>> {
    let collection = get_product_collection(client).await;
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

pub async fn insert_product(client: Option<Client>, product: &Product) -> Result<InsertOneResult> {
    let collection = get_product_collection(client).await;
    let result = collection
        .insert_one(product, None)
        .await
        .with_context(|| format!("Failed to insert product {:?}", product))?;

    Ok(result)
}

pub async fn replace_product(client: Option<Client>, product: &Product) -> Result<UpdateResult> {
    let collection = get_product_collection(client).await;
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

pub async fn insert_or_overwrite_product(
    client: Option<Client>,
    product: &Product,
) -> Result<(), ()> {
    match get_product(client.clone(), product.product_id).await {
        Some(product) => match replace_product(client.clone(), &product).await {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        },
        None => match insert_product(client.clone(), &product).await {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        },
    }
}

use futures::stream::{StreamExt, TryStreamExt};

use anyhow::Result;

use mongodb::error::Error;

use mongodb::{
    bson::doc,
    options::FindOptions,
    results::{DeleteResult, InsertOneResult, UpdateResult},
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

pub async fn insert_product(
    client: Option<Client>,
    product: &Product,
) -> Result<InsertOneResult, Error> {
    let collection = get_product_collection(client).await;
    let result = collection.insert_one(product, None).await;

    result
}

pub async fn replace_product(
    client: Option<Client>,
    product: &Product,
) -> Result<UpdateResult, Error> {
    let collection = get_product_collection(client).await;
    let filter = doc! { "productId": product.product_id };
    let options = mongodb::options::ReplaceOptions::builder()
        .upsert(Some(true))
        .build();
    let result = collection.replace_one(filter, product, options).await;

    result
}

// TODO: Do a merge update instead of overwrite - not working as intended
pub async fn insert_or_overwrite_product(
    client: Option<Client>,
    product: &Product,
) -> Result<(), Error> {
    match get_product(client.clone(), product.product_id).await {
        Some(product) => match replace_product(client.clone(), &product).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        None => match insert_product(client.clone(), &product).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
    }
}

pub async fn delete_product(client: Option<Client>, product_id: Id) -> Result<DeleteResult, Error> {
    let collection = get_product_collection(client).await;
    let filter = doc! { "productId": product_id };
    let options = mongodb::options::DeleteOptions::builder().build();
    let result = collection.delete_one(filter, options).await;
    result
}

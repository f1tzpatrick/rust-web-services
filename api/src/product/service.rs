use mongodb::Client;
use warp::reply::Json;

use crate::database::product as database;
use crate::product::product::Product;

pub async fn list_products(client: Client) -> Result<Json, warp::Rejection> {
    match database::list_products(Some(client)).await {
        Ok(products) => Ok(warp::reply::json(&products)),
        Err(_) => Err(warp::reject::reject()), // Better error here?
    }
}

pub async fn get_product(id: u32, client: Client) -> Result<Json, warp::Rejection> {
    match database::get_product(Some(client), id).await {
        Some(product) => Ok(warp::reply::json(&product)),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn create_or_update_product(
    client: Client,
    product: Product,
) -> Result<impl warp::Reply, warp::Rejection> {
    match database::insert_or_overwrite_product(Some(client), &product).await {
        Ok(()) => Ok(warp::reply::with_status(
            "SUCCEEDED",
            warp::http::StatusCode::CREATED,
        )),
        Err(()) => Err(warp::reject::reject()),
    }
}

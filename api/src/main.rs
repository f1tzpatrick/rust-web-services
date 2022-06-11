use std::env;

use warp::{http::Method, Filter};

mod database;
mod product;

use crate::database::database::get_client;
use crate::product::service as products;

#[tokio::main]
async fn main() {
    let api_base = "api";

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let client = match get_client().await {
        Ok(client) => client,
        Err(_) => panic!("failed to connect to the database"),
    };
    let client_filter = warp::any().map(move || client.clone());

    let list_products = warp::get()
        .and(warp::path(api_base))
        .and(warp::path("products"))
        .and(warp::path::end())
        .and(client_filter.clone())
        .and_then(products::list_products);

    let get_product = warp::get()
        .and(warp::path(api_base))
        .and(warp::path("products"))
        .and(warp::path::param::<u32>())
        .and(warp::path::end())
        .and(client_filter.clone())
        .and_then(products::get_product);

    let create_product = warp::post()
        .and(warp::path(api_base))
        .and(warp::path("products"))
        .and(warp::path::end())
        .and(client_filter.clone())
        .and(warp::body::json())
        .and_then(products::create_or_update_product);

    let delete_product = warp::delete()
        .and(warp::path(api_base))
        .and(warp::path("products"))
        .and(warp::path::param::<u32>())
        .and(warp::path::end())
        .and(client_filter.clone())
        .and_then(products::delete_product);

    let products_api = list_products
        .or(get_product)
        .or(create_product)
        .or(delete_product);

    let routes = products_api.with(cors);

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 14142,
    };

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

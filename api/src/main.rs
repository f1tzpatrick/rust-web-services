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

    let products_api = list_products.or(get_product).or(create_product);

    let routes = products_api.with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
}

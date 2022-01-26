use std::env;

use warp::{reply::Json, Filter};

use crate::product::data::load_products_from_json_file;


pub fn products_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let products_base = warp::path("products");

    let list = products_base
        .and(warp::get())
        .and(warp::path::end())
        .and_then(list_products);

    let get = products_base
        .and(warp::get())
        .and(warp::path::param())
        .and_then(get_product);

    list.or(get)
}


async fn list_products() -> Result<Json, warp::Rejection> {
	let products = load_products_from_json_file(env::var("PRODUCT_FILE").expect("Define PRODUCT_FILE"));
    let products= warp::reply::json(&products);

    Ok(products)
}

async fn get_product(id: u32) -> Result<Json, warp::Rejection> {
	let products = load_products_from_json_file(env::var("PRODUCT_FILE").expect("Define PRODUCT_FILE"));
    for product in products {
        if product.product_id == id {
            let product= warp::reply::json(&product);
            return Ok(product)
        }
    }
    Err(warp::reject::not_found())
}

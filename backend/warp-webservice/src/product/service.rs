use serde_json::{Value, from_value};
use warp::{reply::Json, Filter};

use super::product::{Product, ProductCache};


pub fn products_filter(cache: ProductCache) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let products_base = warp::path("products");
    let cache_filter = warp::any().map(move || cache.clone());

    let list = products_base
        .and(warp::get())
        .and(warp::path::end())
        .and(cache_filter.clone())
        .and_then(list_products);

    let get = products_base
        .and(warp::get())
        .and(warp::path::param())
        .and(cache_filter.clone())
        .and_then(get_product);

    let create = products_base
        .and(warp::get())
        .and(warp::body::json())
        .and(cache_filter.clone())
        .and_then(create_product);

    list.or(get).or(create)
}


async fn list_products(cache: ProductCache) -> Result<Json, warp::Rejection> {
    let products = cache.as_list();
    let products= warp::reply::json(&products);

    Ok(products)
}

async fn get_product(id: u32, cache: ProductCache) -> Result<Json, warp::Rejection> {
    match cache.get(id) {
        Some(product) => Ok(warp::reply::json(product)),
        None => Err(warp::reject::not_found())
    }
    
}

async fn create_product(data: Value, cache: ProductCache) -> Result<impl warp::Reply, warp::Rejection> {
    match from_value(data) {
        Ok::<Product, _>(_) => {
            // TODO: Get write lock on Cache and add product
            Ok(warp::reply::with_status("Product Added", warp::http::StatusCode::CREATED))
        },
        Err(_) => Err(warp::reject::reject())
    }
}
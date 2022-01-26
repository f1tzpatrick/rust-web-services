mod product;

use std::env;

use warp::Filter;

use product::product::{ProductCache};
use product::service::products_filter;



#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let cache = ProductCache::new_from_file(
        env::var("PRODUCT_FILE")
        .expect("Define PRODUCT_FILE")
    );

    let hello = warp::path("hello")
            .map(|| format!("Hello, World!"));
    
    let apis = hello.or(products_filter(cache));

    warp::serve(apis).run(([127, 0, 0, 1], 5000)).await;
}
 
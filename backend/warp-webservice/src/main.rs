mod product;

use warp::Filter;

use product::service::products_filter;


#[tokio::main]
async fn main() {

    println!("Hello, world!");

    let hello = warp::path("hello")
            .map(|| format!("Hello, World!"));
    
    let apis = hello.or(products_filter());

    warp::serve(apis).run(([127, 0, 0, 1], 5000)).await;
}
 
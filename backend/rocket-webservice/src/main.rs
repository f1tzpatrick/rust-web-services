#[macro_use] extern crate rocket;

mod product;

use rocket::fairing::AdHoc;
use rocket::serde::Deserialize;
use rocket_sync_db_pools::{diesel, database};


type ProductFilePath = String;

#[derive(Debug,Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
    products_file: ProductFilePath,
}

#[database("productdb")]
struct ProductDbConn(diesel::MysqlConnection);

#[launch]
fn rocket() -> _ {
    println!("Hello, world!");
    rocket::build()
        .attach(product::service::stage())
        .attach(AdHoc::config::<AppConfig>())
        .attach(ProductDbConn::fairing())
}

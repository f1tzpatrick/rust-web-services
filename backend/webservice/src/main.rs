#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use rocket::fairing::AdHoc;
use rocket::serde::Deserialize;
use rocket_sync_db_pools::database;

mod product;

type ProductFilePath = String;

#[derive(Debug,Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
    products_file: ProductFilePath,
}

#[database("productdb")]
struct DbConn(diesel::MysqlConnection);

#[launch]
fn rocket() -> _ {
    println!("Hello, world!");
    rocket::build()
        .attach(product::service::stage())
        .attach(AdHoc::config::<AppConfig>())
        .attach(DbConn::fairing())
}

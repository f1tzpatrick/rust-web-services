#[macro_use] extern crate rocket;

mod product;

use rocket::fairing::AdHoc;
use rocket::serde::Deserialize;

type ProductFilePath = String;

#[derive(Debug,Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
    products_file: ProductFilePath,
}

#[launch]
fn rocket() -> _ {
    println!("Hello, world!");
    rocket::build()
        .attach(product::service::stage())
        .attach(AdHoc::config::<AppConfig>())
}

#[macro_use] extern crate rocket;

use rocket::{State, Config};
use rocket::fairing::AdHoc;
use rocket::serde::Deserialize;

mod product;

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
        .attach(product::stage())
        .attach(AdHoc::config::<AppConfig>())
}

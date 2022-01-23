use rocket::State;
use rocket::tokio::sync::Mutex;
use rocket::serde::json::Json;

use crate::AppConfig;
use crate::product::product::{Product, Id};
use crate::product::data;

type ProductList = Mutex<Vec<Product>>;

#[get("/<id>")]
async fn get_product(id: Id, list: &State<ProductList>, app_config: &State<AppConfig>) -> Option<Json<Product>> {
	let mut list = list.lock().await;
    if list.len() == 0 {
		// TODO: Make more DRY
        println!("Reading product list");
        let products_file = &app_config.products_file;
        let mut product_data = data::load_products_from_json_file(products_file);
        list.append(&mut product_data);
    }
	let product = data::get_product(id, &list)?;
	Some(Json(product))
}

#[get("/")]
async fn list_products(list: &State<ProductList>, app_config: &State<AppConfig>) -> Json<Vec<Product>> {
    let mut list = list.lock().await;
    if list.len() == 0 {
		// TODO: Make more DRY
        println!("Reading product list");
        let products_file = &app_config.products_file;
        let mut product_data = data::load_products_from_json_file(products_file);
        list.append(&mut product_data);
    }
    Json(list.to_vec())
}

pub fn stage() -> rocket::fairing::AdHoc {
	rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
		rocket.mount("/products", routes![list_products, get_product])
			.manage(ProductList::new(vec![]))
	})
}

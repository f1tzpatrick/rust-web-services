// NOTE: Refer to Rocket/examples/serialization

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use rocket::State;
use rocket::tokio::sync::Mutex;
use rocket::serde::json::{Json, Value, json, from_str};
use rocket::serde::{Deserialize, Serialize};

use crate::AppConfig;

type Id = usize;
type ProductList = Mutex<Vec<Product>>;
type Products<'r> = &'r State<ProductList>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Product {
	pub ProductId: usize,
	pub Manufacturer: String,
	pub Sku: String,
	pub Upc: String,
	pub PricePer_Unit: String,
	pub QuantityOnHand: usize,
	pub ProductName: String,
}

#[get("/<id>")]
async fn get_product(id: Id, list: Products<'_>) -> Option<Json<Product>> {
	// TODO: Load Products from State and pull product out of list
	let product = Product {
		ProductId: id,
		Manufacturer: String::from("Hello World industries"),
		Sku: String::from("HelloWorld1"),
		Upc: String::from("0000000000001"),
		PricePer_Unit: String::from("0.00"),
		QuantityOnHand: 0,
		ProductName: String::from("Hello World")
	};

	Some(Json(product))
}

#[get("/")]
async fn list_products(list: Products<'_>, app_config: &State<AppConfig>) -> String {
	let products_file = &app_config.products_file;
	let file = File::open(products_file)
		.expect("Could not open file");
	let mut buffered_reader = BufReader::new(file);
	let mut product_data = String::new();
	let _ = match buffered_reader.read_to_string(&mut product_data) {
		Ok(number_of_bytes) => number_of_bytes,
		Err(_err) => 0
	};

	product_data
	// TODO: convert product_data String into Vec<Products>
	// if State Products is empty, write product_data to it
	// Return JSON instead of String
}

pub fn stage() -> rocket::fairing::AdHoc {
	rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
		rocket.mount("/products", routes![list_products, get_product])
			.manage(ProductList::new(vec![]))
	})
}

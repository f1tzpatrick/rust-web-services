use std::collections::HashMap;

use serde::{Serialize,Deserialize};

use super::data::load_products_from_json_file;

pub type Id = u32;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Product {
	pub product_id: Id,
	pub manufacturer: String,
	pub sku: String,
	pub upc: String,
	pub price_per_unit: String,
	pub quantity_on_hand: usize,
	pub product_name: String,
}


#[derive(Clone)]
pub struct ProductCache {
    products: HashMap<Id, Product>,
}

impl ProductCache {
	pub fn new() -> Self {
		ProductCache {
			products: HashMap::new(),
		}
	}

	pub fn new_from_file(path: String) -> Self  {
		let products = load_products_from_json_file(path);
		let mut cache = ProductCache::new();
		for product in products {
			cache.add(&product);
		}
		cache
	}

	pub fn add(&mut self, product: &Product) -> &mut Self {
		self.products.insert(product.product_id.clone(), product.clone());
		self
	}

	pub fn as_list(&self) -> Vec<Product> {
		self.products.values().cloned().collect()
	}

	pub fn get(&self, id: Id) -> Option<&Product> {
		self.products.get(&id)
	}

}
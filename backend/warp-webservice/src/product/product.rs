use serde::{Serialize,Deserialize};


pub type Id = u32;

#[derive(Serialize, Deserialize, Debug)]
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

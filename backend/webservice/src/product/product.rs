use std::str::FromStr;

use rocket::serde::{Deserialize, Serialize, ser::SerializeStruct};
use rocket_sync_db_pools::diesel::Queryable;

use bigdecimal::BigDecimal;

use crate::product::schema;


// #[derive(Debug, Clone)]
// struct Price{
// 	deciaml: BigDecimal
// }

// impl Serialize for Price {
// 	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// 	where
// 			S: rocket::serde::Serializer {
// 				let mut state = serializer.serialize_struct("Price", 1)?;
// 				state.serialize_field("decimal", &self.deciaml);
// 				state.end()
// 	}
// }

// impl Deserialize for Price {
// 	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
// 	where
// 			D: rocket::serde::Deserializer<'de> {
		
// 	}
// }


pub type Id = i32;


#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct Product {
	pub product_id: Id,
	pub manufacturer: String,
	pub sku: String,
	pub upc: String,
	pub price_per_unit: BigDecimal,
	pub quantity_on_hand: i32,
	pub product_name: String,
}

// impl Serialize for Product {
// 	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// 	where
// 		S: rocket::serde::Serializer {
// 			let mut state = serializer.serialize_struct("Product", 7)?;
// 			state.serialize_field("product_id", &self.product_id)?;
// 			state.serialize_field("manufacturer", &self.manufacturer)?;
// 			state.serialize_field("sku", &self.sku)?;
// 			state.serialize_field("upc", &self.upc)?;
// 			// state.serialize_field("price_per_unit", &self.price_per_unit)?;
// 			state.serialize_field("quantity_on_hand", &self.quantity_on_hand)?;
// 			state.serialize_field("product_name", &self.product_name)?;
// 			state.end()
// 	}
// }
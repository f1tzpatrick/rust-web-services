use rocket::State;
use rocket::tokio::sync::Mutex;
use rocket::serde::json::Json;
use rocket::response::Debug;
use rocket_sync_db_pools::diesel;
use self::diesel::prelude::*;
// use self::diesel::{RunQueryDsl, QueryDsl,sql_query};

use crate::{AppConfig, DbConn};
use crate::product::product::{Product, Id};
use crate::product::data;
use crate::product::schema;
// use crate::product::schema::products::dsl::products;

type Result<T, E = Debug<self::diesel::result::Error>> = std::result::Result<T, E>;
type ProductList = Mutex<Vec<Product>>;

#[get("/")]
async fn list_products(conn: DbConn) -> Json<Vec<Product>> {
    let list: Vec<Product> = schema::products::table
        .load::<Product>(&conn).unwrap();
    Json(list)
}

pub fn stage() -> rocket::fairing::AdHoc {
	rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
		rocket.mount("/products", routes![list_products])
			.manage(ProductList::new(vec![]))
	})
}

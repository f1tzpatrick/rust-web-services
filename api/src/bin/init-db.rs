extern crate api;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use serde_json::from_str;

use api::database::database::*;
use api::product::product::Product;

pub fn load_products_from_json_file(path: String) -> Vec<Product> {
    let products = read_products_file(path);
    let products = from_str(&products).expect("Could not deserialize product list");
    products
}

fn read_products_file(path: String) -> String {
    let file = File::open(path).expect("Could not open products file");
    let mut buffered_reader = BufReader::new(file);
    let mut products = String::new();
    buffered_reader
        .read_to_string(&mut products)
        .expect("Products File was empty");
    products
}

#[tokio::main]
async fn main() {
    let products_file = env::var("PRODUCT_FILE").expect("Define PRODUCT_FILE");
    let products = load_products_from_json_file(products_file);
    println!("Found {:?} products in file", products.len());

    for product in products.iter() {
        let _ = insert_or_overwrite_product(product).await;
    }

    let products = list_products().await.unwrap_or(vec![]);
    println!("Read back {:?} products", products.len());
}

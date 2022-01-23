use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use rocket::serde::json::from_str;

use crate::product::product::Product;

pub fn load_products_from_json_file(path: &String) -> Vec<Product> {
    let product_data = read_products_file(path);
    let products: Vec<Product> = from_str(&product_data).unwrap();
    products
}

fn read_products_file(path: &String) -> String {
    let file = File::open(path).expect("Could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut product_data = String::new();
    let _ = match buffered_reader.read_to_string(&mut product_data) {
        Ok(number_of_bytes) => number_of_bytes,
        Err(_err) => 0
    };
    product_data
}

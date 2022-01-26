use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use serde_json::from_str;

use crate::product::product::Product;


pub fn load_products_from_json_file(path: String) -> Vec<Product> {
    let products = read_products_file(path);
    let products = from_str(&products)
        .expect("Could not deserialize product list");
    products
}

fn read_products_file(path: String) -> String {
    let file = File::open(path)
        .expect("Could not open products file");
    let mut buffered_reader = BufReader::new(file);
    let mut products = String::new();
    buffered_reader.read_to_string(&mut products)
        .expect("Products File was empty");
    products
}

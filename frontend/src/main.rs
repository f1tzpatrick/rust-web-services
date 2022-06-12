extern crate api;

use yew::prelude::*;

use crate::api::product::product::{Id, Product};

#[function_component(App)]
fn app() -> Html {
    let product = Product {
        product_id: 1,
        manufacturer: String::from("Hello World Industries"),
        sku: String::from("123456"),
        upc: String::from("78901234567"),
        price_per_unit: String::from("334"),
        quantity_on_hand: 34000,
        product_name: String::from("Yew Logs"),
    };

    html! {
        <h1></h1>
    }
}

fn main() {
    yew::start_app::<App>();
}

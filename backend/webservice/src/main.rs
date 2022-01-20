#[macro_use] extern crate rocket;

mod product;

#[launch]
fn rocket() -> _ {
    println!("Hello, world!");
    rocket::build()
        .attach(product::stage())
}

use rocket::{launch, routes};

pub mod database;
pub mod endpoints;
pub mod entities;
pub mod errors;
pub mod pagination;
pub mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![])
}

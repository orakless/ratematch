use rocket::{launch, routes};

mod database;
mod endpoints;
mod entities;
mod pagination;
mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![])
}

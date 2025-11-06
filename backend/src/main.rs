use rocket::{launch, routes};

mod endpoints;
mod entities;
mod pagination;
mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![])
}

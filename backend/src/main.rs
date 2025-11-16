use rocket::{launch, routes};

use crate::{
    database::{Database, ManageDatabaseConnection},
    endpoints::{get_event_matches, get_events},
};
use crate::endpoints::get_match_desc;

pub mod database;
pub mod endpoints;
pub mod entities;
pub mod errors;
pub mod pagination;
pub mod schema;

pub struct AppState {
    database: Database,
}

#[launch]
fn rocket() -> _ {
    let state = AppState {
        database: Database::new(),
    };

    rocket::build()
        .mount("/", routes![get_events, get_event_matches, get_match_desc])
        .manage(state)
}

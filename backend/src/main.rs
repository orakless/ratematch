use rocket::{launch, routes};

use crate::endpoints::{add_match_rating, get_match_desc};
use crate::{
    database::{Database, ManageDatabaseConnection},
    endpoints::{get_event_matches, get_events, get_match_ratings},
};

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
        .mount(
            "/",
            routes![
                get_events,
                get_event_matches,
                get_match_desc,
                get_match_ratings,
                add_match_rating,
            ],
        )
        .manage(state)
}

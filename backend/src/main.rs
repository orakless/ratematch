use rocket::{launch, routes};
use rocket_cors::CorsOptions;

use crate::endpoints::{add_match_rating, get_event, get_match, get_match_desc, get_ratings};
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

    let cors_layer = CorsOptions::default().to_cors().unwrap();

    rocket::build()
        .mount(
            "/",
            routes![
                get_events,
                get_event,
                get_event_matches,
                get_match,
                get_match_desc,
                get_match_ratings,
                get_ratings,
                add_match_rating,
            ],
        )
        .manage(state)
        .attach(cors_layer)
}

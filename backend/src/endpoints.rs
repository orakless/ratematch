use crate::{
    AppState,
    database::DatabaseOperations,
    entities::{Event, Language, Match, MatchDesc},
    pagination::Page,
};
use rocket::{State, get, response::status::NotFound, serde::json::Json};

#[get("/events?<page>")]
pub fn get_events(
    state: &State<AppState>,
    page: i64,
) -> Result<Json<Page<Event>>, NotFound<String>> {
    match state.database.get_events(page) {
        Ok(events) => Ok(Json(events)),
        Err(_) => Err(NotFound("No ressources".to_string())),
    }
}

#[get("/events/<event_id>/matches")]
pub fn get_event_matches(
    state: &State<AppState>,
    event_id: i32,
) -> Result<Json<Vec<Match>>, NotFound<String>> {
    match state.database.get_card(event_id) {
        Ok(card) => Ok(Json(card)),
        Err(_) => Err(NotFound("No resources".to_string())),
    }
}

#[get("/match/<match_id>/description?<language_code>")]
pub fn get_match_desc(
    state: &State<AppState>,
    match_id: i32,
    language_code: Language,
) -> Result<Json<MatchDesc>, NotFound<String>> {
    match state.database.get_match_description(match_id, language_code) {
        Ok(desc) => Ok(Json(desc)),
        Err(_) => Err(NotFound("No resources".to_string())),
    }
}

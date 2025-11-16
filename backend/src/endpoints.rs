use crate::{
    AppState,
    database::DatabaseOperations,
    entities::{Event, Language, Match, MatchDesc, NewRating, Rating},
    pagination::Page,
};
use rocket::{State, get, http::Status, post, response::status::NotFound, serde::json::Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponseWithData<T> {
    message: String,
    data: T,
}

#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
}

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
) -> Result<Json<ApiResponseWithData<Vec<Match>>>, (Status, Json<ApiResponse>)> {
    match state.database.get_card(event_id) {
        Ok(data) => Ok(Json(ApiResponseWithData {
            message: "Operation done".to_string(),
            data: data,
        })),
        Err(_) => Err((
            Status::UnprocessableEntity,
            Json(ApiResponse {
                message: "Could not get the data.".to_string(),
            }),
        )),
    }
}

#[get("/match/<match_id>/description?<lang>")]
pub fn get_match_desc(
    state: &State<AppState>,
    match_id: i32,
    lang: Language,
) -> Result<Json<ApiResponseWithData<MatchDesc>>, (Status, Json<ApiResponse>)> {
    match state.database.get_match_description(match_id, lang) {
        Ok(data) => Ok(Json(ApiResponseWithData {
            message: "Operation done".to_string(),
            data: data,
        })),
        Err(_) => Err((
            Status::UnprocessableEntity,
            Json(ApiResponse {
                message: "Could not get the data.".to_string(),
            }),
        )),
    }
}

#[get("/match/<match_id>/ratings?<page>&<lang>")]
pub fn get_match_ratings(
    state: &State<AppState>,
    match_id: i32,
    page: i64,
    lang: Language,
) -> Result<Json<ApiResponseWithData<Page<Rating>>>, (Status, Json<ApiResponse>)> {
    match state.database.get_ratings(match_id, page, lang) {
        Ok(data) => Ok(Json(ApiResponseWithData {
            message: "Operation done".to_string(),
            data: data,
        })),
        Err(_) => Err((
            Status::UnprocessableEntity,
            Json(ApiResponse {
                message: "Could not get the data.".to_string(),
            }),
        )),
    }
}

#[post("/match/ratings", format = "application/json", data = "<rating>")]
pub fn add_match_rating(
    state: &State<AppState>,
    rating: Json<NewRating>,
) -> Result<Json<ApiResponse>, (Status, Json<ApiResponse>)> {
    match state.database.new_rating(rating.0) {
        Ok(()) => Ok(Json(ApiResponse {
            message: "Rating added".to_string(),
        })),
        Err(_) => Err((
            Status::UnprocessableEntity,
            Json(ApiResponse {
            message: "Could not add this rating. You may have already submitted a rating with this username.".to_string(),
        }))),
    }
}

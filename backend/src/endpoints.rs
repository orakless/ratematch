use crate::{
    AppState,
    database::DatabaseOperations,
    entities::{Event, Language, Match, MatchDesc, NewRating, Rating},
    pagination::Page,
};
use bigdecimal::BigDecimal;
use chrono::Utc;
use rocket::{State, get, http::Status, post, response::status::NotFound, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ApiResponseWithData<T> {
    message: String,
    data: T,
}

#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
}

#[derive(Deserialize)]
pub struct RequestRating {
    pub match_id: i32,
    pub language_code: Language,
    pub username: String,
    pub score: BigDecimal,
    pub opinion: Option<String>,
}

impl From<RequestRating> for NewRating {
    fn from(value: RequestRating) -> Self {
        NewRating {
            match_id: value.match_id,
            language_code: value.language_code,
            username: value.username,
            score: value.score,
            opinion: value.opinion,
            publication_date: Utc::now().naive_utc(),
        }
    }
}

#[get("/events?<page>")]
pub fn get_events(
    state: &State<AppState>,
    page: i64,
) -> Result<Json<ApiResponseWithData<Page<Event>>>, NotFound<String>> {
    match state.database.get_events(page) {
        Ok(data) => Ok(Json(ApiResponseWithData {
            message: "Operation done".to_string(),
            data: data,
        })),
        Err(_) => Err(NotFound("No ressources".to_string())),
    }
}

#[get("/events/<event_id>")]
pub fn get_event(
    state: &State<AppState>,
    event_id: i32,
) -> Result<Json<ApiResponseWithData<Event>>, (Status, Json<ApiResponse>)> {
    match state.database.get_event_by(event_id) {
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

#[get("/match/<match_id>")]
pub fn get_match(
    state: &State<AppState>,
    match_id: i32,
) -> Result<Json<ApiResponseWithData<Match>>, (Status, Json<ApiResponse>)> {
    match state.database.get_match_by(match_id) {
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
    match state.database.get_ratings_for_match(match_id, page, lang) {
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

#[get("/ratings?<page>&<lang>")]
pub fn get_ratings(
    state: &State<AppState>,
    page: i64,
    lang: Language,
) -> Result<Json<ApiResponseWithData<Page<Rating>>>, (Status, Json<ApiResponse>)> {
    match state.database.get_ratings(page, lang) {
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
    rating: Json<RequestRating>,
) -> Result<Json<ApiResponse>, (Status, Json<ApiResponse>)> {
    match state.database.new_rating(rating.0.into()) {
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

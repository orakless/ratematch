use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::{Insertable, Queryable},
    serialize::ToSql,
    sql_types::VarChar,
};
use rocket::FromFormField;
use serde::{Deserialize, Serialize};

use crate::schema::{event, match_, match_desc, rating};

// based on ISO 3 letter representation of languages for DB and API representation
#[derive(
    AsExpression, Serialize, Deserialize, Clone, Debug, Copy, PartialEq, FromSqlRow, FromFormField,
)]
#[diesel(sql_type = VarChar)]
pub enum Language {
    #[serde(rename = "FRE")]
    // to have the same output as input
    #[field(value = "FRE")]
    French,
    #[serde(rename = "ENG")]
    // to have the same output as input
    #[field(value = "ENG")]
    English,
}

// Implement conversion from DB types (string literals) into Language enum
impl<DB> FromSql<VarChar, DB> for Language
where
    DB: Backend,
    String: FromSql<VarChar, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let value = String::from_sql(bytes)?;
        match value {
            _ if value == "FRE" => Ok(Language::French),
            _ if value == "ENG" => Ok(Language::English),
            lang => Err(format!("Unrecognized language {}", lang).into()),
        }
    }
}

// Implement conversion from Language enum to DB type for inserting new matches or ratings
impl<DB> ToSql<VarChar, DB> for Language
where
    DB: Backend,
    str: ToSql<VarChar, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        match self {
            Language::French => "FRE".to_sql(out),
            Language::English => "ENG".to_sql(out),
        }
    }
}

#[derive(Insertable, Queryable, Serialize, Clone)]
#[diesel(table_name = event)]
pub struct Event {
    id: i32,
    name: String,
    promotion: String,
    date: NaiveDate,
}

#[derive(Insertable, Queryable, Serialize, Clone)]
#[diesel(table_name = match_)]
#[diesel(belongs_to(Event, foreign_key = event_id))]
pub struct Match {
    id: i32,
    event_id: i32,
    workers: String,
}

#[derive(Insertable, Queryable, Serialize, Clone)]
#[diesel(table_name = match_desc)]
#[diesel(belongs_to(Match, foreign_key = match_id))]
pub struct MatchDesc {
    id: i32,
    match_id: i32,
    description: String,
    language_code: Language,
}

#[derive(Insertable, Queryable, Serialize, Clone)]
#[diesel(table_name = rating)]
#[diesel(belongs_to(Match, foreign_key = match_id))]
pub struct Rating {
    id: i32,
    match_id: i32,
    language_code: Language,
    username: String,
    score: BigDecimal,
    publication_date: NaiveDateTime,
    opinion: Option<String>,
}

impl Event {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn promotion(&self) -> &str {
        &self.promotion
    }

    pub fn date(&self) -> &NaiveDate {
        &self.date
    }
}

impl Match {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn event_id(&self) -> i32 {
        self.event_id
    }

    pub fn workers(&self) -> &str {
        &self.workers
    }
}

impl MatchDesc {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn match_id(&self) -> i32 {
        self.match_id
    }

    pub fn language_code(&self) -> Language {
        self.language_code
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

impl Rating {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn match_id(&self) -> i32 {
        self.match_id
    }

    pub fn language_code(&self) -> Language {
        self.language_code
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn score(&self) -> &BigDecimal {
        &self.score
    }

    pub fn opinion(&self) -> &Option<String> {
        &self.opinion
    }

    pub fn publication_date(&self) -> &NaiveDateTime {
        &self.publication_date
    }
}

// Struct without ID, this way it will be possible to
// insert new ratings in the database
#[derive(Insertable, Deserialize)]
#[diesel(table_name = rating)]
pub struct NewRating {
    pub match_id: i32,
    pub language_code: Language,
    pub username: String,
    pub score: BigDecimal,
    pub publication_date: NaiveDateTime,
    pub opinion: Option<String>,
}

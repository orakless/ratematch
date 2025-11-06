use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::{Insertable, Queryable},
    serialize::ToSql,
    sql_types::VarChar,
};

use crate::schema::{event, match_, match_desc, rating};

#[derive(AsExpression, Clone, Debug, Copy, FromSqlRow)]
#[diesel(sql_type = VarChar)]
pub enum Language {
    French,
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

#[derive(Insertable, Queryable)]
#[diesel(table_name = event)]
pub struct Event {
    id: i32,
    name: String,
    promotion: String,
    date: NaiveDate,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = match_)]
#[diesel(belongs_to(Event, foreign_key = event_id))]
pub struct Match {
    id: i32,
    event_id: i32,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = match_desc)]
#[diesel(belongs_to(Match, foreign_key = match_id))]
pub struct MatchDesc {
    id: i32,
    match_id: i32,
    description: String,
    language_code: Language,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = rating)]
#[diesel(belongs_to(Match, foreign_key = match_id))]
pub struct Rating {
    id: i32,
    match_id: i32,
    language_code: Language,
    username: String,
    score: BigDecimal,
    opinion: Option<String>,
}

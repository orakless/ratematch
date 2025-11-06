use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::{Insertable, Queryable},
    sql_types::VarChar,
};

use crate::schema::{event, match_, match_desc, rating};

#[repr(C)]
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
pub struct Match {
    id: i32,
    event_id: i32,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = match_desc)]
pub struct MatchDesc {
    id: i32,
    match_id: i32,
    description: String,
    language_code: Language,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = rating)]
pub struct Rating {
    id: i32,
    match_id: i32,
    language_code: Language,
    username: String,
    score: BigDecimal,
    opinion: String,
}

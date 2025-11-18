use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenvy::dotenv;
use std::env;

use crate::{
    entities::{Event, Language, Match, MatchDesc, NewRating, Rating},
    errors::Error,
    pagination::{Page, Paginate},
    schema::{event, match_, match_desc, rating},
};

pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
}

pub trait ManageDatabaseConnection {
    fn new() -> Self;
    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error>;
}

impl ManageDatabaseConnection for Database {
    fn new() -> Self {
        dotenv().ok();

        // picks from either exported env or .env file
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<PgConnection>::new(&database_url);

        // using a pool is good: multi-connecton support, reusable, ...
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        Self { pool }
    }
    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
        Ok(self.pool.get()?)
    }
}

const ITEMS_PER_PAGE: i64 = 4;

pub trait DatabaseOperations {
    fn get_events(&self, page: i64) -> Result<Page<Event>, Error>;
    fn get_event_by(&self, event_id: i32) -> Result<Event, Error>;
    fn get_match_by(&self, match_id: i32) -> Result<Match, Error>;
    fn get_match_description(&self, match_id: i32, language: Language) -> Result<MatchDesc, Error>;
    fn get_card(&self, event_id: i32) -> Result<Vec<Match>, Error>;
    fn get_ratings(&self, page: i64, language: Language) -> Result<Page<Rating>, Error>;
    fn get_ratings_for_match(
        &self,
        match_id: i32,
        page: i64,
        language: Language,
    ) -> Result<Page<Rating>, Error>;
    fn new_rating(&self, rating: NewRating) -> Result<(), Error>;
}

impl DatabaseOperations for Database {
    fn get_events(&self, page: i64) -> Result<Page<Event>, Error> {
        let mut connection = self.get_connection()?;

        match event::table
            .order_by(event::date.desc())
            .paginate(page)
            .per_page(ITEMS_PER_PAGE)
            .load_and_count_pages::<Event>(&mut connection)
        {
            Ok(page) => Ok(page),
            Err(err) => Err(err.into()),
        }
    }

    fn get_event_by(&self, event_id: i32) -> Result<Event, Error> {
        let mut connection = self.get_connection()?;

        match event::table
            .filter(event::id.eq(event_id))
            .first::<Event>(&mut connection)
        {
            Ok(event) => Ok(event),
            Err(err) => Err(err.into()),
        }
    }

    fn get_match_by(&self, match_id: i32) -> Result<Match, Error> {
        let mut connection = self.get_connection()?;

        match match_::table
            .filter(match_::id.eq(match_id))
            .first::<Match>(&mut connection)
        {
            Ok(match_obj) => Ok(match_obj),
            Err(err) => Err(err.into()),
        }
    }

    fn get_match_description(&self, match_id: i32, language: Language) -> Result<MatchDesc, Error> {
        let mut connection = self.get_connection()?;

        match match_desc::table
            .filter(match_desc::match_id.eq(match_id))
            .filter(match_desc::language_code.eq(language))
            .first::<MatchDesc>(&mut connection)
        {
            Ok(matchdesc) => Ok(matchdesc),
            Err(err) => Err(err.into()),
        }
    }

    fn get_card(&self, event_id: i32) -> Result<Vec<Match>, Error> {
        let mut connection = self.get_connection()?;

        match match_::table
            .filter(match_::event_id.eq(event_id))
            .load::<Match>(&mut connection)
        {
            Ok(card) => Ok(card),
            Err(err) => Err(err.into()),
        }
    }

    fn get_ratings(&self, page: i64, language: Language) -> Result<Page<Rating>, Error> {
        let mut connection = self.get_connection()?;

        match rating::table
            .filter(rating::language_code.eq(language))
            .paginate(page)
            .per_page(ITEMS_PER_PAGE)
            .load_and_count_pages::<Rating>(&mut connection)
        {
            Ok(page) => Ok(page),
            Err(err) => Err(err.into()),
        }
    }

    fn get_ratings_for_match(
        &self,
        match_id: i32,
        page: i64,
        language: Language,
    ) -> Result<Page<Rating>, Error> {
        let mut connection = self.get_connection()?;

        match rating::table
            .filter(rating::match_id.eq(match_id))
            .filter(rating::language_code.eq(language))
            .paginate(page)
            .per_page(ITEMS_PER_PAGE)
            .load_and_count_pages::<Rating>(&mut connection)
        {
            Ok(page) => Ok(page),
            Err(err) => Err(err.into()),
        }
    }

    fn new_rating(&self, rating: NewRating) -> Result<(), Error> {
        let mut connection = self.get_connection()?;

        rating.insert_into(rating::table).execute(&mut connection)?;
        Ok(())
    }
}

use bigdecimal::BigDecimal;
use core::panic;
use diesel::{
    dsl::{self, avg},
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;
use std::env;

use crate::{
    entities::{Event, Language, Match, MatchDesc, NewRating, Rating},
    errors::Error,
    pagination::{Page, Paginate},
    schema::{event, match_, match_desc, rating},
};

// to automatically make the tables
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
}

// Trait used for specifying the ways of connecting and getting a connection pool to the database
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

        match pool.get() {
            Ok(mut connection) => match connection.run_pending_migrations(MIGRATIONS) {
                Ok(_) => (),
                Err(_) => panic!(
                    "Could not run migrations. Use the migrations/*/up.sql scripts manually on your database."
                ),
            },
            Err(_) => panic!(
                "Could not get DB connection for running migrations. Is the database online?"
            ),
        }

        Self { pool }
    }
    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
        Ok(self.pool.get()?)
    }
}

const ITEMS_PER_PAGE: i64 = 4;

// Trait used for specifying the database operations.
pub trait DatabaseOperations {
    /// Returns a list of events, page by page
    fn get_events(&self, page: i64) -> Result<Page<Event>, Error>;
    /// Returns informations about an event
    fn get_event_by(&self, event_id: i32) -> Result<Event, Error>;
    /// Returns information about a match
    fn get_match_by(&self, match_id: i32) -> Result<Match, Error>;
    /// Returns localized description about a match
    fn get_match_description(&self, match_id: i32, language: Language) -> Result<MatchDesc, Error>;
    /// Returns the list of matches associated with an event
    fn get_card(&self, event_id: i32) -> Result<Vec<Match>, Error>;
    /// Returns the global ratings (in a specific language), page by page
    fn get_ratings(&self, page: i64, language: Language) -> Result<Page<Rating>, Error>;
    /// Returns the ratings for an event (in a specific language), page by page
    fn get_ratings_for_event(
        &self,
        event_id: i32,
        page: i64,
        language: Language,
    ) -> Result<Page<Rating>, Error>;
    /// Returns the ratings for a match (in a specific language), page by page
    fn get_ratings_for_match(
        &self,
        match_id: i32,
        page: i64,
        language: Language,
    ) -> Result<Page<Rating>, Error>;
    /// Creates a new rating in the database
    fn new_rating(&self, rating: NewRating) -> Result<(), Error>;
    /// Returns the average score for an event
    fn get_average_rating_for_event(&self, event_id: i32) -> Result<Option<BigDecimal>, Error>;
    /// Returns the average score for a match
    fn get_average_rating_for_match(&self, match_id: i32) -> Result<Option<BigDecimal>, Error>;
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
            .order_by(rating::publication_date.desc())
            .paginate(page)
            .load_and_count_pages::<Rating>(&mut connection)
        {
            Ok(page) => Ok(page),
            Err(err) => Err(err.into()),
        }
    }

    fn get_ratings_for_event(
        &self,
        event_id: i32,
        page: i64,
        language: Language,
    ) -> Result<Page<Rating>, Error> {
        let mut connection = self.get_connection()?;

        match rating::table
            .inner_join(match_::table)
            .filter(match_::event_id.eq(event_id))
            .filter(rating::language_code.eq(language))
            // workaround, not working without (outputs a tuple of rating and id)
            .select(rating::all_columns)
            .order_by(rating::publication_date.desc())
            .paginate(page)
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
            .order_by(rating::publication_date.desc())
            .paginate(page)
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

    fn get_average_rating_for_event(&self, event_id: i32) -> Result<Option<BigDecimal>, Error> {
        let mut connection = self.get_connection()?;

        let ratings = match_::table
            .filter(match_::event_id.eq(event_id))
            .inner_join(rating::table)
            .select(avg(rating::score));

        match ratings.get_result(&mut connection) {
            Ok(avg_score) => Ok(avg_score),
            Err(err) => Err(err.into()),
        }
    }

    fn get_average_rating_for_match(&self, match_id: i32) -> Result<Option<BigDecimal>, Error> {
        let mut connection = self.get_connection()?;

        let ratings = rating::table
            .filter(rating::match_id.eq(match_id))
            .select(dsl::avg(rating::score));

        match ratings.first(&mut connection) {
            Ok(avg_score) => Ok(avg_score),
            Err(err) => Err(err.into()),
        }
    }
}

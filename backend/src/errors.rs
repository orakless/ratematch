use diesel::r2d2;

pub enum Error {
    UsernameAlreadyUsed,
    ResourceDoesNotExists,
    WrongRating,
    Diesel { e: String },
    R2D2 { e: String },
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel {
            e: value.to_string(),
        }
    }
}

impl From<r2d2::PoolError> for Error {
    fn from(value: r2d2::PoolError) -> Self {
        Self::R2D2 {
            e: value.to_string(),
        }
    }
}

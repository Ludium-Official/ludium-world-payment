use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::adapter::output::persistence::db::error::DbError;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Clone, Serialize, Debug)]
pub enum Error {
	DbError(DbError)
}

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

impl From<DbError> for Error {
    fn from(db_error: DbError) -> Self {
        Error::DbError(db_error)
    }
}
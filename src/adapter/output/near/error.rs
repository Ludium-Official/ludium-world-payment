use serde::Serialize;
use serde_with::serde_as;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Clone)]
pub enum Error {
    TransactionNotExecuted { message: String },
	NotWhitelisted { message: String },
}

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		match self {
			Self::TransactionNotExecuted { message } => {
				write!(fmt, "TransactionNotExecuted: {message}")
			}
			Self::NotWhitelisted { message } => {
				write!(fmt, "NotWhitelisted: {message}")
			}
		}
	}
}

impl std::error::Error for Error {}
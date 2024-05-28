use axum::http::StatusCode;
pub mod utrait;
pub mod process_meta_tx;
pub mod reward_claim_usecase_impl;
pub mod near_usecase_impl;
pub mod error;


pub use self::error::Error;
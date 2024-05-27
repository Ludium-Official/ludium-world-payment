use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub trait TimestampTrait {
    fn created_date(&self) -> NaiveDateTime;
    fn updated_date(&self) -> NaiveDateTime;
}

// #[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
// pub struct TimestampFields {
//     pub created_date: NaiveDateTime,
//     pub updated_date: NaiveDateTime,
// }
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::adapter::output::persistence::db::schema::detailed_posting;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = detailed_posting)]
#[diesel(primary_key(detail_id))]
pub struct DetailedPosting {
    pub detail_id: Uuid,
    pub posting_id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub deadline: Option<NaiveDateTime>,
    pub status: String,
    pub is_pinned: bool,
    pub pin_order: i32,
    pub reward_token: Option<Uuid>,
    pub reward_amount: Option<BigDecimal>,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}


#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = detailed_posting)]
pub struct NewDetailedPosting {
    pub detail_id: Uuid,
    pub posting_id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub deadline: Option<NaiveDateTime>,
    pub status: String,
    pub is_pinned: bool,
    pub pin_order: i32,
    pub reward_token: Option<Uuid>,
    pub reward_amount: Option<BigDecimal>,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}
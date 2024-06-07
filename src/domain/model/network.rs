use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use crate::adapter::output::persistence::db::schema::network;
use chrono::NaiveDateTime;
#[derive(Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = network)]
pub struct Network {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = network)]
pub struct NewNetwork {
    pub id: Uuid,
    pub name: String,
    pub code: String,
}

#[derive(Deserialize, Clone)]
pub struct NewNetworkPayload {
    pub name: String,
    pub code: String,
}

#[derive(Serialize, ToSchema)]
pub struct NetworkResponse {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub created_date: i64, 
    pub updated_date: i64, 
}

impl From<Network> for NetworkResponse {
    fn from(network: Network) -> Self {
        NetworkResponse {
            id: network.id.to_string(),
            name: network.name,
            symbol: network.code,
            created_date: network.created_date.and_utc().timestamp(),
            updated_date: network.updated_date.and_utc().timestamp(),
        }
    }
}

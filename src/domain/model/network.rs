use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::adapter::output::persistence::db::schema::network;
use super::TimestampTrait;
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

impl TimestampTrait for Network {
    fn created_date(&self) -> NaiveDateTime {
        self.created_date
    }

    fn updated_date(&self) -> NaiveDateTime {
        self.updated_date
    }
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

#[derive(Serialize)]
pub struct NetworkResponse {
    id: String,
    name: String,
    symbol: String,
    created_date: NaiveDateTime,
    updated_date: NaiveDateTime,
}

impl From<Network> for NetworkResponse {
    fn from(network: Network) -> Self {
        NetworkResponse {
            id: network.id.to_string(),
            name: network.name,
            symbol: network.code,
            created_date: network.created_date,
            updated_date: network.updated_date,
        }
    }
}

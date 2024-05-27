use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::adapter::output::persistence::db::schema::network;

#[derive(Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = network)]
pub struct Network {
    pub id: Uuid,
    pub name: String,
    pub code: String,
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
}

impl From<Network> for NetworkResponse {
    fn from(network: Network) -> Self {
        NetworkResponse {
            id: network.id.to_string(),
            name: network.name,
            symbol: network.code,
        }
    }
}

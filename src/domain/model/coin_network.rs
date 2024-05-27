use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::adapter::output::persistence::db::schema::coin_network;
use crate::domain::model::coin::Coin;
use crate::domain::model::network::Network;

use super::coin::CoinResponse;
use super::network::NetworkResponse;
use super::TimestampTrait;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(table_name = coin_network)]
#[diesel(belongs_to(Coin))]
#[diesel(belongs_to(Network))]
pub struct CoinNetwork {
    pub id: Uuid,
    pub coin_id: Uuid,
    pub network_id: Uuid,
    pub contract_address: Option<String>,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl TimestampTrait for CoinNetwork {
    fn created_date(&self) -> NaiveDateTime {
        self.created_date
    }

    fn updated_date(&self) -> NaiveDateTime {
        self.updated_date
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = coin_network)]
pub struct NewCoinNetwork {
    pub id: Uuid,
    pub coin_id: Uuid,
    pub network_id: Uuid,
    pub contract_address: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct NewCoinNetworkPayload {
    pub coin_id: Uuid,
    pub network_id: Uuid,
    pub contract_address: Option<String>,
}

// region: --- Response 

#[derive(Serialize)]
pub struct CoinNetworkResponse {
    pub id: String,
    pub coin_id: String,
    pub network_id: String,
    pub contract_address: Option<String>,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl From<CoinNetwork> for CoinNetworkResponse {
    fn from(coin_network: CoinNetwork) -> Self {
        CoinNetworkResponse {
            id: coin_network.id.to_string(),
            coin_id: coin_network.coin_id.to_string(),
            network_id: coin_network.network_id.to_string(),
            contract_address: coin_network.contract_address,
            created_date: coin_network.created_date,
            updated_date: coin_network.updated_date,
        }
    }
}

#[derive(Serialize)]
pub struct CoinNetworkDetailsResponse {
    pub id: String,
    pub coin: CoinResponse,
    pub network: NetworkResponse,
    pub contract_address: Option<String>,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl From<(CoinNetwork, Coin, Network)> for CoinNetworkDetailsResponse {
    fn from((coin_network, coin, network): (CoinNetwork, Coin, Network)) -> Self {
        CoinNetworkDetailsResponse {
            coin: CoinResponse::from(coin),
            network: NetworkResponse::from(network),
            id: coin_network.id.to_string(),
            contract_address: coin_network.contract_address,
            created_date: coin_network.created_date,
            updated_date: coin_network.updated_date,
        }
    }
}

// endregion: --- Response 
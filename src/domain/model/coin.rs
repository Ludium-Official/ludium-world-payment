use chrono::NaiveDateTime;
use diesel_derive_enum::DbEnum;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::adapter::output::persistence::db::schema::coin;

#[derive(Clone, Debug, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::adapter::output::persistence::db::schema::sql_types::CoinType"]
pub enum CoinType {
    #[db_rename = "NATIVE"]
    Native,
    #[db_rename = "FT"]
    FT,
    #[db_rename = "NFT"]
    NFT,
}

impl From<String> for CoinType {
    fn from(coin_type: String) -> Self {
        match coin_type.to_uppercase().as_str() {
            "NATIVE" => CoinType::Native,
            "FT" => CoinType::FT,
            "NFT" => CoinType::NFT,
            _ => CoinType::Native,
        }
    }
}

impl PartialEq for CoinType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CoinType::FT, CoinType::FT) => true,
            (CoinType::NFT, CoinType::NFT) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Queryable, Identifiable)]
#[diesel(table_name = coin)]
pub struct Coin {
    pub id: Uuid,
    pub name: String,
    pub symbol: String,
    pub coin_type: CoinType,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Debug)]
pub struct CoinWithNetwork {
    pub coin: Coin,
    pub coin_network_id: Uuid,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = coin)]
pub struct NewCoin {
    pub id: Uuid,
    pub name: String,
    pub symbol: String,
    pub coin_type: CoinType,
}

#[derive(Deserialize, Clone)]
pub struct NewCoinPayload {
    pub name: String,
    pub symbol: String,
    pub coin_type: String,
}

#[derive(Serialize)]
pub struct CoinResponse {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub coin_type: CoinType,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
    pub coin_network_id: Option<String>,
}


impl From<CoinWithNetwork> for CoinResponse {
    fn from(cwn: CoinWithNetwork) -> Self {
        CoinResponse {
            id: cwn.coin.id.to_string(),
            name: cwn.coin.name,
            symbol: cwn.coin.symbol,
            coin_type: cwn.coin.coin_type,
            created_date: cwn.coin.created_date,
            updated_date: cwn.coin.updated_date,
            coin_network_id: Some(cwn.coin_network_id.to_string()),
        }
    }
}

impl From<Coin> for CoinResponse {
    fn from(coin: Coin) -> Self {
        CoinResponse {
            id: coin.id.to_string(),
            name: coin.name,
            symbol: coin.symbol,
            coin_type: coin.coin_type,
            created_date: coin.created_date,
            updated_date: coin.updated_date,
            coin_network_id: None,
        }
    }
}
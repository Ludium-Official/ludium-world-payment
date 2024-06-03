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

impl core::fmt::Display for CoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoinType::Native => write!(f, "NATIVE"),
            CoinType::FT => write!(f, "FT"),
            CoinType::NFT => write!(f, "NFT"),
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
    pub decimals: i32,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = coin)]
pub struct NewCoin {
    pub id: Uuid,
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub coin_type: CoinType,
}

#[derive(Deserialize, Clone)]
pub struct NewCoinPayload {
    pub name: String,
    pub symbol: String,
    pub coin_type: String,
    pub decimals: i32,
}

#[derive(Serialize)]
pub struct CoinResponse {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub coin_type: CoinType,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}



impl From<Coin> for CoinResponse {
    fn from(coin: Coin) -> Self {
        CoinResponse {
            id: coin.id.to_string(),
            name: coin.name,
            symbol: coin.symbol,
            decimals: coin.decimals,
            coin_type: coin.coin_type,
            created_date: coin.created_date,
            updated_date: coin.updated_date,
        }
    }
}
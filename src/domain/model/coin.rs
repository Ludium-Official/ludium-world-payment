use diesel::sql_types::Text;
use diesel_derive_enum::DbEnum;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::adapter::output::persistence::db::schema::{coin, coin_network, network};

#[derive(Clone, Debug, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::adapter::output::persistence::db::schema::sql_types::CoinType"]
pub enum CoinType {
    #[db_rename = "NATIVE"]
    NATIVE,
    #[db_rename = "FT"]
    FT,
    #[db_rename = "NFT"]
    NFT,
}

impl From<String> for CoinType {
    fn from(coin_type: String) -> Self {
        match coin_type.to_uppercase().as_str() {
            "NATIVE" => CoinType::NATIVE,
            "FT" => CoinType::FT,
            "NFT" => CoinType::NFT,
            _ => CoinType::NATIVE,
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
}

impl From<Coin> for CoinResponse {
    fn from(coin: Coin) -> Self {
        CoinResponse {
            id: coin.id.to_string(),
            name: coin.name,
            symbol: coin.symbol,
            coin_type: coin.coin_type,
        }
    }
}

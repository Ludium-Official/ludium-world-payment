use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use diesel_derive_enum::DbEnum;
use super::coin::Coin;
use super::coin_network::CoinNetworkDetailsResponse;
use super::network::Network;
use super::reward_claim_detail::{RewardClaimDetail, RewardClaimDetailResponse};
use crate::domain::model::coin_network::CoinNetwork;
use crate::adapter::output::persistence::db::schema::reward_claim;

#[derive(Clone, Debug, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::adapter::output::persistence::db::schema::sql_types::RewardClaimStatus"]
pub enum RewardClaimStatus {
    #[db_rename = "READY"]
    Ready, 
    #[db_rename = "TRANSACTION_APPROVED"]
    TransactionApproved,
    #[db_rename = "TRANSACTION_FAILED"]
    TransactionFailed,
}

impl From<String> for RewardClaimStatus {
    fn from(reward_claim_status: String) -> Self {
        match reward_claim_status.to_uppercase().as_str() {
            "READY" => RewardClaimStatus::Ready,
            "TRANSACTION_APPROVED" => RewardClaimStatus::TransactionApproved,
            "TRANSACTION_FAILED" => RewardClaimStatus::TransactionFailed,
            _ => RewardClaimStatus::Ready,
        }
    }
}

impl PartialEq for RewardClaimStatus {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RewardClaimStatus::Ready, RewardClaimStatus::Ready) => true,
            (RewardClaimStatus::TransactionApproved, RewardClaimStatus::TransactionApproved) => true,
            (RewardClaimStatus::TransactionFailed, RewardClaimStatus::TransactionFailed) => true,
            _ => false,
        }
    }
}

impl core::fmt::Display for RewardClaimStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RewardClaimStatus::Ready => write!(f, "READY"),
            RewardClaimStatus::TransactionApproved => write!(f, "TRANSACTION_APPROVED"),
            RewardClaimStatus::TransactionFailed => write!(f, "TRANSACTION_FAILED"),
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::adapter::output::persistence::db::schema::sql_types::ResourceType"]
pub enum ResourceType {
    #[db_rename = "MISSION"]
    Mission,
    #[db_rename = "DETAILED_POSTING"]
    DetailedPosting    
}

impl From<String> for ResourceType {
    fn from(resource_type: String) -> Self {
        match resource_type.to_uppercase().as_str() {
            "MISSION" => ResourceType::Mission,
            "DETAILED_POSTING" => ResourceType::DetailedPosting,
            _ => panic!("Invalid resource_type"), // You might want to handle this more gracefully
        }
    }
}

impl PartialEq for ResourceType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ResourceType::Mission, ResourceType::Mission) => true,
            (ResourceType::DetailedPosting, ResourceType::DetailedPosting) => true,
            (ResourceType::Mission, ResourceType::DetailedPosting) => false,
            (ResourceType::DetailedPosting, ResourceType::Mission) => false,
        }
    }
}


impl core::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::Mission => write!(f, "MISSION"),
            ResourceType::DetailedPosting => write!(f, "DETAILED_POSTING"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = reward_claim)]
pub struct RewardClaim {
    pub id: Uuid,
    pub resource_id: Uuid,
    pub resource_type: ResourceType,
    pub coin_network_id: Uuid,
    pub reward_claim_status: RewardClaimStatus,
    pub amount: BigDecimal,
    pub user_id: Uuid,
    pub user_address: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = reward_claim)]
pub struct NewRewardClaim {
    pub id: Uuid,
    pub resource_id: Uuid,
    pub resource_type: ResourceType,
    pub coin_network_id: Uuid,
    pub reward_claim_status: RewardClaimStatus,
    pub amount: BigDecimal,   
    pub user_id: Uuid,
    pub user_address: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = reward_claim)]
pub struct UpdateRewardClaimStatus {
    pub reward_claim_status: RewardClaimStatus,
    pub updated_date: NaiveDateTime,
}

#[derive(Deserialize, Clone, ToSchema)]
pub struct NewRewardClaimPayload {
    #[schema(value_type = String)]
    pub resource_id: Uuid,
    #[schema(value_type = String, example = "MISSION | DETAILED_POSTING")]
    pub resource_type: String,
    #[schema(value_type = String)]
    pub coin_network_id: Uuid,
    pub amount: String,
    pub user_address: String,
}

#[derive(Serialize, ToSchema)]
pub struct CombinedRewardClaimResponse {
    id: String,
    amount: String,
    resource_id: String,
    resource_type: String,
    coin_network: CoinNetworkDetailsResponse,
    user_id: String,
    user_address: String,
    reward_claim_status: String,
    detail: RewardClaimDetailResponse,
    created_date: i64,
    updated_date: i64,
}

impl From<(RewardClaim, RewardClaimDetail, CoinNetwork, Coin, Network)> for CombinedRewardClaimResponse {
    fn from((claim, detail, coin_network, coin, network): (RewardClaim, RewardClaimDetail, CoinNetwork, Coin, Network)) -> Self {
        Self {
            id: claim.id.to_string(),
            amount: claim.amount.to_string(),
            resource_id: claim.resource_id.to_string(),
            resource_type: claim.resource_type.to_string(),
            coin_network: CoinNetworkDetailsResponse::from((coin_network, coin, network)),
            user_id: claim.user_id.to_string(),
            user_address: claim.user_address,
            reward_claim_status: claim.reward_claim_status.to_string(),
            detail: RewardClaimDetailResponse::from(detail),
            created_date: claim.created_date.and_utc().timestamp(),
            updated_date: claim.updated_date.and_utc().timestamp(),
        }
    }
}

#[derive(Serialize)]
pub struct RewardClaimResponse {
    id: String,
    amount: String,
    resource_id: String,
    resource_type: String,
    coin_network_id: String,
    user_id: String,
    user_address: String,
    reward_claim_status: String,
    created_date: i64,
    updated_date: i64,
}

impl From<RewardClaim> for RewardClaimResponse {
    fn from(claim: RewardClaim) -> Self {
        Self {
            id: claim.id.to_string(),
            amount: claim.amount.to_string(),
            resource_id: claim.resource_id.to_string(),
            resource_type: claim.resource_type.to_string(),
            coin_network_id: claim.coin_network_id.to_string(),
            user_id: claim.user_id.to_string(),
            user_address: claim.user_address,
            reward_claim_status: claim.reward_claim_status.to_string(),
            created_date: claim.created_date.and_utc().timestamp(),
            updated_date: claim.updated_date.and_utc().timestamp(),
        }
    }
}


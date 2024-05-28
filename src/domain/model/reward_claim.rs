use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel_derive_enum::DbEnum;
use diesel::sql_types::Decimal;
use bigdecimal::BigDecimal;
use super::coin::Coin;
use super::coin_network::{CoinNetworkDetailsResponse, CoinNetworkResponse};
use super::network::Network;
use super::reward_claim_detail::{RewardClaimDetail, RewardClaimDetailResponse};
use super::TimestampTrait;
use crate::domain::model::coin_network::CoinNetwork;
use crate::adapter::output::persistence::db::schema::reward_claim;

#[derive(Clone, Debug, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::adapter::output::persistence::db::schema::sql_types::RewardClaimStatus"]
pub enum RewardClaimStatus {
    #[db_rename = "READY"]
    Ready,
    #[db_rename = "PENDING_APPROVAL"]
    PendingApproval,
    #[db_rename = "TRANSACTION_APPROVED"]
    TransactionApproved,
    #[db_rename = "TRANSACTION_FAILED"]
    TransactionFailed,
    #[db_rename = "REJECTED"]
    Rejected,
}

impl From<String> for RewardClaimStatus {
    fn from(reward_claim_status: String) -> Self {
        match reward_claim_status.to_uppercase().as_str() {
            "READY" => RewardClaimStatus::Ready,
            "PENDING_APPROVAL" => RewardClaimStatus::PendingApproval,
            "TRANSACTION_APPROVED" => RewardClaimStatus::TransactionApproved,
            "TRANSACTION_FAILED" => RewardClaimStatus::TransactionFailed,
            "REJECTED" => RewardClaimStatus::Rejected,
            _ => RewardClaimStatus::Ready,
        }
    }
}

impl PartialEq for RewardClaimStatus {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RewardClaimStatus::Ready, RewardClaimStatus::Ready) => true,
            (RewardClaimStatus::PendingApproval, RewardClaimStatus::PendingApproval) => true,
            (RewardClaimStatus::TransactionApproved, RewardClaimStatus::TransactionApproved) => true,
            (RewardClaimStatus::TransactionFailed, RewardClaimStatus::TransactionFailed) => true,
            (RewardClaimStatus::Rejected, RewardClaimStatus::Rejected) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = reward_claim)]
pub struct RewardClaim {
    pub id: Uuid,
    pub mission_id: Uuid,
    pub coin_network_id: Uuid,
    pub reward_claim_status: RewardClaimStatus,
    pub amount: BigDecimal,
    pub user_id: Uuid,
    pub user_address: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl TimestampTrait for RewardClaim {
    fn created_date(&self) -> NaiveDateTime {
        self.created_date
    }

    fn updated_date(&self) -> NaiveDateTime {
        self.updated_date
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = reward_claim)]
pub struct NewRewardClaim {
    pub id: Uuid,
    pub mission_id: Uuid,
    pub coin_network_id: Uuid,
    pub reward_claim_status: RewardClaimStatus,
    pub amount: BigDecimal,
    pub user_id: Uuid,
    pub user_address: String,
}

#[derive(Deserialize, Clone)]
pub struct NewRewardClaimPayload {
    pub mission_id: Uuid,
    pub coin_network_id: Uuid,
    pub amount: BigDecimal,
    pub user_id: Uuid,
    pub user_address: String,
}

#[derive(Serialize)]
pub struct CombinedRewardClaimResponse {
    id: String,
    amount: String,
    mission_id: String,
    coin_network: CoinNetworkDetailsResponse,
    user_id: String,
    user_address: String,
    reward_claim_status: RewardClaimStatus,
    created_date: String,
    updated_date: String,
}

impl From<(RewardClaim, CoinNetwork, Coin, Network)> for CombinedRewardClaimResponse {
    fn from((claim, coin_network, coin, network): (RewardClaim, CoinNetwork, Coin, Network)) -> Self {
        Self {
            id: claim.id.to_string(),
            amount: claim.amount.to_string(),
            mission_id: claim.mission_id.to_string(),
            coin_network: CoinNetworkDetailsResponse::from((coin_network, coin, network)),
            user_id: claim.user_id.to_string(),
            user_address: claim.user_address,
            reward_claim_status: claim.reward_claim_status,
            created_date: claim.created_date.to_string(),
            updated_date: claim.updated_date.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct RewardClaimResponse {
    id: String,
    amount: String,
    mission_id: String,
    coin_network_id: String,
    user_id: String,
    user_address: String,
    reward_claim_status: RewardClaimStatus,
    created_date: String,
    updated_date: String,
}

impl From<RewardClaim> for RewardClaimResponse {
    fn from(claim: RewardClaim) -> Self {
        Self {
            id: claim.id.to_string(),
            amount: claim.amount.to_string(),
            mission_id: claim.mission_id.to_string(),
            coin_network_id: claim.coin_network_id.to_string(),
            user_id: claim.user_id.to_string(),
            user_address: claim.user_address,
            reward_claim_status: claim.reward_claim_status,
            created_date: claim.created_date.to_string(),
            updated_date: claim.updated_date.to_string(),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct RewardClaimApprovePayload {
    pub encode_signed_delegate: Vec<u8>,
}

#[derive(Serialize)]
pub struct RewardClaimApproveResponse {
    reward_claim: RewardClaimResponse,
    reward_claim_detail: RewardClaimDetailResponse
}

impl From<(RewardClaim, RewardClaimDetail)> for RewardClaimApproveResponse {
    fn from((claim, detail): (RewardClaim, RewardClaimDetail)) -> Self {
        Self {
            reward_claim: RewardClaimResponse::from(claim),
            reward_claim_detail: RewardClaimDetailResponse::from(detail),
        }
    }
}

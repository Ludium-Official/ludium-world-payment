use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::adapter::output::persistence::db::schema::reward_claim_detail;
use crate::domain::model::reward_claim::RewardClaim;


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Associations, Selectable)]
#[diesel(table_name = reward_claim_detail)]
#[diesel(belongs_to(RewardClaim))]
pub struct RewardClaimDetail {
    pub id: Uuid,
    pub reward_claim_id: Uuid,
    pub transaction_hash: String,
    pub sended_user_id: Uuid,
    pub sended_user_address: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = reward_claim_detail)]
pub struct NewRewardClaimDetail {
    pub id: Uuid,
    pub reward_claim_id: Uuid,
    pub transaction_hash: String,
    pub sended_user_id: Uuid,
    pub sended_user_address: String,
}


#[derive(Serialize, ToSchema)]
pub struct RewardClaimDetailResponse {
    id: String,
    reward_claim_id: String,
    transaction_hash: String,
    sended_user_id: String,
    sended_user_address: String,
    created_date: i64,
    updated_date: i64,
}

impl From<RewardClaimDetail> for RewardClaimDetailResponse {
    fn from(reward_claim_detail: RewardClaimDetail) -> Self {
        RewardClaimDetailResponse {
            id: reward_claim_detail.id.to_string(),
            reward_claim_id: reward_claim_detail.reward_claim_id.to_string(),
            transaction_hash: reward_claim_detail.transaction_hash,
            sended_user_id: reward_claim_detail.sended_user_id.to_string(),
            sended_user_address: reward_claim_detail.sended_user_address,
            created_date: reward_claim_detail.created_date.and_utc().timestamp(),
            updated_date: reward_claim_detail.updated_date.and_utc().timestamp(),
        }
    }
}
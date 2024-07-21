use std::collections::HashMap;

use axum::async_trait;
use deadpool_diesel::postgres::Object;
use diesel::prelude::*;
use uuid::Uuid;
use crate::{adapter::output::persistence::db::schema::reward_claim_detail, domain::model::{reward_claim::{NewRewardClaim, ResourceType, RewardClaim, RewardClaimStatus, UpdateRewardClaimStatus}, reward_claim_detail::{NewRewardClaimDetail, RewardClaimDetail}}};
use crate::port::output::reward_claim_repository::RewardClaimRepository;
use super::{Error, Result, adapt_db_error, reward_claim};

#[derive(Clone, Debug)]
pub struct PostgresRewardClaimRepository;

#[async_trait]
impl RewardClaimRepository for PostgresRewardClaimRepository {
    async fn insert(&self, conn: Object, new_reward_claim: NewRewardClaim) -> Result<RewardClaim> {
        conn.interact(move |conn| {
            diesel::insert_into(reward_claim::table)
                .values(new_reward_claim)
                .returning(RewardClaim::as_select()) 
                .get_result::<RewardClaim>(conn)
        })
        .await?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn get_by_resource_and_user(
        &self,
        conn: Object,
        resource_type: ResourceType,
        resource_id: Uuid,
        user_id: Uuid
    ) -> Result<RewardClaim> {
        conn.interact(move |conn| {
            reward_claim::table
                .filter(reward_claim::resource_type.eq(resource_type))
                .filter(reward_claim::resource_id.eq(resource_id))
                .filter(reward_claim::user_id.eq(user_id))
                .select(RewardClaim::as_select())
                .for_update()
                .first::<RewardClaim>(conn)
        })
        .await?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn insert_detail(&self, conn: Object, new_reward_claim_detail: NewRewardClaimDetail) -> Result<RewardClaimDetail> {
        conn.interact(move |conn| {
            diesel::insert_into(reward_claim_detail::table)
                .values(new_reward_claim_detail)
                .get_result::<RewardClaimDetail>(conn)
        })
        .await?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn list_all_by_user(&self, conn: Object, user_id: Uuid) -> Result<Vec<(RewardClaim, RewardClaimDetail)>> {
        let result = conn.interact(move |conn| {
            reward_claim::table
                .filter(reward_claim::user_id.eq(user_id))
                .inner_join(reward_claim_detail::table)
                .select((RewardClaim::as_select(), RewardClaimDetail::as_select())) 
                .load::<(RewardClaim, RewardClaimDetail)>(conn)
        })
        .await?;

        match result {
            Ok(claim_details) => {
                let mut latest_claim_details: HashMap<Uuid, (RewardClaim, RewardClaimDetail)> = HashMap::new();
                for (claim, detail) in claim_details {
                    let entry = latest_claim_details.entry(claim.id).or_insert((claim.clone(), detail.clone()));
                    if detail.created_date > entry.1.created_date {
                        *entry = (claim, detail);
                    }
                }
                Ok(latest_claim_details.into_iter().map(|(_, v)| v).collect())
            },
            Err(e) => Err(Error::from(adapt_db_error(e))),
        }
    }

    async fn update_status(&self, conn: Object, reward_claim_id: Uuid, status: RewardClaimStatus, retryable: bool) -> Result<RewardClaim>{
        conn.interact(move |conn| {
            conn.transaction(|conn| {
                let target_claim = reward_claim::table
                    .filter(reward_claim::id.eq(reward_claim_id))
                    .for_update()
                    .select(RewardClaim::as_select()) 
                    .first::<RewardClaim>(conn)?;
                
                if retryable && target_claim.reward_claim_status == RewardClaimStatus::Ready {
                    tracing::error!(
                        "[Ready - update failed] Reward Claim Duplicate: Resource Id: {}, Resource Type: {}, User Id: {}",
                        target_claim.resource_id, target_claim.resource_type, target_claim.user_id
                    );
                    return Err(diesel::result::Error::RollbackTransaction);
                }

                let changes = UpdateRewardClaimStatus {
                    reward_claim_status: status,
                    updated_date: chrono::Utc::now().naive_utc(),
                };

                diesel::update(reward_claim::table)
                    .filter(reward_claim::id.eq(target_claim.id))
                    .set(&changes)
                    .returning(RewardClaim::as_select())
                    .get_result::<RewardClaim>(conn)
            })
        })
        .await?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }
}


// region: --- reward claim repository tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::output::persistence::db::_dev_utils;
    use crate::domain::model::reward_claim::{ResourceType, RewardClaimStatus};
    use crate::domain::model::reward_claim_detail::NewRewardClaimDetail;
    use crate::port::output::reward_claim_repository::RewardClaimRepository;
    use crate::port::output::DbManager;
    use bigdecimal::BigDecimal;
    use serial_test::serial;
    use uuid::Uuid;

    #[serial]
    #[tokio::test]
    async fn test_get_by_resource_and_user() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresRewardClaimRepository;

        let resource_type = ResourceType::Mission;
        let new_reward_claim = NewRewardClaim {
            id: Uuid::new_v4(),
            resource_id: Uuid::new_v4(),
            resource_type: resource_type.clone(),
            coin_network_id: Uuid::new_v4(),
            amount: BigDecimal::from(20000),
            user_id: Uuid::new_v4(),
            user_address: "test_address_1".to_string(),
            reward_claim_status: RewardClaimStatus::TransactionApproved,
        };

        let inserted_claim = repo.insert(db_manager.get_connection().await?, new_reward_claim.clone()).await?;
        let fetched_claim = repo.get_by_resource_and_user(db_manager.get_connection().await?, resource_type.clone(), new_reward_claim.resource_id, new_reward_claim.user_id).await?;
        assert_eq!(fetched_claim.id, inserted_claim.id);
        assert_eq!(fetched_claim.resource_id, inserted_claim.resource_id);
        assert_eq!(fetched_claim.user_id, inserted_claim.user_id);

        let not_found_claim = repo.get_by_resource_and_user(db_manager.get_connection().await?, resource_type.clone(), Uuid::new_v4(), Uuid::new_v4()).await;
        assert!(not_found_claim.is_err());

        let not_found_claim2 = repo.get_by_resource_and_user(db_manager.get_connection().await?, ResourceType::DetailedPosting, new_reward_claim.resource_id, new_reward_claim.user_id).await;
        assert!(not_found_claim2.is_err());

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_insert_all() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresRewardClaimRepository;

        let new_reward_claim = NewRewardClaim {
            id: Uuid::new_v4(),
            resource_id: Uuid::new_v4(),
            resource_type: ResourceType::Mission,
            coin_network_id: Uuid::new_v4(),
            amount: BigDecimal::from(10000),
            user_id: Uuid::new_v4(),
            user_address: "test_address".to_string(),
            reward_claim_status: RewardClaimStatus::TransactionApproved,
        };
        
        let inserted_claim = repo.insert(db_manager.get_connection().await?, new_reward_claim.clone()).await?;

        let new_reward_claim_detail = NewRewardClaimDetail {
            id: Uuid::new_v4(),
            reward_claim_id: inserted_claim.id,
            transaction_hash: "test_hash".to_string(),
            sended_user_id: Uuid::new_v4(),
            sended_user_address: "sended_address".to_string(),
        };

        let inserted_detail = repo.insert_detail(db_manager.get_connection().await?, new_reward_claim_detail.clone()).await?;
        assert_eq!(inserted_detail.reward_claim_id, inserted_claim.id);
        assert_eq!(inserted_detail.transaction_hash, new_reward_claim_detail.transaction_hash);
        assert_eq!(inserted_detail.sended_user_id, new_reward_claim_detail.sended_user_id);
        assert_eq!(inserted_detail.sended_user_address, new_reward_claim_detail.sended_user_address);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_insert_error_unique() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresRewardClaimRepository;

        let resource_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let new_reward_claim_1 = NewRewardClaim {
            id: Uuid::new_v4(),
            resource_id,
            resource_type: ResourceType::Mission,
            coin_network_id: Uuid::new_v4(),
            amount: BigDecimal::from(10000),
            user_id,
            user_address: "test_address_1".to_string(),
            reward_claim_status: RewardClaimStatus::Ready,
        };

        let result_1 = repo.insert(db_manager.get_connection().await?, new_reward_claim_1).await;
        assert!(result_1.is_ok());

        let new_reward_claim_2 = NewRewardClaim {
            id: Uuid::new_v4(),
            resource_id,
            resource_type: ResourceType::DetailedPosting,
            coin_network_id: Uuid::new_v4(),
            amount: BigDecimal::from(20000),
            user_id,
            user_address: "test_address_2".to_string(),
            reward_claim_status: RewardClaimStatus::Ready,
        };

        let result_2 = repo.insert(db_manager.get_connection().await?, new_reward_claim_2).await;
        assert!(result_2.is_ok()); 

        let new_reward_claim_3 = NewRewardClaim {
            id: Uuid::new_v4(),
            resource_id,
            resource_type: ResourceType::Mission,
            coin_network_id: Uuid::new_v4(),
            amount: BigDecimal::from(20000),
            user_id,
            user_address: "test_address_2".to_string(),
            reward_claim_status: RewardClaimStatus::Ready,
        };

        let result_3 = repo.insert(db_manager.get_connection().await?, new_reward_claim_3).await;
        assert!(result_3.is_err()); 


        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_insert_detail_error() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresRewardClaimRepository;

        let invalid_reward_claim_detail = NewRewardClaimDetail {
            id: Uuid::nil(), 
            reward_claim_id: Uuid::nil(), // fk error
            transaction_hash: "".to_string(), 
            sended_user_id: Uuid::nil(),
            sended_user_address: "sended_address".to_string(), 
        };

        let result = repo.insert_detail(db_manager.get_connection().await?, invalid_reward_claim_detail).await;
        assert!(result.is_err());

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_all_by_user() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresRewardClaimRepository;
        let user_id = Uuid::new_v4();

        let new_reward_claim_1 = NewRewardClaim {
            id: Uuid::new_v4(),
            resource_id: Uuid::new_v4(),
            resource_type: ResourceType::Mission,
            coin_network_id: Uuid::new_v4(),
            amount: BigDecimal::from(10000),
            user_id,
            user_address: "test_address_1".to_string(),
            reward_claim_status: RewardClaimStatus::TransactionApproved,
        };

        let new_reward_claim_2 = NewRewardClaim {
            id: Uuid::new_v4(),
            resource_id: Uuid::new_v4(),
            resource_type: ResourceType::DetailedPosting,
            coin_network_id: Uuid::new_v4(),
            amount: BigDecimal::from(20000),
            user_id,
            user_address: "test_address_2".to_string(),
            reward_claim_status: RewardClaimStatus::TransactionApproved,
        };

        let inserted_claim_1 = repo.insert(db_manager.get_connection().await?, new_reward_claim_1.clone()).await?;
        let inserted_claim_2 = repo.insert(db_manager.get_connection().await?, new_reward_claim_2.clone()).await?;

        let new_reward_claim_detail_1_1 = NewRewardClaimDetail {
            id: Uuid::new_v4(),
            reward_claim_id: inserted_claim_1.id,
            transaction_hash: "test_hash_1".to_string(),
            sended_user_id: Uuid::new_v4(),
            sended_user_address: "sended_address_1".to_string(),
        };

        let new_reward_claim_detail_1_2 = NewRewardClaimDetail {
            id: Uuid::new_v4(),
            reward_claim_id: inserted_claim_1.id,
            transaction_hash: "test_hash_1_2".to_string(),
            sended_user_id: Uuid::new_v4(),
            sended_user_address: "sended_address_1".to_string(),
        };

        let new_reward_claim_detail_2_1 = NewRewardClaimDetail {
            id: Uuid::new_v4(),
            reward_claim_id: inserted_claim_2.id,
            transaction_hash: "test_hash_2".to_string(),
            sended_user_id: Uuid::new_v4(),
            sended_user_address: "sended_address_2".to_string(),
        };
        let new_reward_claim_detail_2_2 = NewRewardClaimDetail {
            id: Uuid::new_v4(),
            reward_claim_id: inserted_claim_2.id,
            transaction_hash: "test_hash_2".to_string(),
            sended_user_id: Uuid::new_v4(),
            sended_user_address: "sended_address_2".to_string(),
        };
        let new_reward_claim_detail_2_3 = NewRewardClaimDetail {
            id: Uuid::new_v4(),
            reward_claim_id: inserted_claim_2.id,
            transaction_hash: "test_hash_2".to_string(),
            sended_user_id: Uuid::new_v4(),
            sended_user_address: "sended_address_2".to_string(),
        };

        repo.insert_detail(db_manager.get_connection().await?, new_reward_claim_detail_1_1.clone()).await?;
        repo.insert_detail(db_manager.get_connection().await?, new_reward_claim_detail_1_2.clone()).await?;
        repo.insert_detail(db_manager.get_connection().await?, new_reward_claim_detail_2_1.clone()).await?;
        repo.insert_detail(db_manager.get_connection().await?, new_reward_claim_detail_2_2.clone()).await?;
        repo.insert_detail(db_manager.get_connection().await?, new_reward_claim_detail_2_3.clone()).await?;

        let claims = repo.list_all_by_user(db_manager.get_connection().await?, user_id).await?;
        assert_eq!(claims.len(), 2);

        claims.iter().for_each(|(claim, detail)| {
            if claim.resource_type == ResourceType::Mission {
                assert_eq!(claim.id, inserted_claim_1.id);
                assert_eq!(detail.transaction_hash, new_reward_claim_detail_1_2.transaction_hash);
            } else {
                assert_eq!(claim.id, inserted_claim_2.id);
                assert_eq!(detail.transaction_hash, new_reward_claim_detail_2_3.transaction_hash);
            }
        });

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_status() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresRewardClaimRepository;

        let new_reward_claim = NewRewardClaim {
            id: Uuid::new_v4(),
            resource_id: Uuid::new_v4(),
            resource_type: ResourceType::Mission,
            coin_network_id: Uuid::new_v4(),
            amount: BigDecimal::from(10000),
            user_id: Uuid::new_v4(),
            user_address: "test_address".to_string(),
            reward_claim_status: RewardClaimStatus::Ready,
        };

        let inserted_claim = repo.insert(db_manager.get_connection().await?, new_reward_claim.clone()).await?;
        let updated_claim = repo.update_status(db_manager.get_connection().await?, inserted_claim.id, RewardClaimStatus::TransactionApproved, false).await?;
        
        assert_eq!(updated_claim.reward_claim_status, RewardClaimStatus::TransactionApproved);
        assert_ne!(updated_claim.updated_date, inserted_claim.updated_date);

        Ok(())
    }
}

// endregion: --- reward claim repository tests
// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "coin_type"))]
    pub struct CoinType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "reward_claim_status"))]
    pub struct RewardClaimStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CoinType;

    coin (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 10]
        symbol -> Varchar,
        coin_type -> CoinType,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    coin_network (id) {
        id -> Uuid,
        coin_id -> Uuid,
        network_id -> Uuid,
        #[max_length = 100]
        contract_address -> Nullable<Varchar>,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    network (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 20]
        code -> Varchar,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RewardClaimStatus;

    reward_claim (id) {
        id -> Uuid,
        mission_id -> Uuid,
        coin_network_id -> Uuid,
        reward_claim_status -> RewardClaimStatus,
        amount -> Numeric,
        user_id -> Uuid,
        #[max_length = 100]
        user_address -> Varchar,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    reward_claim_detail (id) {
        id -> Uuid,
        reward_claim_id -> Uuid,
        #[max_length = 100]
        transaction_hash -> Varchar,
        sended_user_id -> Uuid,
        #[max_length = 100]
        sended_user_address -> Varchar,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    tb_ldm_usr (id) {
        id -> Uuid,
        #[max_length = 30]
        nick -> Varchar,
        #[max_length = 100]
        self_intro -> Varchar,
        phn_nmb -> Varchar,
    }
}

diesel::table! {
    tb_ldm_usr_rgh (id) {
        id -> Uuid,
        is_crt -> Bool,
        is_prv -> Bool,
        is_adm -> Bool,
    }
}

diesel::joinable!(coin_network -> coin (coin_id));
diesel::joinable!(coin_network -> network (network_id));
diesel::joinable!(reward_claim_detail -> reward_claim (reward_claim_id));
diesel::joinable!(tb_ldm_usr_rgh -> tb_ldm_usr (id));

diesel::allow_tables_to_appear_in_same_query!(
    coin,
    coin_network,
    network,
    reward_claim,
    reward_claim_detail,
    tb_ldm_usr,
    tb_ldm_usr_rgh,
);

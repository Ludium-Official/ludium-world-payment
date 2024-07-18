// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "coin_type"))]
    pub struct CoinType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "resource_type"))]
    pub struct ResourceType;

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
        decimals -> Int4,
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
    detailed_posting (detail_id) {
        detail_id -> Uuid,
        posting_id -> Uuid,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        description -> Nullable<Text>,
        deadline -> Nullable<Timestamp>,
        #[max_length = 50]
        status -> Varchar,
        is_pinned -> Bool,
        pin_order -> Int4,
        reward_token -> Nullable<Uuid>,
        reward_amount -> Nullable<Numeric>,
        create_at -> Timestamp,
        update_at -> Timestamp,
    }
}

diesel::table! {
    mission (mission_id) {
        mission_id -> Uuid,
        curriculum_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        create_at -> Timestamp,
        usr_id -> Uuid,
        mission_submit_form -> Text,
    }
}

diesel::table! {
    mission_submit (mission_id, usr_id) {
        mission_id -> Uuid,
        usr_id -> Uuid,
        description -> Text,
        #[max_length = 50]
        status -> Varchar,
        create_at -> Timestamp,
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
    use super::sql_types::ResourceType;

    reward_claim (id) {
        id -> Uuid,
        coin_network_id -> Uuid,
        reward_claim_status -> RewardClaimStatus,
        amount -> Numeric,
        user_id -> Uuid,
        #[max_length = 100]
        user_address -> Varchar,
        created_date -> Timestamp,
        updated_date -> Timestamp,
        resource_type -> ResourceType,
        resource_id -> Uuid,
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
diesel::joinable!(mission -> tb_ldm_usr (usr_id));
diesel::joinable!(mission_submit -> mission (mission_id));
diesel::joinable!(mission_submit -> tb_ldm_usr (usr_id));
diesel::joinable!(reward_claim_detail -> reward_claim (reward_claim_id));
diesel::joinable!(tb_ldm_usr_rgh -> tb_ldm_usr (id));

diesel::allow_tables_to_appear_in_same_query!(
    coin,
    coin_network,
    detailed_posting,
    mission,
    mission_submit,
    network,
    reward_claim,
    reward_claim_detail,
    tb_ldm_usr,
    tb_ldm_usr_rgh,
);

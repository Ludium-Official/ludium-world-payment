// @generated automatically by Diesel CLI.

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

diesel::joinable!(tb_ldm_usr_rgh -> tb_ldm_usr (id));

diesel::allow_tables_to_appear_in_same_query!(
    tb_ldm_usr,
    tb_ldm_usr_rgh,
);
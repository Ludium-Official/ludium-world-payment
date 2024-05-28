use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::adapter::output::persistence::db::schema::tb_ldm_usr;

#[derive(Clone, Debug, Queryable, Identifiable)]
#[diesel(table_name = tb_ldm_usr)]
pub struct User {
    pub id: Uuid,
    pub nick: String,
    pub self_intro: String,
    pub phn_nmb: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    id: String,
    nick: String,
    self_intro: String,
    phn_nmb: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id.to_string(),
            nick: user.nick,
            self_intro: user.self_intro,
            phn_nmb: user.phn_nmb,
        }
    }
}

#[derive(Insertable, Clone)]
#[diesel(table_name = tb_ldm_usr)]
pub struct NewUser {
    pub id: Uuid,
    pub nick: String,
    pub self_intro: String,
    pub phn_nmb: String,
}

#[derive(Deserialize, Clone)]
pub struct NewUserPayload {
    pub nick: String,
    pub self_intro: String,
    pub phn_nmb: String,
}
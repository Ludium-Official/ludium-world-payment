use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{adapter::output::persistence::db::{error::adapt_db_error, schema::tb_ldm_usr}, domain::model::{Error,Result}, port::output::{DbManager, UserRepository}};

// region:    --- User Types

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

// endregion: --- User Types

// region:    --- UserBmc (usecase)

pub struct UserBmc;

impl UserBmc {
    pub async fn insert(
        db_manager: &dyn DbManager,
        repo: &dyn UserRepository,
        new_user_payload: NewUserPayload,
    ) -> Result<User> {
        let conn = db_manager.get_connection().await?;
        repo.insert(conn, new_user_payload).await
    }

    pub async fn get(
        db_manager: &dyn DbManager,
        repo: &dyn UserRepository,
        user_id: Uuid,
    ) -> Result<User> {
        let conn = db_manager.get_connection().await?;
        repo.get(conn, user_id).await
    }

    pub async fn first_by_nick(
        db_manager: &dyn DbManager,
        repo: &dyn UserRepository,
        user_nick: String,
    ) -> Result<Option<User>> {
        let conn = db_manager.get_connection().await?;
        repo.first_by_nick(conn, user_nick).await
    }

    pub async fn list(db_manager: &dyn DbManager, repo: &dyn UserRepository) -> Result<Vec<User>> {
        let conn = db_manager.get_connection().await?;
        repo.list(conn).await
    }
}

// endregion:    --- UserBmc (usecase)

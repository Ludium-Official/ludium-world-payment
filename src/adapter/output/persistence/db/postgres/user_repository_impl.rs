use axum::async_trait;
use deadpool_diesel::postgres::Object;
use diesel::prelude::*;
use uuid::Uuid;
use crate::domain::model::user::{NewUser, NewUserPayload, User};
use crate::port::output::UserRepository;
use super::{Error, Result, adapt_db_error, tb_ldm_usr};

#[derive(Clone, Debug)]
pub struct PostgresUserRepository;

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn insert(&self, conn: Object, new_user_payload: NewUserPayload) -> Result<User> {
        let new_user = NewUser {
            id: Uuid::new_v4(),
            nick: new_user_payload.nick,
            self_intro: new_user_payload.self_intro,
            phn_nmb: new_user_payload.phn_nmb,
        };

        conn.interact(move |conn| {
            diesel::insert_into(tb_ldm_usr::table)
                .values(new_user)
                .get_result::<User>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn get(&self, conn: Object, user_id: Uuid) -> Result<User> {
        conn.interact(move |conn| {
            tb_ldm_usr::table
                .find(user_id)
                .get_result::<User>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::output::persistence::db::_dev_utils;
    use crate::port::output::{DbManager, UserRepository};
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_get_user() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let user_repo = PostgresUserRepository;

        let new_user_payload = NewUserPayload {
            nick: "user1".to_string(),
            self_intro: "I am user 1".to_string(),
            phn_nmb: "123456789".to_string(),
        };

        let new_user_payload2 = NewUserPayload {
            nick: "user2".to_string(),
            self_intro: "I am user 2".to_string(),
            phn_nmb: "987654321".to_string(),
        };

        let created_user = user_repo.insert(db_manager.get_connection().await?, new_user_payload.clone()).await?;
        let created_user2 = user_repo.insert(db_manager.get_connection().await?, new_user_payload2.clone()).await?;

        let user = user_repo.get(db_manager.get_connection().await?, created_user.id).await?;
        assert_eq!(user.nick, new_user_payload.nick);
        assert_eq!(user.self_intro, new_user_payload.self_intro);
        assert_eq!(user.phn_nmb, new_user_payload.phn_nmb);

        let user2 = user_repo.get(db_manager.get_connection().await?, created_user2.id).await?;
        assert_eq!(user2.nick, new_user_payload2.nick);
        assert_eq!(user2.self_intro, new_user_payload2.self_intro);
        assert_eq!(user2.phn_nmb, new_user_payload2.phn_nmb);

        Ok(())
    }
}
use axum::async_trait;
use deadpool_diesel::postgres::Object;
use diesel::prelude::*;
use uuid::Uuid;
use crate::adapter::output::persistence::db::error::{Error, Result};
use crate::domain::model::user::{NewUser, NewUserPayload, User};
use crate::port::output::UserRepository;

use super::{adapt_db_error, tb_ldm_usr};

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

        conn.interact(|conn| {
            diesel::insert_into(tb_ldm_usr::table)
                .values(new_user)
                .get_result::<User>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn list(&self, conn: Object) -> Result<Vec<User>> {
        conn.interact(|conn| {
            tb_ldm_usr::table.load::<User>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }
}


// region: --- user repository tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::output::persistence::db::_dev_utils;
    use crate::port::output::{DbManager, UserRepository};
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_list_users() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let user_repo = PostgresUserRepository;

        let initial_users = user_repo.list(db_manager.get_connection().await?).await?;
        let initial_count = initial_users.len();

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

        user_repo.insert(db_manager.get_connection().await?, new_user_payload.clone()).await?;
        user_repo.insert(db_manager.get_connection().await?, new_user_payload2.clone()).await?;

        let users = user_repo.list(db_manager.get_connection().await?).await?;
        assert_eq!(users.len(), initial_count + 2);

        Ok(())
    }
}

// endregion: --- user repository tests
use axum::async_trait;
use deadpool_diesel::postgres::{Manager, Object, Pool};
use diesel::prelude::*;
use uuid::Uuid;
use crate::domain::model::{Error, Result};
use crate::domain::model::user::{NewUser, NewUserPayload, User};
use crate::port::output::{UserRepository, DbManager};

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

    async fn first_by_nick(&self, conn: Object, user_nick: String) -> Result<Option<User>> {
        conn.interact(move |conn| {
            tb_ldm_usr::table
                .filter(tb_ldm_usr::nick.eq(user_nick))
                .first::<User>(conn)
                .optional()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::output::persistence::db::_dev_utils;
    use crate::domain::model::user::{NewUser, User};
    use crate::port::output::UserRepository;
    use uuid::Uuid;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_insert_and_get_user() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let user_repo = PostgresUserRepository;

        let new_user_payload = NewUserPayload {
            nick: "test_nick".to_string(),
            self_intro: "Hello, I am a test user".to_string(),
            phn_nmb: "123456789".to_string(),
        };

        let inserted_user = user_repo.insert(db_manager.get_connection().await?, new_user_payload.clone()).await?;
        assert_eq!(inserted_user.nick, new_user_payload.nick);

        let fetched_user = user_repo.get(db_manager.get_connection().await?, inserted_user.id).await?;
        assert_eq!(fetched_user.id, inserted_user.id);
        assert_eq!(fetched_user.nick, inserted_user.nick);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_first_by_nick() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let user_repo = PostgresUserRepository;

        let new_user_payload = NewUserPayload {
            nick: "unique_nick".to_string(),
            self_intro: "Hello, I am a test user".to_string(),
            phn_nmb: "123456789".to_string(),
        };

        let inserted_user = user_repo.insert(db_manager.get_connection().await?, new_user_payload.clone()).await?;
        assert_eq!(inserted_user.nick, new_user_payload.nick);

        let fetched_user = user_repo.first_by_nick(db_manager.get_connection().await?, new_user_payload.nick.clone()).await?;
        assert!(fetched_user.is_some());
        assert_eq!(fetched_user.unwrap().nick, new_user_payload.nick);

        Ok(())
    }

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

use axum::async_trait;
use deadpool_diesel::postgres::Object;
use uuid::Uuid;
use crate::adapter::output::persistence::db::error::Result;
use crate::domain::model::user::{NewUserPayload, User};

#[async_trait]
pub trait UserRepository {
    //! test only 
    async fn insert(&self, conn: Object, new_user_payload: NewUserPayload) -> Result<User>;

    // usecase
    async fn get(&self, conn: Object, user_id: Uuid) -> Result<User>;
    async fn first_by_nick(&self, conn: Object, user_nick: String) -> Result<Option<User>>;
    async fn list(&self, conn: Object) -> Result<Vec<User>>;
}

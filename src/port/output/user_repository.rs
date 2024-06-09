use axum::async_trait;
use deadpool_diesel::postgres::Object;
use uuid::Uuid;
use crate::adapter::output::persistence::db::error::Result;
use crate::domain::model::user::{NewUserPayload, User};

#[async_trait]
pub trait UserRepository {
    #[allow(unused)]
    async fn insert(&self, conn: Object, new_user_payload: NewUserPayload) -> Result<User>;
    
    async fn get(&self, conn: Object, user_id: Uuid) -> Result<User>;
}

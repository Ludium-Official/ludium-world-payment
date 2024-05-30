use axum::async_trait;
use deadpool_diesel::postgres::Object;
use crate::adapter::output::persistence::db::error::Result;
use crate::domain::model::user::{NewUserPayload, User};

#[async_trait]
pub trait UserRepository {
    //! test only 
    async fn insert(&self, conn: Object, new_user_payload: NewUserPayload) -> Result<User>;

    // usecase
    async fn list(&self, conn: Object) -> Result<Vec<User>>;
}

use axum::async_trait;
use deadpool_diesel::postgres::Object;
use uuid::Uuid;
use crate::adapter::output::persistence::db::error::Result;
use crate::domain::model::mission_submit::MissionSubmit;

#[async_trait]
pub trait MissionSubmitRepository {
    async fn get(&self, conn: Object, user_id: Uuid, mission_id: Uuid) -> Result<MissionSubmit>;
}
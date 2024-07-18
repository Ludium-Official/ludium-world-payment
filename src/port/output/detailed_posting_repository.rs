use axum::async_trait;
use deadpool_diesel::postgres::Object;
use uuid::Uuid;
use crate::{adapter::output::persistence::db::error::Result, domain::model::detailed_posting::DetailedPosting};

#[async_trait]
pub trait DetailedPostingRepository {
    async fn get(&self, conn: Object, detail_id: Uuid) -> Result<DetailedPosting>;
}
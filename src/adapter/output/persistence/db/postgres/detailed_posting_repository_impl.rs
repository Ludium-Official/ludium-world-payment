use axum::async_trait;
use deadpool_diesel::postgres::Object;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{domain::model::detailed_posting::DetailedPosting, port::output::detailed_posting_repository::DetailedPostingRepository};

use super::{Error, Result, adapt_db_error, detailed_posting};

#[derive(Clone, Debug)]
pub struct PostgresDetailedPostingRepository;

#[async_trait]
impl DetailedPostingRepository for PostgresDetailedPostingRepository {
    async fn get(&self, conn: Object, detail_id: Uuid) -> Result<DetailedPosting>{
        conn.interact(move |conn| {
            detailed_posting::table
                .filter(detailed_posting::detail_id.eq(detail_id))
                .get_result::<DetailedPosting>(conn)
        })
        .await?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

}
use axum::async_trait;
use deadpool_diesel::postgres::Object;
use diesel::prelude::*;
use uuid::Uuid;
use crate::domain::model::mission_submit::MissionSubmit;
use crate::port::output::mission_submit_repository::MissionSubmitRepository;

use super::{Error, Result, adapt_db_error, mission_submit};

#[derive(Clone, Debug)]
pub struct PostgresMissionSubmitRepository;

#[async_trait]
impl MissionSubmitRepository for PostgresMissionSubmitRepository {
    async fn get(&self, conn: Object, user_id: Uuid, mission_id: Uuid) -> Result<MissionSubmit> {
        conn.interact(move |conn| {
            mission_submit::table
                .filter(mission_submit::usr_id.eq(user_id))
                .filter(mission_submit::mission_id.eq(mission_id))
                .get_result::<MissionSubmit>(conn)
        })
        .await?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }
}
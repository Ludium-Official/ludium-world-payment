use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

use crate::adapter::output::persistence::db::schema::mission;

#[derive(Clone, Debug, Queryable)]
#[diesel(table_name = mission)]
pub struct Mission {
    pub mission_id: Uuid,
    pub usr_id: Uuid,
    pub curriculum_id: Uuid,
    pub title: String,
    pub description: String,
    pub create_at: NaiveDateTime,
    pub mission_submit_form: String,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = mission)]
pub struct NewMission {
    pub mission_id: Uuid,
    pub usr_id: Uuid,
    pub curriculum_id: Uuid,
    pub title: String,
    pub description: String,
    pub create_at: NaiveDateTime,
    pub mission_submit_form: String,
}

#[derive(Deserialize, Clone)]
pub struct NewMissionPayload {
    pub curriculum_id: Uuid,
    pub title: String,
    pub description: String,
    pub usr_id: Uuid,
    pub mission_submit_form: String,
}

#[derive(Serialize, ToSchema)]
pub struct MissionResponse {
    pub mission_id: String,
    pub curriculum_id: String,
    pub title: String,
    pub description: String,
    pub create_at: i64,
    pub usr_id: String,
    pub mission_submit_form: String,
}

impl From<Mission> for MissionResponse {
    fn from(mission: Mission) -> Self {
        MissionResponse {
            mission_id: mission.mission_id.to_string(),
            curriculum_id: mission.curriculum_id.to_string(),
            title: mission.title,
            description: mission.description,
            create_at: mission.create_at.and_utc().timestamp(),
            usr_id: mission.usr_id.to_string(),
            mission_submit_form: mission.mission_submit_form,
        }
    }
}

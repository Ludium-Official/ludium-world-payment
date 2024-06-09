use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

use crate::{adapter::output::persistence::db::schema::mission_submit, domain::model::{mission::Mission, user::User}};

#[derive(Clone, Debug, Queryable, Associations)]
#[diesel(table_name = mission_submit)]
#[diesel(belongs_to(Mission, foreign_key = mission_id))]
#[diesel(belongs_to(User, foreign_key = usr_id))]
pub struct MissionSubmit {
    pub mission_id: Uuid,
    pub usr_id: Uuid,
    pub description: String,
    pub status: String, // "APPROVE", "SUBMIT"
    pub create_at: NaiveDateTime,
}

impl MissionSubmit {
    pub fn is_approved(&self) -> bool {
        self.status == "APPROVE"
    }
}


#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = mission_submit)]
pub struct NewMissionSubmit {
    pub mission_id: Uuid,
    pub usr_id: Uuid,
    pub description: String,
    pub status: String,
    pub create_at: NaiveDateTime,
}

#[derive(Deserialize, Clone)]
pub struct NewMissionSubmitPayload {
    pub mission_id: Uuid,
    pub usr_id: Uuid,
    pub description: String,
    pub status: String,
}

#[derive(Serialize, ToSchema)]
pub struct MissionSubmitResponse {
    pub mission_id: String,
    pub usr_id: String,
    pub description: String,
    pub status: String,
    pub create_at: i64,
}

impl From<MissionSubmit> for MissionSubmitResponse {
    fn from(mission_submit: MissionSubmit) -> Self {
        MissionSubmitResponse {
            mission_id: mission_submit.mission_id.to_string(),
            usr_id: mission_submit.usr_id.to_string(),
            description: mission_submit.description,
            status: mission_submit.status,
            create_at: mission_submit.create_at.and_utc().timestamp(),
        }
    }
}

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    user_id: String,
    adm: bool,
    prv: bool,
    crt: bool,
}

impl UserInfo {
    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn adm(&self) -> bool {
        self.adm
    }
}

#[derive(Clone, Debug)]
pub struct Ctx {
    user_info: UserInfo,
    access_token: String,
    ggl_id: String,
}

impl Ctx {
    pub fn new(user_info: UserInfo, access_token: String, ggl_id: String) -> Self {
        Self {
            user_info,
            access_token,
            ggl_id,
        }
    }

    pub fn user_info(&self) -> &UserInfo {
        &self.user_info
    }

    pub fn is_admin(&self) -> bool {
        self.user_info.adm()
    }
}

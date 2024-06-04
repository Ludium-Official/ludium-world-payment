use serde::Deserialize;


#[allow(unused)] // authentication code will be added later
#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    id: String,
    adm: bool,
    prv: bool,
    crt: bool,
}

impl UserInfo {
    pub fn user_id(&self) -> &str {
        &self.id
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

    pub fn is_authenticated(&self) -> bool {
        !self.access_token.is_empty() && !self.ggl_id.is_empty()
    }
}

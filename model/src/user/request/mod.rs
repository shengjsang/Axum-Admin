use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct CreateReq {
    pub username: String,
    pub password: String,
    pub email: String,
    pub phone: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginReq {
    pub account: String,
    pub password: String,
}

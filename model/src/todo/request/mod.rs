use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct CreateReq {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct TodoIDReq {
    pub id: i32,
}

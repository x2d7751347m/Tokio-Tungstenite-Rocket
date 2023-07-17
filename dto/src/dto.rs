use entity::*;
use rocket::{serde::{Deserialize, Serialize}, form::FromForm};
use rocket_okapi::okapi::schemars::{self, JsonSchema};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct PostsDto {
    pub page: u64,
    pub posts_per_page: u64,
    pub num_pages: u64,
    pub posts: Vec<post::Model>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct UserPost {
    pub username: String,
    pub email: String,
    pub password: String,
    pub nickname: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct EmailPost {
    pub email: String,
    pub user_id: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct UsersDto {
    pub page: u64,
    pub users_per_page: u64,
    pub num_pages: u64,
    pub users: Vec<user::Model>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct UserPatch {
    pub username: Option<String>,
    pub password: Option<String>,
    pub nickname: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResMe {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, JsonSchema, FromForm, Clone, Debug, PartialEq, Eq)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignUp {
    pub username: String,
    pub email: String,
    pub password: String,
    pub nickname: String,
}

#[derive(Serialize, Deserialize, JsonSchema, FromForm, Clone, Debug, PartialEq, Eq)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignIn {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResSignIn {
    pub token: String,
}
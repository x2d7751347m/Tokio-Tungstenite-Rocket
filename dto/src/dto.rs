use entity::*;
use rocket::{serde::{Deserialize, Serialize}, form::FromForm};
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use validator::Validate;

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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct EmailPost {
    #[validate(email)]
    pub email: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct EmailPatch {
    #[validate(email)]
    pub email: Option<String>,
    pub user_id: Option<i64>,
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
pub struct EmailsWithPageDto {
    pub page: u64,
    pub emails_per_page: u64,
    pub num_pages: u64,
    pub emails: Vec<email::Model>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct EmailsDto {
    pub emails: Vec<email::Model>,
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

#[derive(Serialize, Deserialize, JsonSchema, FromForm, Clone, Debug, PartialEq, Eq, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignUp {
    pub username: String,
    #[validate(email)]
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
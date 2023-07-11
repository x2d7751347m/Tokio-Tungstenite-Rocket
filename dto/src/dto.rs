use entity::*;
use rocket::serde::{Deserialize, Serialize};
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
    pub id: i32,
    pub email: String,
    pub password: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

use std::env;
use std::time::SystemTime;
use config::app_config::AppConfig;

use ::dto::dto::*;
use rocket::serde::json::Json;
use service::HttpAuth;
use service::{Mutation, Query};

use sea_orm_rocket::Connection;

use rocket_okapi::okapi::openapi3::OpenApi;

use crate::error;
use crate::okapi::{DataResult, R};
use crate::pool;
use pool::Db;

pub use entity::post;
pub use entity::post::Entity as Post;

use rocket_okapi::settings::OpenApiSettings;

use rocket_okapi::{openapi, openapi_get_routes_spec};

const DEFAULT_POSTS_PER_PAGE: u64 = 5;

use crate::{ErrorResponse};
use super::{Response, SuccessResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
extern crate entity;
pub use entity::user;
pub use entity::user::Entity as User;
use jsonwebtoken::{encode, EncodingKey, Header};
use migration::sea_orm::{DatabaseConnection, self};
use rocket::{
    http::Status,
    serde::{Deserialize, Serialize},
    State,
};
use sea_orm::*;
use service::{Claims, AuthenticatedUser};

use rocket_okapi::okapi::schemars::{self, JsonSchema};

#[openapi(tag = "USER")]
#[post("/sign-in", data = "<req_sign_in>")]
pub async fn sign_in(
    conn: Connection<'_, Db>,
    req_sign_in: DataResult<'_, ReqSignIn>,
) -> R<ResSignIn> {
    let db = conn.into_inner();

    let form = req_sign_in?.into_inner();
    
    let user: Option<user::Model> = 
    if form.username_or_email.contains("@") {
    Query::find_user_by_email(db, form.username_or_email)
        .await
        .expect("could not find user")
    } else {
        Query::find_user_by_username(db, form.username_or_email).await.expect("could not find user")
    };

    if !verify(&form.password, &user.clone().unwrap().password).unwrap() {
        let m = error::Error {
            err: "Invalid password".to_string(),
            msg: Some("Invalid password".to_string()),
            http_status_code: 400,
        };
        return Err(m);
    }

    let claims = Claims {
        sub: user.unwrap().id,
        role: "user".to_string(),
        exp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 4 * 60 * 60,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(AppConfig::default().jwt_secret.as_bytes()),
    )
    .unwrap();

    Ok(Json(ResSignIn { token: token }))
}
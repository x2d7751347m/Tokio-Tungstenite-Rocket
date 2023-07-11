use std::time::SystemTime;

use ::dto::dto::*;
use rocket::serde::json::Json;
use service::{Mutation, Query};

use sea_orm_rocket::Connection;

use rocket_okapi::okapi::openapi3::OpenApi;

use crate::error;
use crate::pool;
use pool::Db;

pub use entity::post;
pub use entity::post::Entity as Post;

use rocket_okapi::settings::OpenApiSettings;

use rocket_okapi::{openapi, openapi_get_routes_spec};

const DEFAULT_POSTS_PER_PAGE: u64 = 5;

pub type R<T> = std::result::Result<rocket::serde::json::Json<T>, error::Error>;
pub type DataResult<'a, T> =
    std::result::Result<rocket::serde::json::Json<T>, rocket::serde::json::Error<'a>>;


/**
 * 📕 BookStore
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/bookstore
 */
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
use service::{Claims, AuthenticatedUser, AppConfig};

use rocket_okapi::okapi::schemars::{self, JsonSchema};

#[openapi(tag = "USER")]
#[post("/sign-in", data = "<req_sign_in>")]
pub async fn sign_in(
    conn: Connection<'_, Db>,
    req_sign_in: DataResult<'_, ReqSignIn>,
) -> R<ResSignIn> {
    let db = conn.into_inner();

    let form = req_sign_in?.into_inner();
    
    let user: Option<user::Model> = Query::find_user_by_email(db, form.email)
        .await
        .expect("could not find user");

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
        &EncodingKey::from_secret("config.jwt_secret.as_bytes()".as_bytes()),
    )
    .unwrap();

    Ok(Json(ResSignIn { token: token }))
}


#[openapi(tag = "USER")]
#[post("/user", data = "<req_sign_up>")]
pub async fn sign_up(
    conn: Connection<'_, Db>,
    req_sign_up: DataResult<'_, ReqSignUp>,
) -> R<Option<String>> {
    
    //send email for ownership
    // Mail::send().now_or_never();
    let db = conn.into_inner();
    let form = req_sign_up?.into_inner();
    let cmd = Mutation::create_user(db, form);
    match cmd.await {
        Ok(_) => Ok(Json(Some("User successfully added.".to_string()))),
        Err(e) => {
            let m = error::Error {
                err: "Could not insert user".to_string(),
                msg: Some(e.to_string()),
                http_status_code: 400,
            };
            Err(m)
        }
    }
}

#[post("/sign-up", data = "<req_sign_up>")]
pub async fn sign_up1(
    db: &State<DatabaseConnection>,
    req_sign_up: Json<ReqSignUp>,
) -> Response<String> {
    let db = db as &DatabaseConnection;

    if User::find()
        .filter(user::Column::Email.eq(&req_sign_up.email))
        .one(db)
        .await?
        .is_some()
    {
        return Err(ErrorResponse((
            Status::UnprocessableEntity,
            "An account exists with that email address.".to_string(),
        )));
    }

    User::insert(user::ActiveModel {
        email: Set(req_sign_up.email.to_owned()),
        password: Set(hash(&req_sign_up.password, DEFAULT_COST).unwrap()),
        firstname: Set(req_sign_up.firstname.to_owned()),
        lastname: Set(req_sign_up.lastname.to_owned()),
        ..Default::default()
    })
    .exec(db)
    .await?;

    Ok(SuccessResponse((
        Status::Created,
        "Account created!".to_string(),
    )))
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResMe {
    id: i32,
    email: String,
    firstname: Option<String>,
    lastname: Option<String>,
}

#[get("/me")]
pub async fn me(db: &State<DatabaseConnection>, user: AuthenticatedUser) -> Response<Json<ResMe>> {
    let db = db as &DatabaseConnection;

    let u: user::Model = User::find_by_id(user.id).one(db).await?.unwrap();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResMe {
            id: u.id,
            email: u.email,
            firstname: u.firstname,
            lastname: u.lastname,
        }),
    )))
}

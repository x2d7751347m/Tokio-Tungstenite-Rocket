use std::time::SystemTime;

use ::dto::dto::*;
use rocket::serde::json::Json;
use service::HttpAuth;
use service::{Mutation, Query};

use sea_orm_rocket::Connection;

use rocket_okapi::okapi::openapi3::OpenApi;

use crate::error;
use crate::okapi_example::{DataResult, R};
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
use service::{Claims, AuthenticatedUser, AppConfig};

use rocket_okapi::okapi::schemars::{self, JsonSchema};

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
    let user = Mutation::create_user(db, form.clone()).await.unwrap();
    let cmd = Mutation::create_email(db, EmailPost { email: (form.email.to_owned()), user_id: (user.id.unwrap()) });
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

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResMe {
    id: i32,
    nickname: String,
}


#[openapi(tag = "USER")]
#[get("/me")]
pub async fn me(conn: Connection<'_, Db>, token: HttpAuth) -> R<ResMe> {
    let db = conn.into_inner();

    let u: user::Model = User::find_by_id(token.id).one(db).await.unwrap().unwrap();

    Ok(Json(ResMe {
        id: u.id,
        nickname: u.nickname,
    }))
}

/// # Update a user
#[openapi(tag = "USER")]
#[post("/user/<id>", data = "<user_data>")]
pub async fn update(
    conn: Connection<'_, Db>,
    id: i32,
    user_data: DataResult<'_, UserPatch>,
) -> R<Option<String>> {
    let db = conn.into_inner();

    let form = user_data?.into_inner();

    let cmd = Mutation::update_user_by_id(db, id, form);
    match cmd.await {
        Ok(_) => Ok(Json(Some("Post successfully updated.".to_string()))),
        Err(e) => {
            let m = error::Error {
                err: "Could not update user".to_string(),
                msg: Some(e.to_string()),
                http_status_code: 400,
            };
            Err(m)
        }
    }
}

/// # Get user list
#[openapi(tag = "USER")]
#[get("/user?<page>&<users_per_page>")]
pub async fn list(
    conn: Connection<'_, Db>,
    page: Option<u64>,
    users_per_page: Option<u64>,
) -> R<UsersDto> {
    let db = conn.into_inner();

    // Set page number and items per page
    let page = page.unwrap_or(1);
    let users_per_page = users_per_page.unwrap_or(DEFAULT_POSTS_PER_PAGE);
    if page == 0 {
        let m = error::Error {
            err: "error getting users".to_string(),
            msg: Some("'page' param cannot be zero".to_string()),
            http_status_code: 400,
        };
        return Err(m);
    }

    let (users, num_pages) = Query::find_users_in_page(db, page, users_per_page)
        .await
        .expect("Cannot find users in page");

    Ok(Json(UsersDto {
        page,
        users_per_page,
        num_pages,
        users,
    }))
}

/// # get user by Id
#[openapi(tag = "USER")]
#[get("/user/<id>")]
pub async fn get_by_id(conn: Connection<'_, Db>, id: i32) -> R<Option<user::Model>> {
    let db = conn.into_inner();

    let user: Option<user::Model> = Query::find_user_by_id(db, id)
        .await
        .expect("could not find user");
    Ok(Json(user))
}

/// # delete user by Id
#[openapi(tag = "USER")]
#[delete("/user/<id>")]
pub async fn delete(conn: Connection<'_, Db>, id: i32) -> R<Option<String>> {
    let db = conn.into_inner();

    let cmd = Mutation::delete_user(db, id);
    match cmd.await {
        Ok(_) => Ok(Json(Some("User successfully deleted.".to_string()))),
        Err(e) => {
            let m = error::Error {
                err: "Error deleting user".to_string(),
                msg: Some(e.to_string()),
                http_status_code: 400,
            };
            Err(m)
        }
    }
}

/// # delete all users
#[openapi(tag = "USER")]
#[delete("/user")]
pub async fn destroy(conn: Connection<'_, Db>) -> R<Option<String>> {
    let db = conn.into_inner();

    let cmd = Mutation::delete_all_users(db);

    match cmd.await {
        Ok(_) => Ok(Json(Some(
            "All Users were successfully deleted.".to_string(),
        ))),
        Err(e) => {
            let m = error::Error {
                err: "Error deleting all users at once".to_string(),
                msg: Some(e.to_string()),
                http_status_code: 400,
            };
            Err(m)
        }
    }
}
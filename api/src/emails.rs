use migration::sea_orm;
/**
 * 📕 EmailStore
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/emailstore
 */
use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use sea_orm::{prelude::DateTimeUtc, *};
use std::time::SystemTime;

use super::{ErrorResponse, Response, SuccessResponse};
extern crate entity;
pub use entity::email;
pub use entity::email::Entity as Email;
use service::{AuthenticatedUser, HttpAuth};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResEmail {
    pub id: i32,
    pub email: String,
}

impl From<&email::Model> for ResEmail {
    fn from(b: &email::Model) -> Self {
        Self {
            id: b.id,
            email: b.email.to_owned(),
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResEmailList {
    pub total: usize,
    pub emails: Vec<ResEmail>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqEmail {
    email: String,
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    token: HttpAuth,
) -> Response<Json<ResEmailList>> {
    let db = db as &DatabaseConnection;

    let emails = Email::find()
        .order_by_desc(email::Column::UpdatedAt)
        .all(db)
        .await?
        .iter()
        .map(ResEmail::from)
        .collect::<Vec<_>>();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResEmailList {
            total: emails.len(),
            emails,
        }),
    )))
}

#[post("/", data = "<req_email>")]
pub async fn create(
    db: &State<DatabaseConnection>,
    token: HttpAuth,
    req_email: Json<ReqEmail>,
) -> Response<Json<ResEmail>> {
    let db = db as &DatabaseConnection;

    let email = email::ActiveModel {
        user_id: Set(token.id),
        email: Set(req_email.email.to_owned()),
        ..Default::default()
    };

    let email = email.insert(db).await?;

    Ok(SuccessResponse((
        Status::Created,
        Json(ResEmail::from(&email)),
    )))
}

#[get("/<id>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    token: HttpAuth,
    id: i32,
) -> Response<Json<ResEmail>> {
    let db = db as &DatabaseConnection;

    let email = Email::find_by_id(id).one(db).await?;

    let email = match email {
        Some(b) => b,
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "No email found with the specified ID.".to_string(),
            )))
        }
    };

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResEmail::from(&email)),
    )))
}

#[put("/<id>", data = "<req_email>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    token: HttpAuth,
    id: i32,
    req_email: Json<ReqEmail>,
) -> Response<Json<ResEmail>> {
    let db = db as &DatabaseConnection;

    let mut email: email::ActiveModel = match Email::find_by_id(id).one(db).await? {
        Some(b) => b.into(),
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "No email found with the specified ID.".to_string(),
            )))
        }
    };

    email.email = Set(req_email.email.to_owned());

    email.updated_at = Set(DateTimeUtc::from(SystemTime::now()));

    let email = email.update(db).await?;

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResEmail::from(&email)),
    )))
}

#[delete("/<id>")]
pub async fn delete(
    db: &State<DatabaseConnection>,
    token: HttpAuth,
    id: i32,
) -> Response<String> {
    let db = db as &DatabaseConnection;

    let email = match Email::find_by_id(id).one(db).await? {
        Some(b) => b,
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "No email found with the specified ID.".to_string(),
            )))
        }
    };

    email.delete(db).await?;

    Ok(SuccessResponse((Status::Ok, "Email deleted.".to_string())))
}

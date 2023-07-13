// use migration::sea_orm;
//
// use rocket::{
//     http::Status,
//     serde::{json::Json, Deserialize, Serialize},
//     State,
// };
// use rocket_okapi::openapi;
// use sea_orm::{prelude::DateTimeUtc, *};
// use sea_orm_rocket::Connection;
// use std::time::SystemTime;

// use crate::{okapi_pararium::R, error::Error, pool::Db};

// use super::{ErrorResponse, Response, SuccessResponse};
// extern crate entity;
// pub use entity::email;
// pub use entity::email::Entity as Email;
// use service::{AuthenticatedUser, HttpAuth, Mutation};

// use crate::error;

// #[derive(Serialize)]
// #[serde(crate = "rocket::serde")]
// pub struct ResEmail {
//     pub id: i32,
//     pub email: String,
// }

// impl From<&email::Model> for ResEmail {
//     fn from(b: &email::Model) -> Self {
//         Self {
//             id: b.id,
//             email: b.email.to_owned(),
//         }
//     }
// }

// #[derive(Serialize)]
// #[serde(crate = "rocket::serde")]
// pub struct ResEmailList {
//     pub total: usize,
//     pub emails: Vec<ResEmail>,
// }

// #[derive(Deserialize)]
// #[serde(crate = "rocket::serde")]
// pub struct ReqEmail {
//     email: String,
// }

// #[openapi(tag = "EMAIL")]
// #[get("/email")]
// pub async fn index(
//     conn: Connection<'_, Db>,
//     token: HttpAuth,
// ) -> R<ResEmailList> {
//     let db = conn.into_inner();

//     let emails = Email::find()
//         .order_by_desc(email::Column::UpdatedAt)
//         .all(db)
//         .await?
//         .iter()
//         .map(ResEmail::from)
//         .collect::<Vec<_>>();

//     Ok(Json(ResEmailList {
//         total: emails.len(),
//             emails,
//     }))
// }

// #[openapi(tag = "EMAIL")]
// #[post("/email", data = "<req_email>")]
// pub async fn create(
//     conn: Connection<'_, Db>,
//     token: HttpAuth,
//     req_email: Json<ReqEmail>,
// ) -> R<ResEmail> {
//     let db = conn.into_inner();

//     let email = email::ActiveModel {
//         user_id: Set(token.id),
//         email: Set(req_email.email.to_owned()),
//         ..Default::default()
//     };

//     let email = email.insert(db).await?;

//     let cmd = Mutation::create_post(db, form);
//     match cmd.await {
//         Ok(_) => Ok(Json(Some("Post successfully added.".to_string()))),
//         Err(e) => {
//             let m = error::Error {
//                 err: "Could not insert post".to_string(),
//                 msg: Some(e.to_string()),
//                 http_status_code: 400,
//             };
//             Err(m)
//         }
//     }

//     Ok(SuccessResponse((
//         Status::Created,
//         Json(ResEmail::from(&email)),
//     )))
// }

// #[openapi(tag = "EMAIL")]
// #[get("/email/<id>")]
// pub async fn show(
//     conn: Connection<'_, Db>,
//     token: HttpAuth,
//     id: i32,
// ) -> R<ResEmail> {
//     let db = conn.into_inner();

//     let email = Email::find_by_id(id).one(db).await?;

//     let email = match email {
//         Some(b) => b,
//         None => {
//             return Err(Error((
//                 Status::NotFound,
//                 "No email found with the specified ID.".to_string(),
//             )))
//         }
//     };

//     Ok(SuccessResponse((
//         Status::Ok,
//         Json(ResEmail::from(&email)),
//     )))
// }

// #[openapi(tag = "EMAIL")]
// #[put("/email/<id>", data = "<req_email>")]
// pub async fn update(
//     conn: Connection<'_, Db>,
//     token: HttpAuth,
//     id: i32,
//     req_email: Json<ReqEmail>,
// ) -> R<ResEmail> {
//     let db = conn.into_inner();

//     let mut email: email::ActiveModel = match Email::find_by_id(id).one(db).await? {
//         Some(b) => b.into(),
//         None => {
//             return Err(ErrorResponse((
//                 Status::NotFound,
//                 "No email found with the specified ID.".to_string(),
//             )))
//         }
//     };

//     email.email = Set(req_email.email.to_owned());

//     email.updated_at = Set(DateTimeUtc::from(SystemTime::now()));

//     let email = email.update(db).await?;

//     Ok(SuccessResponse((
//         Status::Ok,
//         Json(ResEmail::from(&email)),
//     )))
// }

// #[openapi(tag = "EMAIL")]
// #[delete("/email/<id>")]
// pub async fn delete(
//     conn: Connection<'_, Db>,
//     token: HttpAuth,
//     id: i32,
// ) -> R<String> {
//     let db = conn.into_inner();

//     let email = match Email::find_by_id(id).one(db).await? {
//         Some(b) => b,
//         None => {
//             let m = error::Error {
//                 err: "Error finding email".to_string(),
//                 msg: Some(e.to_string()),
//                 http_status_code: 400,
//             };
//             Err(m)
//         }
//     };

//     email.delete(db).await?;

//     Ok(SuccessResponse((Status::Ok, "Email deleted.".to_string())))
// }

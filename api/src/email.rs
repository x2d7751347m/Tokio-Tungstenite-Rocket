use dto::dto::{EmailPost, EmailPatch, EmailsDto, EmailsWithPageDto};
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
use rocket_okapi::openapi;
use sea_orm::{prelude::DateTimeUtc, *};
use sea_orm_rocket::Connection;
use std::time::SystemTime;

use crate::{okapi::{R, DataResult}, error::Error, pool::Db};

use super::{ErrorResponse, Response, SuccessResponse};
extern crate entity;
pub use entity::email;
pub use entity::email::Entity as Email;
use service::{AuthenticatedUser, HttpAuth, Mutation, Query};

// A trait that the Validate derive will impl
use validator::{Validate, ValidationError};
const DEFAULT_EMAILS_PER_PAGE: u64 = 5;

use crate::error;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResEmail {
    pub id: i64,
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

/// # Add an email
#[openapi(tag = "EMAIL")]
#[post("/email", data = "<req_email>")]
pub async fn create(
    conn: Connection<'_, Db>,
    token: HttpAuth,
    req_email: DataResult<'_, EmailPost>,
) -> R<Option<String>> {
    
    //send email for ownership
    // Mail::send().now_or_never();
    let db = conn.into_inner();
    let form = req_email?.into_inner();

    match form.validate() {
        Ok(_) => (),
        Err(e) => {
            let m = error::Error {
                err: "Malformed email".to_string(),
                msg: Some(e.to_string()),
                http_status_code: 400,
            };
            return Err(m)
        }
      };

    let find = Query::find_email_by_email(db, form.clone().email);
    let _ = match find.await {
        Ok(None) => {
            Ok(())
        },
        Ok(Some(_a)) => {let m = error::Error {
                err: "This email is already in use.".to_string(),
                msg: Some("This email is already in use.".to_string()),
                http_status_code: 409,
            };
            return Err(m);
        }
        Err(e) => {
            Ok(())
        }
    };

    let cmd = Mutation::create_email(db, form, token.id);
    match cmd.await {
        Ok(_) => Ok(Json(Some("Email successfully added.".to_string()))),
        Err(e) => {
            let m = error::Error {
                err: "Could not insert email".to_string(),
                msg: Some(e.to_string()),
                http_status_code: 400,
            };
            Err(m)
        }
    }
}


/// # Get my emails
#[openapi(tag = "EMAIL")]
#[get("/email/me")]
pub async fn me(conn: Connection<'_, Db>, 
token: HttpAuth) -> R<EmailsDto> {
    let db = conn.into_inner();

    let user = Query::find_user_by_id(db, token.id)
        .await
        .expect("could not find user");

    let emails: Vec<email::Model> = user.unwrap().find_related(Email).all(db).await.unwrap();
    Ok(Json(EmailsDto {
        emails,
    }))

}

/// # Update an email
#[openapi(tag = "EMAIL")]
#[patch("/email/<id>", data = "<email_data>")]
pub async fn update(
    conn: Connection<'_, Db>,
    id: i64,
    token: HttpAuth,
    email_data: DataResult<'_, EmailPatch>,
) -> R<Option<String>> {
    let db = conn.into_inner();

    let form = email_data?.into_inner();

    if form.clone().email.is_some(){
        match form.validate(){
            Ok(_) => (),
          Err(e) => {
            let m = error::Error {
                err: "Malformed email".to_string(),
                msg: Some(e.to_string()),
                http_status_code: 400,
            };
            return Err(m)}
        }
    let find = Query::find_email_by_email(db, form.clone().email.unwrap());
    let _ = match find.await {
        Ok(None) => {
            Ok(())
        },
        Ok(Some(_a)) => {let m = error::Error {
                err: "This email is already in use.".to_string(),
                msg: Some("This email is already in use.".to_string()),
                http_status_code: 409,
            };
            return Err(m);
        }
        Err(e) =>{
            let m = error::Error {
                err: "Could not find email".to_string(),
                msg: Some(e.to_string()),
                http_status_code: 500,
            };
            Err(m)
        }
    };
    }

    let _email = if Query::find_email_by_id(db, id).await.unwrap().unwrap().user_id != token.id {
        let m = error::Error {
            err: "Invalid authentication".to_string(),
            msg: Some("Invalid authentication".to_string()),
            http_status_code: 401,
        };
        return Err(m)
    };

    let cmd = Mutation::update_email_by_id(db, id, form);
    match cmd.await {
        Ok(_) => Ok(Json(Some("Email successfully updated.".to_string()))),
        Err(e) => {
            let m = error::Error {
                err: "Could not update email".to_string(),
                msg: Some(e.to_string()),
                http_status_code: 400,
            };
            Err(m)
        }
    }
}

/// # Get my email list with page
#[openapi(tag = "EMAIL")]
#[get("/email?<page>&<emails_per_page>")]
pub async fn list(
    conn: Connection<'_, Db>,
    page: Option<u64>,
    token: HttpAuth,
    emails_per_page: Option<u64>,
) -> R<EmailsWithPageDto> {
    let db = conn.into_inner();

    // Set page number and items per page
    let page = page.unwrap_or(1);
    let emails_per_page = emails_per_page.unwrap_or(DEFAULT_EMAILS_PER_PAGE);
    if page == 0 {
        let m = error::Error {
            err: "error getting emails".to_string(),
            msg: Some("'page' param cannot be zero".to_string()),
            http_status_code: 400,
        };
        return Err(m);
    }

    let (emails, num_pages) = Query::find_emails_in_page(db, page, emails_per_page, token.id)
        .await
        .expect("Cannot find emails in page");

    Ok(Json(EmailsWithPageDto {
        page,
        emails_per_page,
        num_pages,
        emails,
    }))
}

/// # get my email by Id
#[openapi(tag = "EMAIL")]
#[get("/email/<id>")]
pub async fn get_by_id(conn: Connection<'_, Db>,token: HttpAuth, id: i64) -> R<Option<email::Model>> {
    let db = conn.into_inner();

    let _email = if Query::find_email_by_id(db, id).await.unwrap().unwrap().user_id != token.id {
        let m = error::Error {
            err: "Invalid authentication".to_string(),
            msg: Some("Invalid authentication".to_string()),
            http_status_code: 401,
        };
        return Err(m)
    };

    let email: Option<email::Model> = Query::find_email_by_id(db, id)
        .await
        .expect("could not find email");
    Ok(Json(email))
}

/// # delete my email by Id
#[openapi(tag = "EMAIL")]
#[delete("/email/<id>")]
pub async fn delete(conn: Connection<'_, Db>, id: i64, token: HttpAuth) -> R<Option<String>> {
    let db = conn.into_inner();

    let email: Option<email::Model> = Query::find_email_by_id(db, id)
        .await
        .expect("could not find email");
    
        let _email = if email.clone().unwrap().user_id != token.id {
            let m = error::Error {
                err: "Invalid authentication".to_string(),
                msg: Some("Invalid authentication".to_string()),
                http_status_code: 401,
            };
            return Err(m)
        };

    let cmd = Mutation::delete_email(db, email.unwrap().id);
    match cmd.await {
        Ok(_) => Ok(Json(Some("Email successfully deleted.".to_string()))),
        Err(e) => {
            let m = error::Error {
                err: "Error deleting email".to_string(),
                msg: Some(e.to_string()),
                http_status_code: 400,
            };
            Err(m)
        }
    }
}

/// # delete all emails of mine
#[openapi(tag = "EMAIL")]
#[delete("/emails")]
pub async fn destroy(conn: Connection<'_, Db>, token: HttpAuth) -> R<Option<String>> {
    let db = conn.into_inner();

    let cmd = Mutation::delete_all_emails(db, token.id);

    match cmd.await {
        Ok(_) => Ok(Json(Some(
            "All Your Emails were successfully deleted.".to_string(),
        ))),
        Err(e) => {
            let m = error::Error {
                err: "Error deleting all emails at once".to_string(),
                msg: Some(e.to_string()),
                http_status_code: 400,
            };
            Err(m)
        }
    }
}
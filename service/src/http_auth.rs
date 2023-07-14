//! ------ HTTP `Authorization` header ------

use rocket::serde::json::Json;
use rocket::{
    get,
    http::Status,
    request::{self, FromRequest, Outcome},
};
use rocket_okapi::okapi::openapi3::{
    Object, SecurityRequirement, SecurityScheme, SecuritySchemeData,
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    openapi,
    request::{OpenApiFromRequest, RequestHeaderInput},
};
use std::env;

pub struct HttpAuth{
    pub id: i32,
}

// Implement the actual checks for the authentication
#[rocket::async_trait]
impl<'a> FromRequest<'a> for HttpAuth {
    type Error = &'static str;
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        // Get the token from the http header
        match request.headers().get_one("Authorization") {
            Some(token) => {
            if token == "Bearer mytoken" {

            let data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(env::var("jwt_secret").unwrap().as_bytes()),
                &Validation::new(jsonwebtoken::Algorithm::HS256),
            );

            let claims = match data {
                Ok(p) => p.claims,
                Err(_) => {
                    return Outcome::Failure((Status::Unauthorized, "Invalid token"))
                }
            };

            Outcome::Success(HttpAuth{ id: claims.sub })
                } else {
                    Outcome::Failure((Status::Unauthorized, "Auth is invalid."))
                }
            }
            None => Outcome::Failure((Status::BadRequest, "Missing `Authorization` header.")),
        }
        // For more info see: https://rocket.rs/v0.5-rc/guide/state/#within-guards
    }
}

impl<'a> OpenApiFromRequest<'a> for HttpAuth {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        // Setup global requirement for Security scheme
        let security_scheme = SecurityScheme {
            description: Some(
                "Requires an Bearer token to access, token is: `mytoken`.".to_owned(),
            ),
            // Setup data requirements.
            // In this case the header `Authorization: mytoken` needs to be set.
            data: SecuritySchemeData::Http {
                scheme: "bearer".to_owned(), // `basic`, `digest`, ...
                // Just gives use a hint to the format used
                bearer_format: Some("bearer".to_owned()),
            },
            extensions: Object::default(),
        };
        // Add the requirement for this route/endpoint
        // This can change between routes.
        let mut security_req = SecurityRequirement::new();
        // Each security requirement needs to be met before access is allowed.
        security_req.insert("HttpAuth".to_owned(), Vec::new());
        // These vvvvvvv-----^^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "HttpAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}


use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::{
    request::Request,
    serde::{Deserialize, Serialize},
};


#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: i32,
    pub role: String,
    pub exp: u64,
}

pub struct AuthenticatedUser {
    pub id: i32,
}

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
    pub jwt_secret: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("BOOKSTORE_DB_HOST").unwrap_or("localhost".to_string()),
            db_port: std::env::var("BOOKSTORE_DB_PORT").unwrap_or("3306".to_string()),
            db_username: std::env::var("BOOKSTORE_DB_USERNAME").unwrap_or("root".to_string()),
            db_password: std::env::var("BOOKSTORE_DB_PASSWORD").unwrap_or("".to_string()),
            db_database: std::env::var("BOOKSTORE_DB_DATABASE").unwrap_or("example-api".to_string()),
            jwt_secret: std::env::var("BOOKSTORE_JWT_SECRET")
                .expect("Please set the BOOKSTORE_JWT_SECRET env variable."),
        }
    }
}

//! ------ HTTP `Authorization` header ------

use config::app_config::AppConfig;
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

pub struct HttpAuth{
    pub id: i64,
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
            if token.contains("Bearer ") {
            let data = decode::<Claims>(
                &token.replace("Bearer ", ""),
                &DecodingKey::from_secret(AppConfig::default().jwt_secret.as_bytes()),
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
    pub sub: i64,
    pub role: String,
    pub exp: u64,
}

pub struct AuthenticatedUser {
    pub id: i64,
}

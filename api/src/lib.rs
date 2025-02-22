#[macro_use]
extern crate rocket;

use sqlx::mysql::MySqlPoolOptions;
use config::app_config::AppConfig;
use rocket::fairing::{self, AdHoc};
use rocket::{Build, Rocket};

use migration::{MigratorTrait, DbErr};
use sea_orm_rocket::Database;

use rocket_okapi::mount_endpoints_and_merged_docs;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};

mod pool;
use pool::Db;
mod error;
mod okapi;

pub use entity::post;
pub use entity::post::Entity as Post;

use rocket::http::Status;

pub mod auth;
pub mod user;
pub mod email;

use rocket::Request;

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[tokio::main]
async fn start() -> Result<(), rocket::Error> {
    let mut query_string = "CREATE DATABASE `".to_owned();
    query_string.push_str(&AppConfig::default().db_database);
    query_string.push_str("` /*!40100 COLLATE 'utf8mb4_unicode_ci' */;");

         // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = MySqlPoolOptions::new().connect(&AppConfig::default().db_url_origin).await.unwrap();
    let _ = sqlx::query(&query_string).execute(&pool).await;
    let config = AppConfig::default();
    let mut building_rocket = rocket::build()
    .manage(config)
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../v1/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("Rocket/SeaOrm - RapiDoc documentation | RapiDoc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../v1/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .attach(cors());

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    let custom_route_spec = (vec![], custom_openapi_spec());
    mount_endpoints_and_merged_docs! {
        building_rocket, "/v1".to_owned(), openapi_settings,
            "/additional" => custom_route_spec,
            "/api" => okapi::get_routes_and_docs(&openapi_settings),
    };
    building_rocket
    .launch().await.map(|_| ())
}

fn cors() -> Cors {
    let mut url = "http://".to_string();
    url.push_str(&AppConfig::default().host);
    url.push_str(":");
    url.push_str(&AppConfig::default().port_web);
    let allowed_origins =
    AllowedOrigins::some_exact(&["http://localhost:8000", "http://127.0.0.1:8000", &url]);
    // AllowedOrigins::all();
        

    rocket_cors::CorsOptions {
        // send_wildcard: true,
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete, Method::Patch, ]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap()
}

use rocket_okapi::okapi::openapi3::*;
fn custom_openapi_spec() -> OpenApi {
    let mut url = "http://".to_string();
    url.push_str(&AppConfig::default().host);
    url.push_str(":");
    url.push_str(&AppConfig::default().port_web);
    url.push_str("/v1");
    OpenApi {
        openapi: OpenApi::default_version(),
        info: Info {
            title: "Tokio-Tungstenite-Rocket-Okapi".to_owned(),
            description: Some("API Docs for Rocket/SeaOrm example".to_owned()),
            terms_of_service: Some("https://github.com/SeaQL/sea-orm#license".to_owned()),
            contact: Some(Contact {
                name: Some("SeaOrm".to_owned()),
                url: Some("https://github.com/SeaQL/sea-orm".to_owned()),
                email: None,
                ..Default::default()
            }),
            license: Some(License {
                name: "MIT".to_owned(),
                url: Some("https://github.com/SeaQL/sea-orm/blob/master/LICENSE-MIT".to_owned()),
                ..Default::default()
            }),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            ..Default::default()
        },
        servers: vec![
            Server {
                url: url.to_owned(),
                description: Some("Remote development server".to_owned()),
                ..Default::default()
            },
            Server {
                url: "http://127.0.0.1:8000/v1".to_owned(),
                description: Some("Localhost".to_owned()),
                ..Default::default()
            },
            // Server {
            //     url: "https://production-server.com/".to_owned(),
            //     description: Some("Remote development server".to_owned()),
            //     ..Default::default()
            // },
        ],
        ..Default::default()
    }
}

pub fn main() {
    let result = start();

    println!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}


#[derive(Responder)]
pub struct SuccessResponse<T>(pub (Status, T));

#[derive(Responder)]
pub struct ErrorResponse(pub (Status, String));

pub type Response<T> = Result<SuccessResponse<T>, ErrorResponse>;

impl From<DbErr> for ErrorResponse {
    fn from(err: DbErr) -> Self {
        ErrorResponse((Status::InternalServerError, err.to_string()))
    }
}
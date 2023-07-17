use config::app_config::AppConfig;
use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {

    //  Setting `DATABASE_URL` environment variable
    let key = "DATABASE_URL";
    if std::env::var(key).is_err() {
        // // Getting the database URL from Rocket.toml if it's not set
        // let figment = rocket::Config::figment();
        // let database_url: String = figment
        //     .extract_inner("databases.sea_orm.url")
        //     .expect("Cannot find Database URL in Rocket.toml");
        let database_url: String = AppConfig::default().db_url;
        std::env::set_var(key, database_url);
    }

    cli::run_cli(migration::Migrator).await;
}

use std::{thread, env};
use dotenv::dotenv;

fn main() {
    // dotenv().ok();
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("DB_PROTOCOL", "mysql");
    env::set_var("JWT_SECRET", "3aDXN0u2KJ5kn9ZBl5MbGDySlzDmpZ");
    env::set_var("DB_HOST", "10.46.40.102");
    env::set_var("DB_PORT", "3306");
    env::set_var("DB_USERNAME", "remote_admin");
    env::set_var("DB_PASSWORD", "password");
    env::set_var("DB_DATABASE", "tokio_tungstenite_rocket");
    env::set_var("HOST", "10.46.40.102");
    env::set_var("BROKER_URL", "121.172.169.213:29092");
    thread::spawn(|| {
        let _ = websocket::main();
    });
    rocket_okapi::main();
}

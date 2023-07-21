use std::{thread, env};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    env::set_var("RUST_BACKTRACE", "1");
    thread::spawn(|| {
        let _ = websocket::main();
    });
    rocket_okapi::main();
}
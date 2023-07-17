use std::thread;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    thread::spawn(|| {
        let _ = websocket::main();
    });
    rocket_okapi::main();
}
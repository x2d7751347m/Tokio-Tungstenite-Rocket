use std::thread;

fn main() {
    thread::spawn(|| {
        let _ = websocket::websocket();
    });
    rocket_example_api::main();
}
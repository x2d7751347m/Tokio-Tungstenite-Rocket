use std::thread;

fn main() {
    thread::spawn(|| {
        let _ = websocket::websocket_main();
    });
    rocket_example_api::main();
}
use std::thread;

fn main() {
    thread::spawn(|| {
        let _ = websocket::main();
    });
    rocket_example_api::main();
}
use proxy::prelude::test_target_server;
use proxy::proxy;
use std::thread;

fn main() {
    thread::Builder::new()
        .name("1".to_string())
        .spawn(|| {
            test_target_server();
        })
        .expect("Thread 1 error");
    proxy();
}

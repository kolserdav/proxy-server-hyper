use proxy::prelude::pass;
use proxy::proxy;
use std::thread;

fn main() {
    thread::Builder::new()
        .name("1".to_string())
        .spawn(|| {
            pass();
        })
        .expect("Thread 1 error");
    proxy();
}

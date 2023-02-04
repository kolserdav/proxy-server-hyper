pub mod pass;
use pass::pass;
use proxy::proxy;
use std::thread;

fn main() {
    thread::spawn(|| {
        pass();
    });
    proxy();
}

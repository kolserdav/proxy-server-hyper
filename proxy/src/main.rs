use std::thread;
extern crate pass;
use pass::pass;
use proxy::proxy;

fn main() {
    thread::spawn(|| {
        pass();
    });
    proxy();
}

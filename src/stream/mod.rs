use futures::{channel::mpsc, executor::ThreadPool, task::SpawnExt, SinkExt, Stream};
use std::{io::Read, io::Result};

pub fn stream() -> impl Stream<Item = Result<Vec<u8>>> {
    let (mut tx, rx) = mpsc::channel(10);
    let pool = ThreadPool::new().unwrap();
    pool.spawn(async move {
        let mut f = std::fs::File::open("./Cargo.toml").expect("file not found");
        loop {
            let mut d = [0; 1];
            let len = f.read(&mut d).unwrap();
            if len == 0 {
                break;
            }
            let mut vec = Vec::new();
            d.map(|_d| {
                if _d == 0 {
                    return;
                }
                vec.push(_d);
            });
            tx.send(Ok(vec)).await.expect("Unable to send block");
        }
    })
    .expect("Unable to spawn thread");
    rx
}

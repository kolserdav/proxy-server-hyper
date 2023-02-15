use super::constants::CHUNK_SIZE;
use futures::{
    channel::mpsc::{channel, Receiver},
    executor::ThreadPool,
    task::SpawnExt,
    SinkExt,
};
use std::io::{Read, Result};

pub fn stream() -> Receiver<Result<Vec<u8>>> {
    let (mut tx, rx) = channel(10);
    let pool = ThreadPool::new().unwrap();
    let block = async move {
        let mut f = std::fs::File::open("./Cargo.toml").expect("file not found");
        loop {
            let mut d = [0; CHUNK_SIZE];
            let len = f.read(&mut d).unwrap();
            if len == 0 {
                break;
            }
            let mut vec = vec![];
            d.map(|_d| {
                if _d == 0 {
                    return;
                }
                vec.push(_d);
            });
            tx.send(Ok(vec)).await.expect("Unable to send block");
        }
    };
    pool.spawn(block).expect("Unable to spawn thread");
    rx
}

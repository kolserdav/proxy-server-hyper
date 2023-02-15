use futures::{
    channel::mpsc,
    executor::ThreadPool,
    task::{Context, Poll, SpawnExt},
    Future, SinkExt, Stream,
};
use std::io::{Read, Result};
use std::pin::Pin;

struct Res;

impl Future for Res {
    type Output = Self;
    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Res> {
        Poll::Ready(Res {})
    }
}

pub fn stream(cb: impl Future<Output = ()>) -> impl Stream<Item = Result<Vec<u8>>> {
    let (mut tx, rx) = mpsc::channel(10);
    let pool = ThreadPool::new().unwrap();
    let block = async move {
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
    };
    pool.spawn(block).expect("Unable to spawn thread");
    rx
}

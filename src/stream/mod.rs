use futures::{
    channel::mpsc, executor::ThreadPool, task::SpawnExt, SinkExt, Stream as FeaturesStream,
};
use std::{io, thread, time::Duration};

pub struct Stream;

impl Stream {
    fn new() -> Self {
        thread::sleep(Duration::from_secs(1));
        Self
    }

    fn next_block(&self) -> io::Result<&[u8]> {
        thread::sleep(Duration::from_secs(1));
        Ok(b"data")
    }
}

pub fn stream(pool: ThreadPool) -> impl FeaturesStream<Item = io::Result<Vec<u8>>> {
    let (mut tx, rx) = mpsc::channel(10);
    pool.spawn(async move {
        let sd = Stream::new();
        for _ in 0..3 {
            let block = sd.next_block().map(|b| b.to_vec());
            tx.send(block).await.expect("Unable to send block");
        }
    })
    .expect("Unable to spawn thread");
    rx
}

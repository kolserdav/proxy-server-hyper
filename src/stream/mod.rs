use super::constants::{CHUNK_SIZE, TEST_RESULT_FILE};
use super::prelude::create_config;
use futures::{
    channel::mpsc::{channel, Receiver},
    executor::ThreadPool,
    task::SpawnExt,
    SinkExt,
};
use hyper::{Body, Client, Request, Response, Uri};
use std::{
    fs::File,
    io::{Read, Result},
};
use tokio::net::TcpStream;

pub fn stream_tcp(_req: Request<Body>) -> Receiver<Result<Vec<u8>>> {
    let (mut tx, rx) = channel(10);
    let pool = ThreadPool::new().unwrap();
    let block = async move {
        let std_stream = std::net::TcpStream::connect("192.168.0.3:3001").expect("Err 66");
        std_stream.set_nonblocking(true).expect("Err 89");
        let _stream = TcpStream::from_std(std_stream).expect("Err 3554");

        // let mut _stream = TcpStream::connect("192.168.0.3:3001")
        //   .await
        //  .expect("err 23"); //expect("Err 2332");
        loop {
            let mut d = [0; CHUNK_SIZE];
            let len = _stream.try_read(&mut d);
            println!("{:?}", len);
            if let Err(_) = len {
                break;
            }
            let len = len.unwrap();
            /*
            if len == 0 {
                break;
            }
            */
            println!("{:?}", len);
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

pub fn stream(_req: Request<Body>) -> Receiver<Result<Vec<u8>>> {
    let (mut tx, rx) = channel(10);
    let pool = ThreadPool::new().unwrap();
    let block = async move {
        let client = Client::new();
        let config = create_config().expect("Failed parse target address");
        let uri = format!(
            "http://{}:{}{}",
            config.test_host,
            config.test_port,
            _req.uri()
        )
        .parse::<Uri>()
        .expect("Failed parse target uri");
        let body = format!("{:?}", _req.body());
        let req = Request::builder()
            .method(_req.method())
            .uri(uri)
            .body(Body::from(body))
            .expect("Failed proxy request");
        let res = client.request(req);
        println!("{:?}", res);
    };
    pool.spawn(block).expect("Unable to spawn thread");
    rx
}

pub fn test_target_stream() -> Receiver<Result<Vec<u8>>> {
    let (mut tx, rx) = channel(10);
    let pool = ThreadPool::new().unwrap();
    let block = async move {
        let mut f = File::open(TEST_RESULT_FILE).expect("file not found");
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

use super::constants::CHUNK_SIZE;
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
    net::TcpStream,
};

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
        let mut f = File::open("./Cargo.toml").expect("file not found");
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

async fn stream_tcp(_req: Request<Body>) -> Receiver<Result<Vec<u8>>> {
    let (mut tx, rx) = channel(10);
    let pool = ThreadPool::new().unwrap();
    let block = async move {
        let mut _stream = TcpStream::connect("192.168.0.3:3001").expect("Err 2332");
        loop {
            let mut d = [0; 1];
            let len = _stream.peek(&mut d);
            println!("{:?}", len);
            if let Err(_) = len {
                break;
            }
            let len = len.unwrap();
            println!("{}", len);
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

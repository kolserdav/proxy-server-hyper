use super::constants::{CHUNK_SIZE, TEST_RESULT_FILE};
use super::prelude::{build_req_headers, create_config};
use futures::{
    channel::mpsc::{channel, Receiver},
    executor::ThreadPool,
    task::SpawnExt,
    SinkExt,
};
use hyper::{Body, Request};
use std::{
    fs::File,
    io::{Read, Result, Write},
    net::TcpStream,
};

pub fn stream_tcp(_req: Request<Body>) -> Receiver<Result<Vec<u8>>> {
    let (mut tx, rx) = channel(10);
    let pool = ThreadPool::new().unwrap();
    println!("{:?}", _req);
    let block = async move {
        // TODO provide address
        let mut socket = TcpStream::connect("192.168.0.3:3001").expect("err 23");
        let headers = build_req_headers(_req);
        println!("proxy to: {}", &headers);
        socket.write(headers.as_bytes()).expect("Err 432");
        loop {
            let mut d = [0; CHUNK_SIZE];
            let len = socket.read(&mut d);
            if let Err(_) = len {
                break;
            }
            let len = len.unwrap();
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

pub fn test_target_stream(_req: Request<Body>) -> Receiver<Result<Vec<u8>>> {
    println!("request to test target: {:?}", _req);
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

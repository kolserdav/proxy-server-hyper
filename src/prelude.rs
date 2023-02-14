use std::net::IpAddr;
extern crate dotenv;
use super::error::Result;
use dotenv::dotenv;
use futures_util;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::env;
use std::fs::File;
use std::io::prelude::Read;
use std::io::BufRead;
use std::net::SocketAddr;
use std::str::FromStr;

const PORT: &str = "3000";
const HOST: &str = "127.0.0.1";
const TEST_PORT: &str = "3001";
const TEST_HOST: &str = "127.0.0.1";

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub host: IpAddr,
    pub test_port: u16,
    pub test_host: IpAddr,
}

pub fn create_config() -> Result<Config> {
    dotenv().ok();
    let mut port: Option<String> = None;
    let mut host: Option<String> = None;
    let mut test_port: Option<String> = None;
    let mut test_host: Option<String> = None;
    for (key, value) in env::vars() {
        if key == "PORT" {
            port = Some(value);
        } else if key == "HOST" {
            host = Some(value);
        } else if key == "TEST_PORT" {
            test_port = Some(value);
        } else if key == "TEST_HOST" {
            test_host = Some(value);
        }
    }
    let port = match port {
        Some(v) => v
            .parse::<u16>()
            .expect("Port from env.PORT is not a integer"),
        None => PORT.parse::<u16>()?,
    };
    let host = match host {
        Some(v) => parse_host(v)?,
        None => parse_host(HOST.to_string())?,
    };
    let test_port = match test_port {
        Some(v) => v
            .parse::<u16>()
            .expect("Port from env.TEST_PORT is not a integer"),
        None => TEST_PORT.parse::<u16>()?,
    };
    let test_host = match test_host {
        Some(v) => parse_host(v)?,
        None => parse_host(TEST_HOST.to_string())?,
    };
    Ok(Config {
        port,
        host,
        test_host,
        test_port,
    })
}

pub fn parse_host(host: String) -> Result<IpAddr> {
    let ip = IpAddr::from_str(host.as_str()).expect("Error parse host");
    Ok(ip)
}

#[test]
fn test_create_config() {
    env::set_var("HOST", "127.0.0.1");
    env::set_var("PORT", "3000");
    let config = create_config();
    assert!(config.is_ok());
    let config = config.unwrap();
    assert_eq!(config.port, 3000);
    let host_arr = [127, 0, 0, 1];
    assert_eq!(config.host, IpAddr::from(host_arr));
}

#[tokio::main]
pub async fn pass() {
    let config = create_config().expect("Failed parse config");
    println!(
        "Listen test target server at: http://{:?}:{} ...",
        &config.test_host, &config.test_port
    );
    let addr = SocketAddr::from((config.test_host, config.test_port));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_world)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

// impl Into<hyper::body::Bytes> for u8 {}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("{:?} body: {:?}", _req, _req.body());
    let f = File::open("./lib.rs").unwrap();
    // let stream = tokio_codec::F:new(f);:wq
    let mut buf = std::io::BufReader::new(f);
    let mut vec = Vec::<u8>::new();
    loop {
        let mut ch: [u8; 1] = [0; 1];
        let d = buf.read(&mut ch);
        if let Err(_) = d {
            break;
        }
        ch.map(|i| {
            vec.push(i);
        });
    }
    let body = hyper::Body::wrap_stream(vec);
    Ok(Response::new(body))
}

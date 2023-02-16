extern crate dotenv;
use super::error::Result;
use super::stream::test_target_stream;
pub mod constants;
use constants::{HOST, PORT, TEST_HOST, TEST_PORT};
use dotenv::dotenv;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::{convert::Infallible, env, net::IpAddr, net::SocketAddr, str::FromStr};

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub host: IpAddr,
    pub test_port: u16,
    pub test_host: IpAddr,
}

#[tokio::main]
pub async fn test_target_server() {
    let config = create_config().expect("Failed parse config");
    println!(
        "Listen test target server at: http://{:?}:{} ...",
        &config.test_host, &config.test_port
    );
    let addr = SocketAddr::from((config.test_host, config.test_port));

    let make_service = make_service_fn(|_socket| async {
        let svc_fn = service_fn(move |_request| async {
            let data = test_target_stream(_request);
            let resp = Response::new(Body::wrap_stream(data));
            Result::<_, Infallible>::Ok(resp)
        });
        Result::<_, Infallible>::Ok(svc_fn)
    });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
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

pub fn build_req_headers(_req: Request<Body>) -> String {
    let mut headers: String = "".to_string();
    for (k, v) in _req.headers().into_iter() {
        headers += format!("{}: {:?}\r\n", k, v).as_str();
    }
    format!(
        "{} {} {:?}\r\n{}\r\n
        Connection: close\r\n\r\n",
        _req.method(),
        _req.uri(),
        _req.version(),
        headers
    )
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

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, Uri};
pub mod error;
pub mod prelude;
pub mod stream;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::{executor::BlockingStream, stream::Stream as FutureStream, Future};
use prelude::*;
use std::convert::Infallible;
use std::io::prelude::Read;
use std::net::SocketAddr;
use tokio::io::BufWriter;

#[tokio::main]
pub async fn proxy() {
    let config = create_config().expect("Failed parse config");
    println!(
        "Listen proxy server at: http://{:?}:{} ...",
        &config.host, &config.port
    );
    let addr = SocketAddr::from((config.host, config.port));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(target)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn target(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
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
    let mut stream = std::net::TcpStream::connect("127.0.0.1:3001").expect("Error 232");
    println!("{:?}", stream);
    let mut vec = Vec::<u8>::new();
    loop {
        let mut d = [0; 1];
        let len = stream.read(&mut d);
        if let Err(_) = len {
            break;
        }
        let len = len.unwrap();
        println!("{}", len);
        if len == 0 {
            break;
        }
        vec.push(d[0]);
    }
    let body = Body::from(vec);
    Ok(Response::new(body))
}

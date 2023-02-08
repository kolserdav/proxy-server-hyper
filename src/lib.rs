use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, Uri};
pub mod error;
pub mod prelude;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::{ready, stream::Stream as FutureStream};
use prelude::*;
use std::convert::Infallible;
use std::io::prelude::Read;
use std::net::SocketAddr;

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
    let mut stream = std::net::TcpStream::connect("127.0.0.1:3001").expect("78");
    println!("{:?}", stream);
    let mut d = [0; 128];
    // stream.write(&[1])?;

    stream.read(&mut d);
    let body = Body::wrap_stream(stream);
    Ok(Response::new(body))
    // let buf = hyper::body::to_bytes(res).await?;
    //Ok(Response::new("".into()))
}

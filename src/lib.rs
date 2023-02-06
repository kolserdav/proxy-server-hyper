use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Method, Request, Response, Server, Uri};
pub mod error;
pub mod prelude;
use prelude::*;
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
pub async fn proxy() {
    let config = create_config().expect("Failed parse config");
    println!(
        "Listen proxy server at: http://{:?}:{} ...",
        &config.host, &config.port
    );
    let addr = SocketAddr::from((config.host, config.port));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_world)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let client = Client::new();
    let config = create_config().expect("Failed parse test config");
    let uri = format!(
        "http://{}:{}{}",
        config.test_host,
        config.test_port,
        _req.uri()
    )
    .parse::<Uri>()
    .expect("Failed parse target uri");
    let body = _req.body().clone();
    let req = Request::builder()
        .method(_req.method())
        .uri(uri)
        .body(*body)
        .expect("request builder");
    let res = client.request(req).await?;
    let buf = hyper::body::to_bytes(res).await?;
    Ok(Response::new(buf.into()))
}

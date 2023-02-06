use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
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
    use hyper::{body::HttpBody as _, Client, Uri};

    let client = Client::new();

    let config = create_config().expect("Failed parse test config");
    // Make a GET /ip to 'http://httpbin.org'
    let uri = format!("http://{}:{}", config.test_host, config.test_port);
    let res = client
        .get(uri.parse::<Uri>().expect("Error parse uri"))
        .await?;

    // And then, if the request gets a response...
    println!("status: {}", res.status());

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;

    println!("body: {:?}", buf);
    Ok(Response::new(buf.into()))
}

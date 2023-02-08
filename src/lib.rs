use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, Uri, body::HttpBody};
pub mod error;
pub mod prelude;
use prelude::*;
use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::client::HttpConnector;
use futures::{TryFutureExt, TryStreamExt};
use hyper_proxy::{Proxy, ProxyConnector, Intercept};
use headers::Authorization;
use std::error::Error;
use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
pub async fn proxy() -> Result<(), Box<dyn Error>> {
    let proxy = {
        let proxy_uri = "http://127.0.0.1:3000".parse().unwrap();
        let mut proxy = Proxy::new(Intercept::All, proxy_uri);
        proxy.set_authorization(Authorization::basic("John Doe", "Agent1234"));
        let connector = HttpConnector::new();
        let proxy_connector = ProxyConnector::from_proxy(connector, proxy).unwrap();
        proxy_connector
    };

    // Connecting to http will trigger regular GETs and POSTs.
    // We need to manually append the relevant headers to the request
    let uri: Uri = "http://127.0.0.1:3001".parse().unwrap();
    let mut req = Request::get(uri.clone()).body(hyper::Body::empty()).unwrap();

    if let Some(headers) = proxy.http_headers(&uri) {
        req.headers_mut().extend(headers.clone().into_iter());
    }

    let client = Client::builder().build(proxy);
    let mut resp = client.request(req).await?;
    println!("Response: {}", resp.status());
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    // Connecting to an https uri is straightforward (uses 'CONNECT' method underneath)
    let uri = "https://my-remote-websitei-secured.com".parse().unwrap();
    let mut resp = client.get(uri).await?;
    println!("Response: {}", resp.status());
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    Ok(())
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
    let res = client.request(req).await?;
    let buf = hyper::body::to_bytes(res).await?;
    Ok(Response::new(buf.into()))
}

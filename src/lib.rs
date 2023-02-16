use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Response, Server};
pub mod error;
pub mod prelude;
pub mod stream;
use prelude::*;
use std::convert::Infallible;
use std::net::SocketAddr;
use stream::{stream, stream_tcp};

#[tokio::main]
pub async fn proxy() {
    let config = create_config().expect("Failed parse config");
    println!(
        "Listen proxy server at: http://{:?}:{} ...",
        &config.host, &config.port
    );
    let addr = SocketAddr::from((config.host, config.port));

    let make_service = make_service_fn(|_socket| async {
        let svc_fn = service_fn(move |_request| async {
            let data = stream_tcp(_request);
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

use hyper::{service::{make_service_fn, service_fn}, Body, Method, Request, Response, Server, StatusCode};

use crate::streams::ChannelAuthor;

use std::{net::SocketAddr, sync::{Arc, Mutex}};
use crate::http::*;

static NOTFOUND: &[u8] = b"Not Found";
type GenericError = Box<dyn std::error::Error + Send + Sync>;

pub async fn start(
    port: u16,
    author: Arc<Mutex<ChannelAuthor>>
) -> Result<(), GenericError> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let service = make_service_fn(move |_| {
        let author = author.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                responder(
                    req,
                    author.clone()
                )
            }))
        }
    });

    let server = Server::bind(&addr).serve(service);

    println!("API listening on http://{}", addr);

    server.await?;

    Ok(())
}

async fn responder(
    req: Request<Body>,
    author: Arc<Mutex<ChannelAuthor>>
) -> Result<Response<Body>, GenericError> {
    match req.method() {
        &Method::OPTIONS => preflight_response().await,
        _ => match (req.method(), req.uri().path()) {
            (&Method::POST, "/subscribe") => subscribe_response(req, author).await,
            (&Method::GET, "/get_channel_address") => {
                channel_address_response(author).await
            }
            (&Method::GET, "/get_announcement_id") => {
                announcement_id_response(author).await
            }
            _ => {
                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(NOTFOUND.into())
                    .unwrap())
            }
        }
    }
}

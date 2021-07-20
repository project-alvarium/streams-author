use hyper::{Request, Body, Response, StatusCode, header};
use crate::streams::ChannelAuthor;
use std::sync::{Mutex, Arc};
use crate::models::subscription::SubscriptionRequest;

type GenericError = Box<dyn std::error::Error + Send + Sync>;

pub async fn preflight_response(
) -> Result<Response<Body>, GenericError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS, PUT, PATCH, DELETE")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .body(Body::from("OK"))
        .unwrap())
}

pub async fn subscribe_response(
    req: Request<Body>,
    author: Arc<Mutex<ChannelAuthor>>,
) -> Result<Response<Body>, GenericError> {
    let data = hyper::body::to_bytes(req.into_body()).await?;

    let response;
    let json_data: serde_json::Result<SubscriptionRequest> = serde_json::from_slice(&data);
    match json_data {
        Ok(sub_req) => {
            let mut author = author.lock().unwrap();
            match author.subscribe(&sub_req.msgid, &hex::decode(sub_req.pk)?) {
                Ok(keyload_link) => {
                    println!("Processed subscription, returning keyload link...");
                    response = Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "application/json")
                        .header("Access-Control-Allow-Origin", "*")
                        .body(Body::from("Subscription processed, keyload link: ".to_owned() + &keyload_link.to_string()))?;
                },
                Err(_) => {
                    response = Response::builder()
                        .status(500)
                        .header(header::CONTENT_TYPE, "application/json")
                        .header("Access-Control-Allow-Origin", "*")
                        .body(Body::from("Error while subscribing"))?;
                }
            }
        },
        Err(e) => {
            dbg!("Error in formatting: {:?}", e);
            response = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(header::CONTENT_TYPE, "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::from("Malformed json request"))?;
        }
    }

    Ok(response)
}

pub async fn channel_address_response(
    author: Arc<Mutex<ChannelAuthor>>,
) -> Result<Response<Body>, GenericError> {
    let response;

    let author = author.lock().unwrap();
    match author.get_channel_address() {
        Ok(channel_address) => {
            response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::from("{ \"channel_address\": \"".to_owned() + &channel_address + "\" }"))?;
        },
        Err(_e) => {
            response = Response::builder()
                .status(500)
                .header(header::CONTENT_TYPE, "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::from("Error while getting channel address"))?;
        }
    }

    Ok(response)
}

pub async fn announcement_id_response(
    author: Arc<Mutex<ChannelAuthor>>,
) -> Result<Response<Body>, GenericError> {
    let response;

    let author = author.lock().unwrap();
    match author.get_announcement_id() {
        Ok(announcement_id) => {
            response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::from("{ \"announcement_id\": \"".to_owned() + &announcement_id.0 +
                    ":" + &announcement_id.1.to_owned() + "\" }"))?;
        },
        Err(_e) => {
            response = Response::builder()
                .status(500)
                .header(header::CONTENT_TYPE, "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::from("Error while getting announcement id"))?;
        }
    }

    Ok(response)
}

fn busy() -> Response<Body>{
    Response::builder()
        .status(500)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(Body::from("Service is busy"))
        .unwrap()
}

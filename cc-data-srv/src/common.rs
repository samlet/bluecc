use std::convert::Infallible;
use warp::{Rejection, Reply};
use serde_derive::Serialize;
use deles::GenericError;
use warp::Filter;
use serde::de::DeserializeOwned;

fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

pub fn to_http_status(err: &GenericError) -> warp::http::StatusCode {
    match err {
        GenericError::NotFound{ .. } => warp::http::StatusCode::NOT_FOUND,
        GenericError::Unknown => warp::http::StatusCode::BAD_REQUEST,
        _ => warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let mut message:String=String::new();

    if err.is_not_found() {
        code = warp::http::StatusCode::NOT_FOUND;
        message.push_str("Not Found");
    } else if let Some(app_err) = err.find::<GenericError>() {
        code = to_http_status(app_err);
        // message = app_err.message.as_str();
        message = format!("err: {:?}", app_err);
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = warp::http::StatusCode::BAD_REQUEST;
        message.push_str("Invalid Body");
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = warp::http::StatusCode::METHOD_NOT_ALLOWED;
        message.push_str("Method Not Allowed");
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
        message.push_str("Unhandled rejection");
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}


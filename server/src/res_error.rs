use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ServerFail
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{:#?}", self);
        (StatusCode::BAD_REQUEST, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
/**
 * This module is responsible to send emails
 */


use axum::{http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{services::{email_service, signature_service::{self, MsgToSign}}, Error, Result};



#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureReqBody {
    email: String,
    slug: Option<String>,
    timestamp: u32
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SignedEmailBody {
    signature: String,
    message: String,
    key_index: u32
}


impl SignedEmailBody {
    fn new(signature: String, message: String, key_index: u32) -> Self {
        SignedEmailBody {signature, message, key_index}
    }
}


pub fn routes() -> Router {
    Router::new().route("/sign_msg", get(sign_message))
}


async fn sign_message(Json(req_body): Json<SignatureReqBody>) -> Result<StatusCode> {
    if !is_email_valid(&req_body.email) {
        return Err(Error::ServerFail);
    }

    let random_slug = req_body.slug.unwrap_or(String::from("some_random_slug"));

    let signature_response = signature_service::sign_message(MsgToSign::new(random_slug, req_body.timestamp))?;

    match email_service::send_email(&req_body.email, json!({"signature_response": signature_response})).await? {
        email_service::EmailSericeResult::Delivered => Ok(StatusCode::OK),
        email_service::EmailSericeResult::Failed => Err(Error::ServerFail),
    }
}

fn is_email_valid(email: &str) -> bool {
    // TODO: Implement email validation logic
    true
}
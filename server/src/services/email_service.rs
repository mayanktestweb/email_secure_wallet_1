/*
    This module will be the base where email services will be implemented
*/


use axum::Json;
use serde_json::Value;

use crate::Result;

#[derive(Debug)]
pub enum EmailSericeResult {
    Delivered,
    Failed
}

pub async fn send_email(email: &str, email_body: Value) -> Result<EmailSericeResult> {
    println!("{email}");
    print!("{:#?}", email_body);
    Ok(EmailSericeResult::Delivered)
}
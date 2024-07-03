/*
    This module will be the base where email services will be implemented
*/


use std::fmt::Display;

use axum::{response::Html, Json};
use serde_json::Value;

use super::signature_service::MsgSignature;

//
// Essential Traits for Email Service
//

pub trait IEmail {
    fn receiver(&self) -> Option<String>;
    fn message(&self) -> Option<Html<String>>;
}

pub trait IEmailService {
    fn send_email(&self, email: Box<dyn IEmail>, email_service_provider: Box<dyn IEmailServiceProvider>) -> Result<(), Box<dyn std::error::Error>>;
}


pub trait IEmailServiceProvider {
    fn deliver_mail(&self, email: Box<dyn IEmail>) -> Result<(), Box<dyn std::error::Error>>;
}


//////////////////////////////////////////////////////



pub struct SignatureEmail {
    receiver: String,
    msg_signature: MsgSignature
}

impl SignatureEmail {
    pub fn new(receiver: String, msg_signature: MsgSignature) -> Self {
        SignatureEmail { receiver, msg_signature }
    }
}

impl IEmail for SignatureEmail {
    fn receiver(&self) -> Option<String> {
        Some(self.receiver)
    }

    fn message(&self) -> Option<Html<String>> {
        Some(Html(format!("<p>Use the message ===> <b>{}</b> with this signature to verify ===> <b>{}</b></p>", self.msg_signature.msg, self.msg_signature.signature.to_string())))
    }
}





pub struct EmailService;

impl IEmailService for EmailService {
    fn send_email(&self, email: Box<dyn IEmail>, email_service_provider: Box<dyn IEmailServiceProvider>) -> Result<(), Box<dyn std::error::Error>> {
        email_service_provider.deliver_mail(email)
    }
}


pub struct LoggerMailDummy;

impl IEmailServiceProvider for LoggerMailDummy {
    fn deliver_mail(&self, email: Box<dyn IEmail>) -> Result<(), Box<dyn std::error::Error>> {
        println!("Receiver: {} \nMessage: {:#?}", email.receiver().unwrap(), email.message().unwrap());
        Ok(())
    }
}
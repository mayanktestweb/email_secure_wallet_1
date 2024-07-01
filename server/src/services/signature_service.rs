use serde::{Deserialize, Serialize};

use crate::Result;




#[derive(Debug, Serialize, Deserialize)]
pub struct MsgToSign {
    msg: String,
    timestamp: u32
}

impl MsgToSign {
    pub fn new(msg: String, timestamp: u32) -> Self {
        MsgToSign {msg, timestamp}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureResponse {
    signature: String,
    msg: MsgToSign
}

impl SignatureResponse {
    pub fn new(signature: String, msg: MsgToSign) -> Self {
        SignatureResponse {signature, msg}
    }
}

pub fn sign_message(msg_to_sign: MsgToSign) -> Result<SignatureResponse> {
    let signature = format!("#@@@{}--{}###", msg_to_sign.msg, msg_to_sign.timestamp);
    Ok(SignatureResponse::new(signature, msg_to_sign))
}
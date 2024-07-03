#![allow(unused)] // in bigning only

use std::{io::Read, str::FromStr};

use alloy::signers::{k256::ecdsa::signature::{Keypair, SignerMut}, local::PrivateKeySigner, Signer};
use axum::{http::StatusCode, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use services::email_service;
use tokio::net::TcpListener;

pub use res_error::{Error, Result};
use web::{basic_routes, email_req_routes};

pub mod res_error;
pub mod web;
pub mod services;
pub mod utils;

#[tokio::main]
async fn main() { 
    
    let app = Router::new()
                        .merge(basic_routes::routes())
                        .merge(email_req_routes::routes())
                        .merge(temp_routes());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}


pub fn temp_routes() -> Router {
    Router::new().route("/key_gen", get(generate_key_pair))
        .route("/key_gen_by_priv_key", post(generate_key_pair_2))
}

pub async fn generate_key_pair() -> Result<StatusCode> {
    let random_signer = alloy::signers::local::LocalSigner::random();
    let pkey = random_signer.credential().to_bytes();
    println!("{:#?}", pkey);
    let address = random_signer.address();
    
    let mut address_bytes = vec![];

    for byt in address.bytes() {
        address_bytes.push(byt.unwrap());
    }

    println!("{:#?}", address_bytes);

    let v_key = random_signer.credential().verifying_key();
    let msg = "some_message".as_bytes();
    let signed_msg = random_signer.sign_message(msg);

    let x = signed_msg.await.unwrap();
    
    println!("{:#?}", x.inner().to_string());

    

    Ok(StatusCode::OK)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyCred {
    priv_key: String
}

pub async fn generate_key_pair_2(Json(key_cred): Json<KeyCred>) -> Result<StatusCode> {

    let signer = PrivateKeySigner::from_str(&key_cred.priv_key).unwrap();
    println!("{:#?}", signer.credential().to_bytes());

    let msg = "some_message".as_bytes();
    let signed_msg = signer.sign_message(msg);

    let x = signed_msg.await.unwrap();
    
    println!("{:#?}", x.inner().to_string());

    Ok(StatusCode::OK)
}
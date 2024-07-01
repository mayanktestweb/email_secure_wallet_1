#![allow(unused)] // in bigning only

use axum::Router;
use services::email_service;
use tokio::net::TcpListener;

pub use res_error::{Error, Result};
use web::{basic_routes, email_req_routes};

pub mod res_error;
pub mod web;
pub mod services;

#[tokio::main]
async fn main() { 
    
    let app = Router::new()
                        .merge(basic_routes::routes())
                        .merge(email_req_routes::routes());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
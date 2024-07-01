use axum::Router;
use tokio::net::TcpListener;

pub use res_error::{Error, Result};

mod res_error;
mod web;

#[tokio::main]
async fn main() { 
    
    let app = Router::new().merge(web::basic_routes::routes());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
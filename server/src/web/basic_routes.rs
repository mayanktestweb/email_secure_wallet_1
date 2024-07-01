use axum::{response::Html, routing::get, Router};

use crate::Result;

pub fn routes() -> Router {
    Router::new().route("/", get(welcome))
}

async fn welcome() -> Result<Html<String>> {
    Ok(Html(String::from("<p><b>Hello, World!</b></p>")))
}
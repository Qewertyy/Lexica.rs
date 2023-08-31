#![allow(non_snake_case,unused_imports,non_camel_case_types)]
use actix_files;
use actix_multipart::Multipart;
use actix_web::{delete, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,http::{StatusCode,Method,header::{self,ContentType}}, FromRequest};
use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};
use serde_json;
use std::*;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
    .content_type("text/html; charset=utf-8")
    .body("<h1>Yea, Its Alive.</h1>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

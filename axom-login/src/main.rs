#![allow(unused)]



use std::net::SocketAddr;

use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    
    let routes_hello = Router::new()
    .route("/hello",get(handler_hello))
    .route("/hello2/:name",get(handler_hello2));

    //region --- start server
    let addr= SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("->> Listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
    //endregion --- start server
    
}
// region: ---handler hello 
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse{        
    println!("->> {:<12} - handler_hello", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse{
    println!("->> {:<12} - handler_hello -{name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}
// endregion: ---handler hello

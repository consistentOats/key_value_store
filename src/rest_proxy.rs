use std::{net::SocketAddr, collections::HashMap};
use warp::{Filter, hyper::{Response, StatusCode}};
use clap::Parser;

#[derive(Parser)]
struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "8080")]
    port: u16,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ServerCli::parse();
    let address: SocketAddr = format!("{}:{}", cli.server, cli.port).parse()?;

    let put_item_route = warp::path!("store" / "item")
        .and(warp::path::end())
        .and(warp::put())
        .and(warp::body::json())
        .map(|body: HashMap<String,String>| put_item(body["key"].clone(), body["value"].clone()));
    
    let get_item_route = warp::path!("store" / "items" / String)
        .and(warp::path::end())
        .and(warp::get())
        .map(|key| get_item(key));

    let delete_item_route = warp::path!("store" / "items" / String)
        .and(warp::path::end())
        .and(warp::delete())
        .map(|key| delete_item(key));

    let routes = put_item_route
        .or(get_item_route)
        .or(delete_item_route);

    warp::serve(routes)
        .run(address)
        .await;
    
    Ok(())
}

// PUT "/store/item"
pub fn put_item(key: String, value: String) -> Result<warp::reply::Response, warp::http::Error> {
    ok_reply(format!("put_item called for '{}' : '{}' ", key, value))
}

// GET "/store/items/<:id>"
pub fn get_item(key: String) -> Result<warp::reply::Response, warp::http::Error> {
    ok_reply(format!("get_item called for '{}'", key))
}

// DELETE "/store/items/<:id>
pub fn delete_item(key: String) -> Result<warp::reply::Response, warp::http::Error> {
    ok_reply(format!("delete_item called for '{}'", key))
}

fn ok_reply(body: String) -> Result<warp::reply::Response, http::Error> {
    Response::builder()
        .status(StatusCode::OK)
        .body(body.into())
}

fn error_reply() -> Result<warp::reply::Response, http::Error> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("key not found".into())
}

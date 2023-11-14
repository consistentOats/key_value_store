use std::{net::SocketAddr, collections::HashMap};

use tonic;
use warp::{Filter, hyper::{Response, StatusCode}};
use serde_json::json;
use clap::Parser;

use key_value_protos::key_value_service_client::KeyValueServiceClient;
use key_value_protos::{KeyValuePair, PutItemRequest, 
    GetItemRequest, DeleteItemRequest};

pub mod key_value_protos {
    tonic::include_proto!("key_value_protos");
}

#[derive(Parser)]
struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "8080")]
    port: u16,
    #[arg(long = "backing-service-route")]
    backing_service_route: String,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ServerCli::parse();
    let address: SocketAddr = format!("{}:{}", cli.server, cli.port).parse()?;
    let rest_proxy = RestProxy::new(cli.backing_service_route);
    let rest_proxy_filter = warp::any().map(move || rest_proxy.clone());

    let put_item_route = warp::path!("store" / "item")
        .and(warp::path::end())
        .and(warp::put())
        .and(warp::body::json())
        .and(rest_proxy_filter.clone())
        .and_then(|body: HashMap<String,String>, rest_proxy: RestProxy| async move {rest_proxy.put_item(body["key"].clone(), body["value"].clone()).await});
    
    let get_item_route = warp::path!("store" / "items" / String)
        .and(warp::path::end())
        .and(warp::get())
        .and(rest_proxy_filter.clone())
        .and_then(|key, rest_proxy: RestProxy| async move {rest_proxy.get_item(key).await});

    let delete_item_route = warp::path!("store" / "items" / String)
        .and(warp::path::end())
        .and(warp::delete())
        .and(rest_proxy_filter)
        .and_then(|key, rest_proxy: RestProxy| async move {rest_proxy.delete_item(key).await});

    let routes = put_item_route
        .or(get_item_route)
        .or(delete_item_route);

    warp::serve(routes)
        .run(address)
        .await;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct RestProxy {
    key_value_service_route: String
}

impl RestProxy {
    pub fn new(key_value_service_route: String) -> Self {
        RestProxy { key_value_service_route }
    }

    // PUT "/store/item"
    pub async fn put_item(&self, key: String, value: String) -> Result<warp::reply::Response, warp::Rejection> {
        // intitialize the connection to the backing service
        let key_value_service_client = KeyValueServiceClient::connect(self.key_value_service_route.clone()).await;
        let mut key_value_service_client = match key_value_service_client {
            Ok(client) => {client},
            Err(e) => {return error_reply(format!("grpc client could not connect: {}", e.to_string()))},
        };

        // call put_item on the backing service with the provided parameters
        let response = key_value_service_client.put_item(tonic::Request::new(PutItemRequest {
            item: Some(KeyValuePair {
                key,
                value,
            })
        })).await;
        
        match response {
            Ok(response) => {
                if let Some(pair) = response.into_inner().updated_item {
                    // The backing service added the provided pair into the data store.
                    ok_reply(pair)
                } else {
                    // Error Case: (Internal Server Error) - the backing service responded with malformed data
                    error_reply("could not parse the server's response.".to_string())
                }
            }
            Err(status) => {
                // Error Case: (Internal Server Error) - the backing service produced an error code
                error_reply(status.to_string())
            }
        }
    }

    // GET "/store/items/<:id>"
    pub async fn get_item(&self, key: String) -> Result<warp::reply::Response, warp::Rejection> {
        // intitialize the connection to the backing service
        let key_value_service_client = KeyValueServiceClient::connect(self.key_value_service_route.clone()).await;
        let mut key_value_service_client = match key_value_service_client {
            Ok(client) => {client},
            Err(e) => {return error_reply(format!("grpc client could not connect: {}", e.to_string()))}
        };

        // call get_item on the backing service with the provided parameters
        let response = key_value_service_client.get_item(tonic::Request::new(GetItemRequest {key})).await;
        
        match response {
            Ok(response) => {
                if let Some(pair) = response.into_inner().item {
                    // The backing service returned a pair for the provided key.
                    ok_reply(pair)
                } else {
                    // Error Case: (Internal Server Error) - the backing service responded with malformed data
                    error_reply("could not parse the server's response.".to_string())
                }
            }
            Err(status) => {
                match status.code() {
                    tonic::Code::NotFound => {
                        // Error Case: (Not Found) - the backing service could not find the provided key in the data store
                        not_found_reply(status.message().to_string())
                    }
                    _ => {
                        // Error Case: (Internal Server Error) - the backing service produced an error code
                        error_reply(status.to_string())
                    }
                }
            }
        }
    }

    // DELETE "/store/items/<:id>
    pub async fn delete_item(&self, key: String) -> Result<warp::reply::Response, warp::Rejection> {
        // intitialize the connection to the backing service
        let key_value_service_client = KeyValueServiceClient::connect(self.key_value_service_route.clone()).await;
        let mut key_value_service_client = match key_value_service_client {
            Ok(client) => {client},
            Err(e) => {return error_reply(format!("grpc client could not connect: {}", e.to_string()))}
        };

        // call delete_item on the backing service with the provided parameters
        let response = key_value_service_client.delete_item(tonic::Request::new(DeleteItemRequest {key})).await;
        
        match response {
            Ok(response) => {
                if let Some(pair) = response.into_inner().item {
                    // The backing service returned the pair corresponding to the key.
                    ok_reply(pair)
                } else {
                    // Error Case: (Internal Server Error) - the backing service responded with malformed data
                    error_reply("could not parse the server's response.".to_string())
                }
            }
            Err(status) => {
                match status.code() {
                    tonic::Code::NotFound => {
                        // Error Case: (Not Found) - the backing service could not find the provided key in the data store
                        not_found_reply(status.message().to_string())
                    }
                    _ => {
                        // Error Case: (Internal Server Error) - the backing service produced an error code
                        error_reply(status.to_string())
                    }
                }
            }
        }
    }

}

fn ok_reply(pair: KeyValuePair) -> Result<warp::reply::Response, warp::Rejection> {
    Response::builder()
        .status(StatusCode::OK)
        .body(json!({pair.key: pair.value}).to_string().into())
        .map_err(|_| warp::reject())
}

fn not_found_reply(details: String) -> Result<warp::reply::Response, warp::Rejection>{
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(details.into())
        .map_err(|_| warp::reject())
}

fn error_reply(error: String) -> Result<warp::reply::Response, warp::Rejection> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(error.into())
        .map_err(|_| warp::reject())
}
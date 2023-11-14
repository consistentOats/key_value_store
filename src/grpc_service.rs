use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};
use clap::Parser;

use key_value_store::KeyValueStore;

use key_value_protos::key_value_service_server::{KeyValueService, KeyValueServiceServer};
use key_value_protos::{KeyValuePair, PutItemRequest, PutItemResponse, 
    GetItemRequest, GetItemResponse, DeleteItemRequest, DeleteItemResponse};

pub mod key_value_store;
pub mod key_value_protos {
    tonic::include_proto!("key_value_protos");
}

#[derive(Parser)]
struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ServerCli::parse();
    let address: SocketAddr = format!("{}:{}", cli.server, cli.port).parse()?;
    let key_value_store = KeyValueStore::new();
    let service = GrpcService::new(key_value_store);

    println!("Server listening on {}", address);

    Server::builder()
        .add_service(KeyValueServiceServer::new(service))
        .serve(address)
        .await?;

    Ok(())
}

#[derive(Debug)]
pub struct GrpcService {
    store: KeyValueStore,
}

impl GrpcService {
    pub fn new(store: KeyValueStore) -> Self {
        GrpcService {
            store
        }
    }
}

#[tonic::async_trait]
impl KeyValueService for GrpcService {

    async fn put_item(&self, request: Request<PutItemRequest>) -> Result<Response<PutItemResponse>, Status> {
        if let Some(pair) = request.into_inner().item {
            // insert the provided pair into the data store and return their values with a successful response
            let (key, value) = self.store.put(pair.key, pair.value);
            
            Ok(Response::new(
                PutItemResponse {
                    updated_item: Some(KeyValuePair {key, value})
                }
            ))
        } else {
            // Error case: malformed request
            Err(Status::invalid_argument("key-value pair could not be parsed from the provided PutItemRequest."))
        }
    }

    async fn get_item(&self, request: Request<GetItemRequest>) -> Result<Response<GetItemResponse>, Status> {
        let retrieved_pair = self.store.get(request.into_inner().key);
        
        match retrieved_pair {
            Ok((key, value)) => { 
                // the pair corresponding to the provided key is returned in the response
                Ok(Response::new(GetItemResponse {
                    item: Some(KeyValuePair {key, value})
                }))
            }
            Err(key_not_found_error) => {
                // Error case: key not found in the data store
                Err(Status::not_found(format!("key '{}' was not found in the associated data store.", key_not_found_error.key)))
            }
        }
    }

    async fn delete_item(&self, request: Request<DeleteItemRequest>) -> Result<Response<DeleteItemResponse>, Status> {
        let deleted_pair = self.store.delete(request.into_inner().key);
        
        match deleted_pair {
            Ok((key, value)) => { 
                // The deleted pair is returned in the response
                Ok(Response::new(DeleteItemResponse {
                    item: Some(KeyValuePair {key, value})
                }))
            }
            Err(key_not_found_error) => {
                // Error case: key not found in the data store
                Err(Status::not_found(format!("key '{}' was not found in the associated data store.", key_not_found_error.key)))
            }
        }
    }

}
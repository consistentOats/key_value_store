use std::net::SocketAddr;

use tonic::{transport::Server, Request, Response, Status};
use clap::Parser;

use key_value_protos::key_value_service_server::{KeyValueService, KeyValueServiceServer};
use key_value_protos::{KeyValuePair, PutItemRequest, PutItemResponse, 
    GetItemRequest, GetItemResponse, DeleteItemRequest, DeleteItemResponse};

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
    let service = Service::default();

    println!("Server listening on {}", address);

    Server::builder()
        .add_service(KeyValueServiceServer::new(service))
        .serve(address)
        .await?;

    Ok(())
}

#[derive(Debug, Default)]
pub struct Service {}

#[tonic::async_trait]
impl KeyValueService for Service {

    async fn put_item(&self, request: Request<PutItemRequest>) -> Result<Response<PutItemResponse>, Status> {
        println!("gRPC Server: Processing PutItemRequest: {:?}", request);

        let reply = PutItemResponse {
            updated_item: Some(KeyValuePair {
                key: "".to_string(),
                value: "".to_string(),
            })
        };

        Ok(Response::new(reply))
    }

    async fn get_item(&self, request: Request<GetItemRequest>) -> Result<Response<GetItemResponse>, Status> {
        println!("gRPC Server: Processing GetItemRequest: {:?}", request);

        let reply = GetItemResponse {
            item: Some(KeyValuePair {
                key: "".to_string(),
                value: "".to_string(),
            })
        };

        Ok(Response::new(reply))
    }

    async fn delete_item(&self, request: Request<DeleteItemRequest>) -> Result<Response<DeleteItemResponse>, Status> {
        println!("gRPC Server: Processing DeleteItemRequest: {:?}", request);

        let reply = DeleteItemResponse {
            item: Some(KeyValuePair {
                key: "".to_string(),
                value: "".to_string(),
            })
        };

        Ok(Response::new(reply))
    }

}
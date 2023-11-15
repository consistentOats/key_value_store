# key_value_store
This project contains a gRPC service used for accessing a simple data store, along with a REST proxy service used to access that gRPC service. 

## Running this project - Docker
### Pre-requisites

- [Install Docker](https://docs.docker.com/get-docker/)
- navigate to the root directory of this project

### Build and Run Steps

1. Run `docker network create key-value-store-network` - this creates a network for the two services to communicate
1. Run `docker build -t grpc-server --file grpc_server.Dockerfile .` - this creates the docker image for the gRPC service
1. Run `docker build -t rest-proxy --file rest_proxy.Dockerfile .` - this creates the docker image for the REST proxy
1. Run `docker run --net key-value-store-network --hostname grpcService -dp 127.0.0.1:50052:50052 grpc-server` - this runs the gRPC service and attaches it to the network (note: this service must keep the hostname flag as that is how the REST proxy finds it)
1. Run `docker run --net key-value-store-network -dp 127.0.0.1:8080:8080 rest-proxy` - this runs the REST proxy and attaches it to the network

## Running this project - Cargo
### Pre-requisites

1. [Install Rust](https://www.rust-lang.org/tools/install)
1. Run `rustup update`
1. navigate to the root directory of this project

### Build and Run Steps

1. Run `cargo build` to compile the project
1. Run `cargo run --bin grpc-server` to start running the gRPC server on port 50052
1. Run `cargo run --bin rest-proxy -- --backing-service-route=http://localhost:50052` to start running the REST proxy on port 8080

## Testing this project

To test this project, you can make requests to the REST proxy service either by using the included postman collections or by making curl requests to the following enpoints:

PUT localhost:8080/store/item (with the body {"key": <:key>, "value": <:value>})
> example curl command `curl -X PUT localhost:8080/store/item -H "Content-Type: application/json" -d '{"key": "hello", "value": "world"}'`

GET localhost:8080/store/items/<:key>
> example curl command `curl -X GET localhost:8080/store/items/hello`

DELETE localhost:8080/store/items/<:key>
> example curl command `curl -X DELETE localhost:8080/store/items/hello`


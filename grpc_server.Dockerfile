# syntax=docker/dockerfile:1

FROM rust:1.73
WORKDIR /grpc_server
COPY . .
RUN apt update && apt upgrade -y
RUN apt install -y protobuf-compiler libprotobuf-dev
RUN cargo build
CMD ["cargo", "run", "--bin", "grpc-server", "--", "--server=0.0.0.0"]
EXPOSE 50052

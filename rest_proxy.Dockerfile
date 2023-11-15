# syntax=docker/dockerfile:1

FROM rust:1.73
WORKDIR /rest_proxy
COPY . .
RUN apt update && apt upgrade -y
RUN apt install -y protobuf-compiler libprotobuf-dev
RUN cargo build
CMD ["cargo", "run", "--bin", "rest-proxy", "--", "--server=0.0.0.0", "--backing-service-route=http://grpcService:50052"]
EXPOSE 8080
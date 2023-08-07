VERSION 0.7

build:
  FROM rust:1.70
  WORKDIR /app
  ENV DEBIAN_FRONTEND=noninteractive
  RUN apt-get update && apt-get install -y libpq-dev libpq5
  RUN cargo install cargo-make
  COPY . .
  RUN cargo make install-clippy
	RUN cargo make all
  SAVE ARTIFACT target/release/worker /worker/bin

all:
  ARG image
  ARG tag
  BUILD +build
  FROM debian:bullseye-slim
  RUN apt-get install -y libpq5
  WORKDIR /app
  COPY +build/worker/bin .
  CMD ["/app/worker"]
  SAVE IMAGE --push $image:$tag

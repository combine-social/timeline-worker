VERSION 0.7

build:
  FROM rust:1.70
  WORKDIR /app
  COPY . .
  RUN cargo install cargo-make
	RUN cargo make all
  SAVE ARTIFACT target/release/worker /worker/bin

all:
  ARG image
  ARG tag
  BUILD +build
  FROM debian:bullseye-slim
  WORKDIR /app
  COPY +build/worker/bin .
  CMD ["/app/worker"]
  SAVE IMAGE --push $image:$tag

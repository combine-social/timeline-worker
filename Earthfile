VERSION 0.7

INSTALL_DEPS:
  COMMAND
  ENV DEBIAN_FRONTEND=noninteractive
  RUN apt-get update && \
    apt-get install -y libpq && \
    rm -rf /var/lib/apt/lists/*

build:
  FROM rust:1.69
  WORKDIR /app
  COPY . .
  DO +INSTALL_DEPS
  RUN cargo install cargo-make
	RUN cargo make all
  SAVE ARTIFACT target/release/worker /worker/bin

all:
  ARG image
  ARG tag
  BUILD +build
  FROM debian:bullseye-slim
  DO +INSTALL_DEPS
  WORKDIR /app
  COPY +build/worker/bin .
  CMD ["/app/worker"]
  SAVE IMAGE --push $image:$tag

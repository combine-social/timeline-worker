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
  RUN make dep
	RUN cargo test
	RUN cargo build --release
	RUN cargo strip
  SAVE ARTIFACT target/release/worker /worker/bin

all:
  BUILD +build
  FROM debian:bullseye-slim
  DO +INSTALL_DEPS
  WORKDIR /app
  COPY +build/worker/bin .
  CMD ["/app/worker"]
  SAVE IMAGE cyborch/toottail-worker:latest

# Worker for combine.social timeline

Combine.social offers an easy and simple solution to missing replies in your
home timeline or apparently empty profiles showing up in likes, boosts or
follow requests showing up in your notifications.

## Features

This worker does all the work of fetching reply urls, recent message urls for
users appearing in notifications, and making your instance fetch those messages.

Specifically, this means that:

- Replies to messages in your home timeline are fetched for up to 24 hours.
- Old messages are fetched for users that appear in notifications for up to 24 hours.

# Development

To get started, first install the build dependencies as described below.

## Build Dependencies

This project is built using [Cargo](https://doc.rust-lang.org/cargo/).

### Installation

To install Rust and Cargo, run (and follow the prompts):

```
brew install rustup-init
rustup-init
```

Build is performed using [cargo-make](https://crates.io/crates/cargo-make/):

```
cargo install cargo-make
```

To install the build dependencies, run:

```
cargo make dep
```

## Runtime Dependencies

State is managed using [PostgreSQL](https://www.postgresql.org) and
[Redis](https://redis.io), and queues are managed using
[RabbitMQ](https://www.rabbitmq.com).

Deployment can be done on any platform which has OCI image support. A
[docker compose](https://docs.docker.com/compose/) file is included for
reference.

## Linting and Testing

To lint and test, run:

```
cargo make lint
cargo make test
```

## Running the worker

To run locally, for development purposes, the worker can be started on its
own by running `cargo run`, but ususally, you will want to run all the services
it depends on. To start all services, run:

```
docker compose up
```

This will start the following services:

- Redis
- RabbitMQ, including [management UI](http://localhost:15672)
- PostgreSQL, listening on port 5432 for easy reference if you have a psql client avaiable.

## Building for production

**Caveat:** The build files assume that image is built and pushed to
`ghcr.io/combine-social/timeline-worker:latest`.

To build the images for production use, run:

```
cargo make image
```

The image name and tag can be overridden with:

```
cargo make --env IMAGE="my-image" --env TAG="1.0"
```

To build and push, run:

```
cargo make push
```

You can add the same `IMAGE` and `TAG` environment variables to `cargo make push`
as you can to `cargo make image`.

## Contributing

Combine.social is free, open-source software licensed under the [MIT license](LICENSE).

You can open issues for bugs you've found or features you think are missing.
You can also submit pull requests to this repository.

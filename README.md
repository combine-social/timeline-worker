# Worker for combine.social timeline

Combine.social offers an easy and simple solution to missing replies in your
home timeline or apparently empty profiles showing up in likes, boosts or
follow requests showing up in your notifications.

## Features

This worker does all the work of fetching reply urls, recent message urls for
users appearing in notifications, and making your instance fetch those messages.

Specifically, this means that:

 * Replies to messages in your home timeline are fetched for up to 24 hours.
 * Old messages are fetched for users that appear in notifications for up to 24 hours.

# Development

To get started, first install the build dependencies as described below.

## Build Dependencies

The front-end of this project is built using [Cargo](https://doc.rust-lang.org/cargo/).
Images are built using [Earthly](https://earthly.dev).

### Installation

To install Earthly run:

```
brew install earthly && earthly bootstrap
```

See installation instructions for other platforms at the Earthly
[Getting Started](https://earthly.dev/get-earthly) page.

To install Rust and Cargo, run (and follow the prompts):

```
brew install rustup-init
rustup-init
```

To install all dependencies and build, run make:

```
make
```

## Runtime Dependencies

State is managed using [PostgreSQL](https://www.postgresql.org) and
[Redis](https://redis.io), and queues are managed using
[RabbitMQ](https://www.rabbitmq.com).

Deployment can be done on any platform which has OCI image support. A 
[docker compose](https://docs.docker.com/compose/) file is included for
reference.

## Running the worker

To run locally, for development purposes, the worker can be started on its
own by running `cargo run`, but ususally, you will want to run all the services
it depends on. To start all services, run:

```
docker compose up
```

This will start the following services:

 * Redis
 * RabbitMQ, including [management UI](http://localhost:15672)
 * PostgreSQL, listening on port 5432 for easy reference if you have a psql client avaiable.

To test that everything is running smoothly, browse to [localhost:8080](http://localhost:8080).

## Building for production

**Caveat:** The build files assume that images are built and pushed to
`cyborch/toottail-<service_name>:latest`. The individual `Earthly` files can be
updated to reflect wherever you want to build and push images to.

To build the images for production use, run:

```
npm run docker:build
```

To build and push (assuming that you have access to a Docker Hub account and
are logged), run:

```
npm run docker:push
```

This will implicitly run `docker:build` if needed.

## Contributing

Combine.social is free, open-source software licensed under the [MIT license](LICENSE).

You can open issues for bugs you've found or features you think are missing.
You can also submit pull requests to this repository.

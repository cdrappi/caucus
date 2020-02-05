# Instructions
To use Rocket, you must use nightly builds of Rust.

## Build
```console
$ cargo build
```

## Database

### Install the diesel CLI with postgres
```console
$ cargo install diesel_cli --no-default-features --features postgres
```

### Migrate database
```console
$ diesel migration run
```

### Create a migration
```console
$ diesel migration generate users
```

## Server
```console
$ cargo run --release
```


# Docker
- For local development, set your database `host` from `localhost` to `docker.for.mac.host.internal`
- To build (from project root)
    ```sh
    $ docker build backend -t voters
    ```
- To run
    ```sh
    $ docker run -p 8000:8000 --env-file=backend/local-docker.env voters
    ```

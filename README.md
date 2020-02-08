# caucus
App for reporting caucus results

## Updating git submodules
```sh
$ git submodule update --init
```

# Tech stack
`caucus` is divided into 3 folders:

- donkey (a tool for deploys)
- clients (e.g. the web app)
- services (e.g. the web server)


## Deploy
To deploy the service named `api`, run:
```sh
$ deploy api
```

To deploy all services, run:
```sh
$ deploy
```

## Clients
This app gives you a web client:
- [Svelte 3](https://svelte.dev/)
- [webpack](https://webpack.js.org/)


## Services
This app gives you a Rust server:
- [Rust (nightly)](https://doc.rust-lang.org/1.2.0/book/nightly-rust.html)
- [Rocket](https://rocket.rs/)
- [diesel](http://diesel.rs/)

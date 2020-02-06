# caucus
App for organizing and deploying caucus strategy

## Updating git submodules
```sh
$ git submodule update --init
```

# Tech stack
`caucus` is divided into 3 folders:

- donkey (for deploys)
- clients (e.g. a mobile app)
- services (e.g. a web server)


## Deploy
To deploy the service named `backend`, run:
```sh
$ deploy backend
```

To deploy all services, run:
```sh
$ deploy
```

## Clients
This app gives you two clients:

### Mobile
- [React Native](https://facebook.github.io/react-native/)
- [expo](https://github.com/expo/expo)

### Web
- [Svelte 3](https://svelte.dev/)
- [webpack](https://webpack.js.org/)


## Services

This app gives you a Rust server:

### Backend
- [Rust (nightly)](https://doc.rust-lang.org/1.2.0/book/nightly-rust.html)
- [Rocket](https://rocket.rs/)
- [diesel](http://diesel.rs/)

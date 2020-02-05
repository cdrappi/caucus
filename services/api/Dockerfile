# From http://whitfin.io/speeding-up-rust-docker-builds/

# select build image
FROM rustlang/rust:nightly as build

# create a new empty shell project
RUN USER=root cargo new --bin voters
WORKDIR /voters

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/voters*
RUN cargo build --release

# our final base
FROM rustlang/rust:nightly

# copy the build artifact from the build stage
COPY --from=build /voters/target/release/voters .

# Finally copy over templates
COPY ./templates ./templates

EXPOSE 8000

# set the startup command to run your binary
CMD ["./voters"]

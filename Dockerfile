FROM rust:latest
RUN apt-get update && apt-get -y install make clang pkg-config libssl-dev
RUN rustup component add rustfmt
WORKDIR /usr/app/src
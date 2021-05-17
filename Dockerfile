FROM rust:latest as builder
WORKDIR /usr/app/src
COPY ./ /usr/app/src
RUN apt-get update && apt-get -y install clang && \
    rm -rf /var/lib/apt/lists/* && \
    rustup component add rustfmt 
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/rust/target \
    cargo build --release

FROM debian:stretch-slim as server
WORKDIR /root
COPY --from=builder /usr/app/src/target/release/indradb-server .
EXPOSE 27615
ENTRYPOINT ["./indradb-server"]

FROM debian:stretch-slim as client
WORKDIR /root
COPY --from=builder /usr/app/src/target/release/indradb-client .
ENTRYPOINT ["./indradb-client"]
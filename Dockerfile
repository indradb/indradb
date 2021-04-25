FROM rust:latest as builder
WORKDIR /usr/app/src
COPY ./ /usr/app/src
RUN apt-get update && apt-get -y install clang && \
    rm -rf /var/lib/apt/lists/* && \
    rustup component add rustfmt && \
    cargo build --lib --bins --release

FROM debian:stretch-slim
WORKDIR /root
COPY --from=builder /usr/app/src/target/release/indradb-server .
COPY --from=builder /usr/app/src/target/release/indradb-client .
CMD ["/bin/bash"] 


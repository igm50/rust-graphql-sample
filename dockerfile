FROM rust:1.41-buster

WORKDIR /usr/src/app

RUN USER=root cargo init --bin

ENV CARGO_TARGET_DIR=/tmp/target

COPY ./Cargo.toml Cargo.toml
COPY ./Cargo.lock Cargo.lock

RUN cargo build \
  && rm src/main.rs

EXPOSE 8000

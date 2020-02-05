FROM rust:1.41

RUN rustup default nightly \
  && rustup override set nightly

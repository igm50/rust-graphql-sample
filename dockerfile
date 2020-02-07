FROM rust:1.41

WORKDIR /usr/src/app

RUN rustup default nightly \
  && rustup override set nightly \
  && cargo install sccache

ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache

EXPOSE 8000

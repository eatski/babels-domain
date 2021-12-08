FROM rust:1.57.0-buster
WORKDIR /app
RUN rustup component add rls rust-analysis rust-src rustfmt
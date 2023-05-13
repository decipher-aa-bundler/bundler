FROM lukemathwalker/cargo-chef:latest-rust-1.69.0 AS chef

WORKDIR /app

RUN apt-get install wget -y
ADD https://github.com/ethereum/solidity/releases/download/v0.8.20/solc-static-linux /usr/bin/solc

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin bundler

FROM debian:buster-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/bundler /usr/local/bin

ENTRYPOINT ["/usr/local/bin/bundler"]



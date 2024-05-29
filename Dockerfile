FROM rust:1.78 AS builder
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
ARG BUILD_FLAGS
RUN if [ -z "$BUILD_FLAGS" ]; then cargo build --release; else cargo build --release --features ${BUILD_FLAGS}; fi


FROM ubuntu:22.04
WORKDIR /payment-app
RUN apt-get update && apt-get install -y \
    libpq-dev \
    openssl \
    ca-certificates \
    jq \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/payment .

ENTRYPOINT ["/payment-app/payment", "--config", "config.toml"]

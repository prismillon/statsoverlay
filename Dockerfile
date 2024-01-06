FROM rust:1.73.0 AS builder
RUN cargo new --bin app
WORKDIR /app
COPY Cargo.* ./
RUN cargo build --release
COPY src/*.rs ./src/.
COPY image image
COPY mini.html ./
RUN touch -a -m ./src/main.rs
RUN cargo build --release

FROM debian:stable-slim
RUN apt update && apt install -y openssl ca-certificates
WORKDIR /app
COPY --from=builder /app/target/release/statsoverlay /app/statsoverlay
CMD "/app/statsoverlay"



FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release 
FROM debian:bullseye-slim

WORKDIR  /app

COPY --from=builder /app/target/release/fibbot /app/fibbot

ENTRYPOINT ["/app/fibbot"]

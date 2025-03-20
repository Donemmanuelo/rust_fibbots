FROM rust:latest as builder

WORkDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release 
FROM debian:bullseye-slim

WORKDIR  /app

COPY --from=builder /app/target/release/fibb /app/fibb

ENTRYPOINT ["/app/fibb"]

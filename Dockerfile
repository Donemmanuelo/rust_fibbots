FROM rust:latest As builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:latest 

RUN apt-get update && apt-get install libssl-dev -y

COPY --from=builder /app/target/release/rust_fibbots /app/rust_fibbots

RUN chmod +x /app/rust_fibbots

ENTRYPOINT  ["/app/rust_fibbots"]



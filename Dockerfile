FROM rust:latest As builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:latest 

Run apt-get update && apt-get install libssl-dev -y

COPY --from=builder /app/target/release/fibbots /app/fibbots

ENTRYPOINT["./fibbots"]


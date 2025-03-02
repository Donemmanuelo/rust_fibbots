FROM rust:latest As builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:latest 

RUN apt-get update && apt-get install libssl-dev -y

COPY --from=builder /app/target/release/prime-number /app/prime-number

RUN chmod +x /app/prime-number

ENTRYPOINT ["/app/prime-number"]



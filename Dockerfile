FROM rust:latest As builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:latest 

RUN apt-get update && apt-get install libssl-dev -y

COPY --from=builder /app/target/release/fibbot /app/fibbot

RUN chmod +x /app/fibbot

ENTRYPOINT  ["/app/fibbot"]



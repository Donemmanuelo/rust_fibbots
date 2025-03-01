FROM rust:latest As builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:latest 

RUN apt-get update && apt-get install libssl-dev -y

RUN   export PR_NUMBER="${{ inputs.pr_number }}" && export GITHUB_TOKEN="${{ inputs.token }}"

COPY --from=builder /app/target/release/fibbot /app/fibbot

ENTRYPOINT ["/app/fibbot"]


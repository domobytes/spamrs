FROM rustlang/rust:nightly-slim as builder
WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y pkg-config libssl-dev
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/spamrs /usr/local/bin/
CMD ["spamrs"]
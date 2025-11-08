FROM rustlang/rust:nightly-slim as builder
WORKDIR /usr/src/app

# Install build dependencies early so this layer can be cached
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy only manifest files and fetch/build dependencies to leverage Docker layer cache.
# When only source files change, this layer will be reused and rebuild will be much faster.
COPY Cargo.toml Cargo.lock ./

# Create a tiny dummy main to allow cargo to compile dependencies.
RUN mkdir -p src && echo 'fn main() {println!("stub");}' > src/main.rs && \
	cargo build --release || true && \
	rm -rf src

# Now copy the full source and build the final binary.
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
# Runtime deps
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/spamrs /usr/local/bin/
CMD ["spamrs"]
# -------- Build Stage --------
FROM rust:1.81 as builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Copy actual source and build
COPY . .
RUN cargo build --release

# -------- Runtime Stage --------
FROM debian:bookworm-slim

# Install system dependencies if needed
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/push_notifications /usr/local/bin/push_notifications

# Run app
CMD ["push_notifications"]

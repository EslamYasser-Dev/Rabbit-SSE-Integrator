# Stage 1: Build
FROM rust:1.81 as builder

WORKDIR /usr/src/app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source and build
COPY . .
RUN cargo build --release

# Stage 2: Runtime (minimal image)
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled binary
COPY --from=builder /usr/src/app/target/release/rabbit-sse-integrator /app/rabbit-sse-integrator

# Expose port
EXPOSE 8080

# Run the binary
CMD ["./rabbit-sse-integrator"]

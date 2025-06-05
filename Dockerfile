# Use Rust as base for both build and runtime
FROM rust:1.78-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    libsqlite3-dev \
    pkg-config \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage uses the same image family to avoid GLIBC issues
FROM rust:1.78-slim

# Install only runtime dependencies
RUN apt-get update && apt-get install -y \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-jwt-proxy /usr/local/bin/rust-jwt-proxy

VOLUME /data

EXPOSE 3000

CMD ["rust-jwt-proxy"]
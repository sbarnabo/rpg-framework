# ---------- Build Stage ----------
FROM rust:1.75 as builder

# Install cargo-watch
RUN cargo install cargo-watch

# Create app directory
WORKDIR /app

# Copy manifests first (caching dependencies)
COPY Cargo.toml .
COPY Cargo.lock .

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Prebuild dependencies
RUN cargo build --release
RUN rm -f src/main.rs

# Copy the full source code
COPY . .

# Final build (optional in dev)
RUN cargo build --release

# ---------- Runtime Stage ----------
FROM debian:bullseye-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m appuser

# Copy binary and source for dev
COPY --from=builder /app /app
COPY --from=builder /usr/local/cargo/bin/cargo-watch /usr/local/bin/cargo-watch

# Set permissions
RUN chown -R appuser:appuser /app

# Use non-root user
USER appuser

# Set working directory
WORKDIR /app

# Expose backend port
EXPOSE 8080

# Default to cargo-watch for dev
CMD ["cargo-watch", "-x", "run"]

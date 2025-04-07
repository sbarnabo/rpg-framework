# ---------- Build Stage ----------
FROM rust:1.75 as builder

# Create app directory
WORKDIR /app

# Copy manifests first (caching dependencies)
COPY Cargo.toml .
COPY Cargo.lock .

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release
RUN rm -f src/main.rs

# Copy the rest of the source
COPY . .

# Build the actual app
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

# Copy the binary from the builder
COPY --from=builder /app/target/release/text_rpg_backend /usr/local/bin/text_rpg_backend

# Copy content directory (or mount as volume)
COPY --from=builder /app/content /opt/text_rpg/content

# Set permissions
RUN chown -R appuser:appuser /opt/text_rpg

# Use non-root user
USER appuser

# Set working directory
WORKDIR /opt/text_rpg

# Expose backend port
EXPOSE 8080

# Start the backend
CMD ["text_rpg_backend"]

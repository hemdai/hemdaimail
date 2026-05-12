# Stage 1: Build
FROM rust:1.95-slim-bookworm AS builder

WORKDIR /usr/src/api

# Install build dependencies
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

# Copy manifests and sqlx data
COPY Cargo.toml ./
COPY Cargo.lock* ./
COPY .sqlx* ./.sqlx/

# Pre-build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    SQLX_OFFLINE=true cargo build --release

# Copy the actual source and build
COPY . .
RUN rm src/main.rs && SQLX_OFFLINE=true cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m appuser
USER appuser

WORKDIR /app
COPY --from=builder /usr/src/api/target/release/api /app/api
COPY --from=builder /usr/src/api/migrations /app/migrations

# Set production envs
ENV RUST_LOG=info
EXPOSE 4000

CMD ["./api"]

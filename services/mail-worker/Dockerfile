# Stage 1: Build
FROM rust:1.95-slim-bookworm AS builder

WORKDIR /usr/src/mail-worker

# Install build dependencies
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

# Copy manifests and sqlx data
COPY Cargo.toml Cargo.lock* ./
COPY .sqlx ./.sqlx/

# Pre-build dependencies
RUN mkdir src && echo "fn main() { panic!(\"dummy binary\"); }" > src/main.rs && \
    SQLX_OFFLINE=true CARGO_BUILD_JOBS=1 cargo build --release

# Copy the actual source code
COPY src ./src
RUN mkdir -p migrations
COPY migrations ./migrations

# Build the real binary
RUN SQLX_OFFLINE=true CARGO_BUILD_JOBS=1 cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m appuser
USER appuser

WORKDIR /app
COPY --from=builder /usr/src/mail-worker/target/release/mail-worker /app/mail-worker

# Set production envs
ENV RUST_LOG=info
EXPOSE 3001

CMD ["./mail-worker"]

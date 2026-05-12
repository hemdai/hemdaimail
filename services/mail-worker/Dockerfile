# Stage 1: Build
FROM rust:1.95-slim-bookworm AS builder

WORKDIR /usr/src/mail-worker

# Copy manifests first to cache dependency downloads
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# Copy the actual source code
COPY src ./src
RUN mkdir -p migrations
COPY migrations ./migrations

# Build the real binary
RUN rm src/main.rs && cargo build --release

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

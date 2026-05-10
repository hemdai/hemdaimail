# Stage 1: Build
FROM rust:1.95-slim-bookworm AS builder

WORKDIR /usr/src/search-worker
COPY . .

# Build for release
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m appuser
USER appuser

WORKDIR /app
COPY --from=builder /usr/src/search-worker/target/release/search-worker /app/search-worker

# Set production envs
ENV RUST_LOG=info
EXPOSE 3002

CMD ["./search-worker"]

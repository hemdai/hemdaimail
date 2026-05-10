# Hemdaimail

Hemdaimail is a production-grade, open-source webmail platform designed for speed, scalability, and deep synchronization with standard IMAP/SMTP services. It features a modern, Gmail-inspired interface built with Next.js and a high-performance backend powered by Rust.

## 🚀 Key Features

- **Blazing Fast Sync:** Incremental IMAP synchronization using UIDNEXT/MODSEQ.
- **Gmail-like UX:** Three-pane dynamic layout with real-time updates.
- **Advanced Threading:** Automated conversation reconstruction using email headers.
- **Real-time Notifications:** WebSocket-based event broadcasting via Redis Pub/Sub.
- **Full-Text Search:** Near-instant email search powered by Meilisearch.
- **Production Ready:** Optimized Docker containers and Kubernetes orchestration.
- **Observability:** Built-in distributed tracing (OpenTelemetry) and Prometheus metrics.
- **Security:** Argon2 password hashing, JWT rotation, and mandatory HTML sanitization.

## 🏗 Architecture

Hemdaimail follows a **Modular Monolith** architecture in Rust, designed to scale into microservices when needed.

- **API Gateway (Rust/Axum):** Handles all client requests and WebSocket persistent connections.
- **Mail Worker (Rust/Tokio):** Manages the heavy lifting of background IMAP sync, MIME parsing, and S3 storage.
- **Search Worker (Rust/Tokio):** Processes parsed content and indexes it for full-text search.
- **Database (PostgreSQL):** Stores relational metadata, users, folders, and message headers.
- **Object Storage (MinIO/S3):** Stores raw email blobs and attachments.
- **Queue/Cache (Redis):** Orchestrates background tasks and real-time event broadcasting.

## 🛠 Tech Stack

- **Backend:** Rust, Axum, Tokio, SQLx, Lettre, async-imap.
- **Frontend:** Next.js 14 (App Router), TypeScript, Tailwind CSS, TipTap, Lucide Icons.
- **Infrastructure:** Docker, Kubernetes, Nginx.
- **Data:** PostgreSQL, Redis, Meilisearch, MinIO.

## 🚦 Getting Started

### Prerequisites

- Docker & Docker Compose
- Rust (v1.95+)
- Node.js (v20+)

### Quick Start (Local Development)

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/hemdai/hemdaimail.git
    cd hemdaimail
    ```

2.  **Start the infrastructure:**
    ```bash
    docker compose -f infrastructure/docker/docker-compose.yml up -d
    ```

3.  **Run migrations:**
    ```bash
    cd services/api
    sqlx database create
    sqlx migrate run
    ```

4.  **Launch the backend services:**
    ```bash
    # In separate terminals:
    cargo run --bin api
    cargo run --bin mail-worker
    cargo run --bin search-worker
    ```

5.  **Start the frontend:**
    ```bash
    cd apps/web
    npm install
    npm run dev
    ```

## 📄 Documentation

- [Detailed Architecture](./docs/architecture.md)
- [Mail Worker Design](./docs/mail-worker-design.md)
- [Production Deployment Strategy](./docs/production-readiness.md)

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for more details.

## 🛡 Security

If you discover a security vulnerability, please let me know.

## ⚖️ License

Hemdaimail is released under the MIT License.

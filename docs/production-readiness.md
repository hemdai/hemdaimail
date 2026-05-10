# Production Readiness Strategy

## 1. Security Hardening
- **Secrets Management:** Use Kubernetes Secrets or Vault. Never commit `.env` files.
- **Network Isolation:** Only the API service is exposed via Ingress. Workers and DBs are in private subnets.
- **HTML Sanitization:** All incoming emails are sanitized via the `ammonia` engine to prevent XSS.

## 2. Horizontal Scaling
- **Stateful vs Stateless:** All application services (API, Workers, Frontend) are stateless and can be scaled horizontally.
- **WebSocket Scaling:** Synchronization is handled via Redis Pub/Sub, allowing users on different API instances to receive notifications.

## 3. Storage & Persistence
- **Database:** PostgreSQL for relational data. Recommended: Managed service with point-in-time recovery.
- **Blob Storage:** Raw MIME and attachments stored in S3/MinIO.
- **Search:** Meilisearch for high-speed indexing.

## 4. Observability
- **Distributed Tracing:** OpenTelemetry OTLP spans emitted by all Rust services.
- **Metrics:** Prometheus endpoints at `/metrics`.
- **Health Checks:** Liveness and Readiness probes at `/health`.

## 5. Deployment Lifecycle
- **CI:** Automated testing and Docker image publishing via GitHub Actions.
- **CD:** Zero-downtime rolling updates on Kubernetes.

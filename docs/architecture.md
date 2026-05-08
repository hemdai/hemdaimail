# Webmail Platform System Architecture

## Overview
The platform uses a modular monolith approach in Rust, designed for future migration to microservices.

### Services
1.  **API Gateway (Axum):** Handles all HTTP/REST/WebSocket traffic.
2.  **Mail Worker (Tokio):** Handles SMTP/IMAP background processing.
3.  **Search Worker:** Synchronizes emails with Meilisearch/OpenSearch.
4.  **Database (PostgreSQL):** Stores relational metadata (users, folders, message headers).
5.  **Storage (MinIO/S3):** Stores raw MIME blobs and attachments.
6.  **Cache/Queue (Redis):** Handles background job queues and session/cache management.

## Data Flow
- **Inbound:** SMTP/IMAP -> Mail Worker -> (Parsed) -> PostgreSQL (Metadata) / S3 (Blob) -> Search Worker -> Meilisearch.
- **Outbound:** User -> API -> PostgreSQL (Draft/Queue) -> Mail Worker -> SMTP.

---

# Initial Database Schema (PostgreSQL)

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    domain_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE domains (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT UNIQUE NOT NULL,
    verification_token TEXT UNIQUE,
    verified BOOLEAN DEFAULT FALSE
);

CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    message_id TEXT NOT NULL,
    subject TEXT,
    sender TEXT,
    recipients TEXT[],
    body_summary TEXT,
    s3_path TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Additional tables: folders, labels, attachments, threads, filters...
```

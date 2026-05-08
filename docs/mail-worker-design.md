# Mail Worker Architectural Design

## 1. IMAP Synchronization Flow
The goal is to keep the local database in sync with the remote IMAP server with minimal data transfer.

### Initial Sync (Cold Start)
1.  **Discovery:** Authenticate and list all mailboxes (folders).
2.  **Metadata Sync:** For each mailbox, fetch UID list and basic headers (Message-ID, Subject, Date, From, To).
3.  **Content Sync:** Fetch full MIME bodies for messages not yet in S3/Database.
4.  **Indexing:** Parse bodies and push to Search Index (Meilisearch).

### Incremental Sync (Periodic/Triggered)
1.  **Check UIDNEXT/HIGHESTMODSEQ:** Quickly determine if a mailbox has changed.
2.  **Fetch New:** Fetch messages with UIDs greater than the last synced UID.
3.  **Detect Deleted:** Compare local UID list with remote UID list to identify and remove deleted messages.
4.  **Detect Flag Changes:** Fetch flags for all UIDs (or using MODSEQ if supported) to sync Read/Starred/Deleted states.

## 2. Mail Processing Lifecycle
1.  **Fetch:** Raw MIME downloaded via IMAP `FETCH`.
2.  **Deduplicate:** Check `Message-ID` header. If exists, link to existing `Message` record (multi-folder support).
3.  **Store Blob:** Upload raw MIME to S3-compatible storage (MinIO).
4.  **Parse:** Extract Subject, Snippet, Body (HTML/Plain), and Attachments.
5.  **Thread:** Use `In-Reply-To` and `References` headers to link the message to a `Thread`.
6.  **Index:** Send parsed text to Search Worker queue.

## 3. Database Schema Refinement
### Thread Reconstruction
We use a `threads` table. Messages are linked to threads via `thread_id`.
Reconstruction logic:
1.  Check `In-Reply-To`. If it matches a `message_id` already in the DB, use its `thread_id`.
2.  Check `References`. Scan all IDs in the list. Use the first found `thread_id`.
3.  If no match, create a new `thread_id`.

### Deduplication
Messages are stored once as "Blobs" in S3. The `messages` table tracks the presence of a message in a specific `mailbox_id`.

## 4. Queue & Worker Orchestration (Redis)
-   **Queue: `mail_sync_tasks`**: Contains jobs like `{ user_id: UUID, mailbox: String, sync_type: "full" | "incremental" }`.
-   **Queue: `search_indexing_tasks`**: Parsed content ready for indexing.
-   **Cron:** Periodic trigger for incremental sync of active users.

## 5. Attachment Strategy
-   Attachments are extracted and stored separately in S3: `attachments/{message_uuid}/{filename}`.
-   Database tracks metadata (size, content_type) for quick UI rendering without fetching the blob.

## 6. Failure Handling
-   **Exponential Backoff:** For IMAP connection failures.
-   **Dead Letter Queue (DLQ):** For messages that consistently fail to parse.
-   **Transactional Integrity:** Use SQL transactions for database updates to ensure metadata and UID state remain consistent.

-- Add sync state to mailboxes
ALTER TABLE mailboxes ADD COLUMN last_uid_validity BIGINT;
ALTER TABLE mailboxes ADD COLUMN last_uid_next BIGINT;
ALTER TABLE mailboxes ADD COLUMN last_modseq BIGINT;

-- Improve messages table for threading and deduplication
ALTER TABLE messages ADD COLUMN remote_uid BIGINT;
ALTER TABLE messages ADD COLUMN message_id TEXT; -- The Message-ID header
ALTER TABLE messages ADD COLUMN in_reply_to TEXT;
ALTER TABLE messages ADD COLUMN references_list TEXT[];
ALTER TABLE messages ADD COLUMN headers JSONB;

-- Ensure message_id indexing for deduplication and threading
CREATE INDEX idx_messages_message_id ON messages(message_id);
CREATE INDEX idx_messages_in_reply_to ON messages(in_reply_to);

-- Add unique constraint to prevent duplicate sync of same message in same mailbox
CREATE UNIQUE INDEX idx_unique_message_mailbox ON messages(mailbox_id, remote_uid);

-- Table for user IMAP credentials (encrypted at rest in production)
CREATE TABLE user_imap_credentials (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    host TEXT NOT NULL,
    port INTEGER NOT NULL,
    username TEXT NOT NULL,
    password_encrypted TEXT NOT NULL,
    use_tls BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

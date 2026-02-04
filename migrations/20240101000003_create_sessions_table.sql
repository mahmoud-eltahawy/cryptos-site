-- Create sessions table for tower-sessions
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY NOT NULL,
    data BYTEA NOT NULL,
    expiry_date TIMESTAMPTZ NOT NULL
);

-- Create index on expiry_date for efficient cleanup
CREATE INDEX IF NOT EXISTS idx_sessions_expiry ON sessions(expiry_date);

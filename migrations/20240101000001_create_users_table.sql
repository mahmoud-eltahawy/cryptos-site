-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    password TEXT NOT NULL,
    level TEXT NOT NULL CHECK (level IN ('Admin', 'User')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on name for faster lookups
CREATE INDEX IF NOT EXISTS idx_users_name ON users(name);

-- Create index on level for role-based queries
CREATE INDEX IF NOT EXISTS idx_users_level ON users(level);

-- Insert default admin user (password: "admin123")
-- Password hash generated with password-auth
INSERT INTO users (id, name, password, level) VALUES
    ('00000000-0000-0000-0000-000000000001', 'admin', '$argon2id$v=19$m=19456,t=2,p=1$d2xhxafgf42yOaQfX3hwQQ$UxypuGP0fwpcl0KqBhpl79aeDZ/n0MUYHRIor0kb/IU', 'Admin')
ON CONFLICT (name) DO NOTHING;

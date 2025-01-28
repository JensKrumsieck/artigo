CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    bio TEXT NOT NULL DEFAULT '',
    image TEXT,
    password TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

SELECT trigger_updated_at('"users"');
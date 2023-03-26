CREATE TABLE IF NOT EXISTS users (
    id              TEXT PRIMARY KEY UNIQUE NOT NULL, 
    name            TEXT                    NOT NULL
);

CREATE TABLE IF NOT EXISTS messages (
    message_id      TEXT                    NOT NULL, 
    author_id       TEXT                    NOT NULL, 
    chat_id         TEXT                    NOT NULL, 
    content         TEXT
);

CREATE TABLE IF NOT EXISTS chats (
    chat_id         TEXT PRIMARY KEY UNIQUE NOT NULL, 
    sender_id       TEXT, 
    recipient_id    TEXT, 
    server_id       TEXT,
    name            TEXT                    NOT NULL
);

CREATE TABLE IF NOT EXISTS servers (
    id              TEXT PRIMARY KEY UNIQUE NOT NULL, 
    name            TEXT                    NOT NULL
);

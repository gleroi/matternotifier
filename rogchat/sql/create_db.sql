CREATE TABLE IF NOT EXISTS posts (
    id TEXT PRIMARY KEY NOT NULL,
    create_at INTEGER,
    update_at INTEGER,
    delete_at INTEGER,
    edit_at INTEGER,
    user_id TEXT NOT NULL,
    channel_id TEXT NOT NULL,
    root_id TEXT,
    parent_id TEXT NOT NULL,
    original_id TEXT NOT NULL,
    message TEXT,
    post_type TEXT
  );

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    create_at INTEGER,
    update_at INTEGER,
    delete_at INTEGER,
    username TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    nickname TEXT NOT NULL,
    email TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS channels (
    id TEXT PRIMARY KEY NOT NULL,
    create_at INTEGER,
    update_at INTEGER,
    delete_at INTEGER,
    team_id TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    display_name TEXT NOT NULL,
    name TEXT NOT NULL,
    header TEXT NOT NULL,
    purpose TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS teams (
    id TEXT PRIMARY KEY NOT NULL,
    create_at INTEGER,
    update_at INTEGER,
    delete_at INTEGER,
    display_name TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    email TEXT NOT NULL
);


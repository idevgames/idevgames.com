-- Your SQL goes here
CREATE TABLE users(
    id INTEGER PRIMARY KEY NOT NULL,
    preferred_name TEXT NOT NULL
);

CREATE TABLE github_user_records(
    id BIGINT PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    login TEXT NOT NULL,
    avatar_url TEXT NOT NULL,
    html_url TEXT NOT NULL
);

CREATE TABLE permissions(
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE snippets(
    id INTEGER PRIMARY KEY NOT NULL,
    creator_id INTEGER NOT NULL,

    taxonomy TEXT NOT NULL,
    hidden BOOLEAN NOT NULL DEFAULT false,

    title TEXT NOT NULL,
    icon TEXT,
    shared_by TEXT NOT NULL,
    shared_on TIMESTAMP NOT NULL,
    summary TEXT NOT NULL,
    description TEXT NOT NULL,
    href TEXT NOT NULL,

    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
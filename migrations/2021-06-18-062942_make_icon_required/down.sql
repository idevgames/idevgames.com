CREATE TABLE snippets2(
    id INTEGER PRIMARY KEY NOT NULL,
    creator_id INTEGER NOT NULL,

    taxonomy TEXT NOT NULL,
    hidden BOOLEAN NOT NULL DEFAULT false,

    title TEXT NOT NULL,
    icon TEXT, -- changing this colum to be nullable
    shared_by TEXT NOT NULL,
    shared_on TIMESTAMP NOT NULL,
    summary TEXT NOT NULL,
    description TEXT NOT NULL,
    href TEXT NOT NULL,

    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

INSERT INTO snippets2 SELECT * FROM snippets;
DROP TABLE snippets;
ALTER TABLE snippets2 RENAME TO snippets;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS tasks
(
    id         uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    title      VARCHAR(255)     NOT NULL,
    content    TEXT             NOT NULL,
    status     VARCHAR(50)      NOT NULL,
    priority   VARCHAR(50)      NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE  DEFAULT timezone('utc'::text, now()),
    updated_at TIMESTAMP WITH TIME ZONE  DEFAULT timezone('utc'::text, now())
);
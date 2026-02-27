-- Add migration script here
CREATE TABLE IF NOT EXISTS tags (
                                    id BIGSERIAL PRIMARY KEY,
                                    name TEXT NOT NULL UNIQUE,
                                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS post_tags (
                                         post_id BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
                                         tag_id BIGINT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
                                         created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                         PRIMARY KEY (post_id, tag_id)
);

CREATE INDEX IF NOT EXISTS idx_post_tags_tag_id ON post_tags(tag_id);

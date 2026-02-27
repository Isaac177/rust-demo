INSERT INTO users (email, full_name)
VALUES
    ('alice@example.com', 'Alice Johnson'),
    ('bob@example.com', 'Bob Smith'),
    ('carol@example.com', 'Carol Davis')
ON CONFLICT (email) DO NOTHING;

INSERT INTO posts (user_id, title, body, published)
SELECT u.id, p.title, p.body, p.published
FROM (
    VALUES
        ('alice@example.com', 'Getting Started with Axum', 'Axum is fast and ergonomic for building APIs in Rust.', true),
        ('bob@example.com', 'SQLx Migrations in Practice', 'Use versioned SQL migrations for reproducible schema changes.', true),
        ('carol@example.com', 'Draft: Building a Comments API', 'This is still a draft and not published yet.', false)
) AS p(email, title, body, published)
JOIN users u ON u.email = p.email
ON CONFLICT DO NOTHING;

INSERT INTO comments (post_id, user_id, body)
SELECT p.id, u.id, c.body
FROM (
    VALUES
        ('Getting Started with Axum', 'bob@example.com', 'Nice intro. Looking forward to part 2.'),
        ('Getting Started with Axum', 'carol@example.com', 'Can you add an example with shared app state?'),
        ('SQLx Migrations in Practice', 'alice@example.com', 'This helped me avoid schema drift, thanks.')
) AS c(post_title, email, body)
JOIN posts p ON p.title = c.post_title
JOIN users u ON u.email = c.email
ON CONFLICT DO NOTHING;

INSERT INTO tags (name)
VALUES
    ('rust'),
    ('axum'),
    ('sqlx'),
    ('postgres')
ON CONFLICT (name) DO NOTHING;

INSERT INTO post_tags (post_id, tag_id)
SELECT p.id, t.id
FROM (
    VALUES
        ('Getting Started with Axum', 'rust'),
        ('Getting Started with Axum', 'axum'),
        ('SQLx Migrations in Practice', 'rust'),
        ('SQLx Migrations in Practice', 'sqlx'),
        ('SQLx Migrations in Practice', 'postgres')
) AS pt(post_title, tag_name)
JOIN posts p ON p.title = pt.post_title
JOIN tags t ON t.name = pt.tag_name
ON CONFLICT (post_id, tag_id) DO NOTHING;

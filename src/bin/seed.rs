use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Debug, Clone, Copy)]
struct SeedConfig {
    users: i64,
    posts_per_user: i64,
    comments_per_post: i64,
    truncate: bool,
}

impl Default for SeedConfig {
    fn default() -> Self {
        Self {
            users: 1_000,
            posts_per_user: 5,
            comments_per_post: 3,
            truncate: false,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = parse_args()?;
    let database_url = resolve_database_url()?;
    let pool = connect_pool(&database_url).await?;

    run_seed(&pool, config).await?;

    println!(
        "Seed completed: users={}, posts_per_user={}, comments_per_post={}, truncate={}",
        config.users, config.posts_per_user, config.comments_per_post, config.truncate
    );

    Ok(())
}

fn parse_args() -> anyhow::Result<SeedConfig> {
    let mut cfg = SeedConfig::default();
    let args: Vec<String> = env::args().collect();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--users" => {
                cfg.users = parse_i64_arg(&args, i, "--users")?;
                i += 2;
            }
            "--posts-per-user" => {
                cfg.posts_per_user = parse_i64_arg(&args, i, "--posts-per-user")?;
                i += 2;
            }
            "--comments-per-post" => {
                cfg.comments_per_post = parse_i64_arg(&args, i, "--comments-per-post")?;
                i += 2;
            }
            "--truncate" => {
                cfg.truncate = true;
                i += 1;
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            arg => {
                anyhow::bail!("Unknown argument: {arg}");
            }
        }
    }

    if cfg.users <= 0 || cfg.posts_per_user <= 0 || cfg.comments_per_post <= 0 {
        anyhow::bail!("All numeric options must be > 0");
    }

    Ok(cfg)
}

fn parse_i64_arg(args: &[String], index: usize, flag: &str) -> anyhow::Result<i64> {
    let Some(raw) = args.get(index + 1) else {
        anyhow::bail!("Missing value for {flag}");
    };

    raw.parse::<i64>()
        .map_err(|e| anyhow::anyhow!("Invalid value for {flag}: {raw} ({e})"))
}

fn print_help() {
    println!("Usage: cargo run --bin seed -- [options]");
    println!("Options:");
    println!("  --users <N>                 Number of users (default: 1000)");
    println!("  --posts-per-user <N>        Posts per user (default: 5)");
    println!("  --comments-per-post <N>     Comments per post (default: 3)");
    println!("  --truncate                  Truncate data tables before seeding");
}

fn resolve_database_url() -> anyhow::Result<String> {
    if let Ok(url) = env::var("DATABASE_URL") {
        return Ok(url);
    }

    if let Ok(url) = env::var("APP__DATABASE__URL") {
        return Ok(url);
    }

    anyhow::bail!("DATABASE_URL (or APP__DATABASE__URL) is required");
}

async fn connect_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}

async fn run_seed(pool: &PgPool, cfg: SeedConfig) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    if cfg.truncate {
        sqlx::query(
            "TRUNCATE TABLE post_tags, comments, posts, tags, users RESTART IDENTITY CASCADE",
        )
        .execute(&mut *tx)
        .await?;
    }

    let batch = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

    sqlx::query("CREATE TEMP TABLE _seed_users (id BIGINT) ON COMMIT DROP")
        .execute(&mut *tx)
        .await?;

    sqlx::query("CREATE TEMP TABLE _seed_posts (id BIGINT, user_id BIGINT) ON COMMIT DROP")
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        r#"
        WITH inserted AS (
            INSERT INTO users (email, full_name)
            SELECT
                format('seed_%s_user_%s@example.com', $1, gs),
                format('Seed Batch %s User %s', $1, gs)
            FROM generate_series(1, $2) AS gs
            RETURNING id
        )
        INSERT INTO _seed_users (id)
        SELECT id FROM inserted
        "#,
    )
    .bind(batch)
    .bind(cfg.users)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        WITH inserted AS (
            INSERT INTO posts (user_id, title, body, published)
            SELECT
                su.id,
                format('Seed %s Post %s for User %s', $1, p.idx, su.id),
                format('Generated body for seed batch %s, post %s', $1, p.idx),
                (random() > 0.2)
            FROM _seed_users su
            CROSS JOIN generate_series(1, $2) AS p(idx)
            RETURNING id, user_id
        )
        INSERT INTO _seed_posts (id, user_id)
        SELECT id, user_id FROM inserted
        "#,
    )
    .bind(batch)
    .bind(cfg.posts_per_user)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO comments (post_id, user_id, body)
        SELECT
            sp.id,
            sp.user_id,
            format('Seed %s comment %s on post %s', $1, c.idx, sp.id)
        FROM _seed_posts sp
        CROSS JOIN generate_series(1, $2) AS c(idx)
        "#,
    )
    .bind(batch)
    .bind(cfg.comments_per_post)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

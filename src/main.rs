use demo::{
    app::{router::create_router, state::AppState},
    config::Settings,
    db::{migrate::run as run_migrations, pool::new_pool},
    observability,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::from_env()?;

    observability::tracing::init();

    let db_pool = new_pool(
        &settings.database.url,
        settings.database.max_connections,
    ).await?;

    run_migrations(&db_pool).await?;

    let app_state = AppState {
        settings: settings.clone(),
        db_pool
    };

    let app = create_router(app_state);

    let address = format!("{}:{}", settings.server.host, settings.server.port);
    let listener = TcpListener::bind(&address).await?;

    tracing::info!("Listening on {}", address);

    axum::serve(listener, app).await?;

    Ok(())
}
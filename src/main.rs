use demo::{
    app::{router::create_router, state::AppState},
    config::Settings,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::from_env()?;
    let app_state = AppState {
        settings: settings.clone(),
    };

    let app = create_router(app_state);

    let address = format!("{}:{}", settings.server.host, settings.server.port);
    let listener = TcpListener::bind(&address).await?;

    println!("Listening on {}", address);

    axum::serve(listener, app).await?;

    Ok(())
}
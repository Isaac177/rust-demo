use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init() {
    let filter = EnvFilter::new("info,tower_http=info");

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer()
            //.json()
            .with_writer(std::io::stdout)
            .with_target(true)
            .with_line_number(true)
        )
        .init();
}
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, fmt};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::CONFIG;

pub fn init_log() {

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&CONFIG.log_level)))
        .with(ErrorLayer::default())
        .with(fmt::layer().with_writer(std::io::stdout))
        .init();
}

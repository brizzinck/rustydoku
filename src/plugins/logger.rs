use bevy::prelude::*;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

pub struct RustydokuLoggerPlugin;

impl Plugin for RustydokuLoggerPlugin {
    fn build(&self, _app: &mut App) {
        let filter =
            EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("error,rustydoku=debug"));

        let fmt_layer = fmt::layer().without_time();

        let subscriber = tracing_subscriber::registry().with(filter).with(fmt_layer);

        subscriber.init();
    }
}

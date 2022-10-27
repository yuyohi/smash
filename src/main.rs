use event::SmashState;
use tracing_subscriber::{self, fmt, prelude::*, EnvFilter};

mod event;

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    SmashState::new().run();
}

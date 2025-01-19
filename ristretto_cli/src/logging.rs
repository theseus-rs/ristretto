use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;

/// Initializes the logging system.
pub(crate) fn initialize() {
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_names(true)
        .with_timer(fmt::time::uptime())
        .compact();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("JAVA_LOG"))
        .fmt_fields(fmt::format::DefaultFields::new())
        .event_format(format)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        // This test just checks that the function doesn't panic.
        initialize();
    }
}

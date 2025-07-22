use ristretto_vm::Error::InternalError;
use ristretto_vm::Result;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;

/// Initializes the logging system.
pub(crate) fn initialize() -> Result<()> {
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_timer(fmt::time::uptime())
        .compact();

    let cranelift_directive = "cranelift=warn"
        .parse()
        .map_err(|error| InternalError(format!("{error}")))?;
    let filter = EnvFilter::from_env("JAVA_LOG").add_directive(cranelift_directive);
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .fmt_fields(fmt::format::DefaultFields::new())
        .event_format(format)
        .init();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() -> Result<()> {
        // This test just checks that the function doesn't panic.
        initialize()
    }
}

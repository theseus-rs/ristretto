use ristretto_vm::Error::InternalError;
use ristretto_vm::Result;
use std::io::IsTerminal;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;

const LOG_ENV_VAR: &str = "RISTRETTO_LOG";

/// Initializes the logging system.
pub(crate) fn initialize() -> Result<()> {
    if std::env::var_os(LOG_ENV_VAR).is_none() {
        return Ok(());
    }

    let enable_ansi = std::io::stdout().is_terminal();

    let format = tracing_subscriber::fmt::format()
        .with_ansi(enable_ansi)
        .with_level(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(true)
        .with_timer(fmt::time::uptime())
        .compact();

    let cranelift_directive = "cranelift=warn"
        .parse()
        .map_err(|error| InternalError(format!("{error}")))?;
    let filter = EnvFilter::from_env(LOG_ENV_VAR).add_directive(cranelift_directive);
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

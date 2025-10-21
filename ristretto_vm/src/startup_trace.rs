#[cfg(feature = "startup-trace")]
use std::sync::{LazyLock, Mutex, OnceLock};
#[cfg(feature = "startup-trace")]
use std::time::{Duration, Instant};

#[cfg(feature = "startup-trace")]
pub static ENABLED: LazyLock<bool> = LazyLock::new(|| {
    matches!(
        std::env::var("RISTRETTO_STARTUP_TRACE"),
        Ok(v) if matches!(&*v.to_ascii_lowercase(), "1" | "true" | "yes" | "on")
    )
});

// First-call timestamp (set exactly on the first trace call)
#[cfg(feature = "startup-trace")]
static START: OnceLock<Instant> = OnceLock::new();

// Previous-call timestamp
#[cfg(feature = "startup-trace")]
static LAST: LazyLock<Mutex<Option<Instant>>> = LazyLock::new(|| Mutex::new(None));

/// Internal logging shim used by the `startup_trace!` macro.
#[doc(hidden)]
#[cfg(feature = "startup-trace")]
pub fn startup_trace_log(message: &str) {
    if !*ENABLED {
        return;
    }

    let now = Instant::now();
    let mut last_guard = LAST.lock().expect("failed to lock LAST");
    let start = START.get_or_init(|| {
        *last_guard = Some(now);
        now
    });

    if let Some(last) = *last_guard {
        let delta_elapsed = now.checked_duration_since(last).unwrap_or(Duration::ZERO);
        let start_elapsed = now.checked_duration_since(*start).unwrap_or(Duration::ZERO);
        *last_guard = Some(now);
        println!("[startup]{message}: +{delta_elapsed:.3?} (Σ {start_elapsed:.3?})");
    } else {
        *last_guard = Some(now);
        println!(
            "[startup]{message}: +{:.3?} (Σ {:.3?})",
            Duration::ZERO,
            Duration::ZERO
        );
    }
}

/// Log a startup phase message and the time elapsed since the last `startup_trace!()` call.
#[macro_export]
macro_rules! startup_trace {
    ($msg:expr) => {{
        #[cfg(feature = "startup-trace")]
        $crate::startup_trace::startup_trace_log($msg);
    }};
}

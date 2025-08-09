#[cfg(feature = "startup-trace")]
use std::time::{Duration, Instant};

#[cfg(feature = "startup-trace")]
use std::sync::{LazyLock, Mutex};

#[cfg(feature = "startup-trace")]
pub static ENABLED: LazyLock<bool> = LazyLock::new(|| {
    matches!(
        std::env::var("RISTRETTO_STARTUP_TRACE"),
        Ok(v) if matches!(&*v.to_ascii_lowercase(), "1" | "true" | "yes" | "on")
    )
});

#[cfg(feature = "startup-trace")]
static START: LazyLock<Instant> = LazyLock::new(Instant::now);
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
    let mut last = LAST.lock().expect("failed to lock");

    let (delta_elapsed, start_elapsed) = match *last {
        Some(prev) => (now.duration_since(prev), now.duration_since(*START)),
        None => (Duration::ZERO, Duration::ZERO),
    };

    *last = Some(now);
    println!("[startup]{message}: +{delta_elapsed:.3?} (Σ {start_elapsed:.3?})");
}

/// No-op version when the feature is disabled; zero code generation.
#[doc(hidden)]
#[cfg(not(feature = "startup-trace"))]
#[inline(always)]
pub fn startup_trace_log(_message: &str) {}

/// Log a startup phase message and the time elapsed since the last `startup_trace!()` call.
#[macro_export]
macro_rules! startup_trace {
    ($msg:expr) => {{
        $crate::startup_trace::startup_trace_log($msg);
    }};
}

//! Monotonic elapsed-time source shared by Java `nanoTime` and runtime telemetry.
//! Linux uses `CLOCK_MONOTONIC` directly; other clocked hosts use a process-local
//! `Instant` origin. Bare wasm has no host clock and returns `None` without panicking.

/// Nanoseconds from an arbitrary monotonic epoch. Values are unrelated to UTC.
/// Returns `None` when the host has no clock or the clock query fails/overflows.
#[must_use]
pub fn monotonic_time_ns() -> Option<u64> {
    #[cfg(target_os = "linux")]
    {
        linux_clock(false)
    }
    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    {
        None
    }
    #[cfg(not(any(
        target_os = "linux",
        all(target_family = "wasm", target_os = "unknown")
    )))]
    {
        static ORIGIN: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
        let origin = ORIGIN.get_or_init(std::time::Instant::now);
        u64::try_from(origin.elapsed().as_nanos()).ok()
    }
}

/// Clock identity; consumers must not combine timestamps from different epochs.
#[must_use]
pub const fn monotonic_clock_name() -> &'static str {
    if cfg!(target_os = "linux") {
        "clock_monotonic_ns"
    } else if cfg!(all(target_family = "wasm", target_os = "unknown")) {
        "unavailable"
    } else {
        "process_instant_ns"
    }
}

/// Declared host resolution, when available. Unknown resolution is not zero.
/// Miri does not implement `clock_getres`; its `clock_gettime` remains usable.
#[must_use]
pub fn monotonic_resolution_ns() -> Option<u64> {
    #[cfg(all(target_os = "linux", not(miri)))]
    {
        linux_clock(true)
    }
    #[cfg(any(not(target_os = "linux"), miri))]
    {
        None
    }
}

#[cfg(target_os = "linux")]
fn linux_clock(resolution: bool) -> Option<u64> {
    let mut value = std::mem::MaybeUninit::<libc::timespec>::uninit();
    // SAFETY: both functions initialize the timespec on success and retain no pointer.
    let result = unsafe {
        if resolution {
            libc::clock_getres(libc::CLOCK_MONOTONIC, value.as_mut_ptr())
        } else {
            libc::clock_gettime(libc::CLOCK_MONOTONIC, value.as_mut_ptr())
        }
    };
    if result != 0 {
        return None;
    }
    // SAFETY: a successful clock_gettime/clock_getres initialized both fields.
    let value = unsafe { value.assume_init() };
    u64::try_from(value.tv_sec)
        .ok()
        .and_then(|seconds| seconds.checked_mul(1_000_000_000))
        .and_then(|seconds| {
            u64::try_from(value.tv_nsec)
                .ok()
                .and_then(|ns| seconds.checked_add(ns))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(
        not(all(target_family = "wasm", target_os = "unknown")),
        expect(clippy::panic_in_result_fn)
    )]
    fn monotonic_clock_does_not_use_the_unix_epoch() -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
        {
            let before = monotonic_time_ns();
            let after = monotonic_time_ns();
            assert!(before.is_some());
            assert!(after >= before);
            // Regression for nanoTime's previous direct SystemTime/UNIX_EPOCH
            // implementation. No supported host has run since the Unix epoch.
            let wall = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos();
            assert!(u128::from(after.ok_or("monotonic clock unavailable")?) < wall / 2);
        }
        #[cfg(all(target_family = "wasm", target_os = "unknown"))]
        assert_eq!(monotonic_time_ns(), None);
        Ok(())
    }

    #[test]
    fn unknown_clock_resolution_is_explicit() {
        #[cfg(all(target_os = "linux", not(miri)))]
        assert!(monotonic_resolution_ns().is_some_and(|ns| ns > 0));
        #[cfg(any(not(target_os = "linux"), miri))]
        assert_eq!(monotonic_resolution_ns(), None);
    }
}

//! Restricted Java execution for canonical catalog research.
#![cfg_attr(
    all(test, feature = "experimental-canonical-runtime"),
    expect(
        clippy::panic_in_result_fn,
        reason = "session tests propagate setup failures and assert Java execution results"
    )
)]

#[cfg(feature = "experimental-canonical-runtime")]
pub mod canonical;
#[cfg(all(feature = "experimental-canonical-telemetry", target_has_atomic = "64"))]
pub mod canonical_telemetry;

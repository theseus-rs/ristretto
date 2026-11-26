//! Restricted Java execution for canonical catalog research.
#[cfg(feature = "experimental-canonical-runtime")]
pub mod canonical;
#[cfg(all(feature = "experimental-canonical-telemetry", target_has_atomic = "64"))]
pub mod canonical_telemetry;

//! Copied object identity; the Q=0 preview has no conditional-reference protocol.
/// Exact identity within one catalog. The eventual operation also checks catalog
/// ownership. An equal slot number alone cannot authorize a replacement target.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObjectKey {
    pub identity: u64,
    pub slot: usize,
}

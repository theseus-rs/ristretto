//! Private, bundled catalog for the optional canonical collector.
//! Numeric trace states alone grant no ownership or payload access.
mod trace;
pub use trace::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Domain {
    pub mode: DomainMode,
    pub epoch: u64,
    pub floor: u8,
    pub pending: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase {
    /// Per-domain root scan cursor; only a canonical scan operation advances it.
    pub root_cursors: [usize; 2],
    /// Per-domain cursor for the current conditional-reference stage.
    pub reference_cursors: [usize; 2],
    pub revision: u64,
    pub domains: [Domain; 2],
}

#[cfg(test)]
mod trace_tests;

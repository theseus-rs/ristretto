//! Nested attachment with temporary roots, shared by timing and count fixtures.
use ristretto_gc::{GarbageCollector, Gc, Trace, TraceOwner};

pub struct ConstructionBatch {
    depth: usize,
    value: u64,
}

impl ConstructionBatch {
    pub fn new() -> Self {
        Self {
            depth: 3,
            value: 42,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

// SAFETY: the payload stores only integers. Temporary allocations remain rooted
// by their guards and the construction scopes that receive those registrations.
#[expect(unsafe_code, reason = "explicit Trace attachment fixture")]
unsafe impl Trace for ConstructionBatch {
    fn trace(&self, _: &GarbageCollector) {}

    #[expect(
        clippy::expect_used,
        reason = "fixture heap has bounded live construction"
    )]
    unsafe fn attach(&mut self, owner: TraceOwner<'_>) {
        let gc = owner.collector().upgrade().expect("fixture collector");
        if self.depth == 0 {
            for _ in 0..8 {
                drop(Gc::try_new(&gc, std::hint::black_box(42_u64)).expect("temporary root"));
            }
        } else {
            drop(
                Gc::try_new(
                    &gc,
                    Self {
                        depth: self.depth - 1,
                        value: self.value,
                    },
                )
                .expect("nested constructor"),
            );
        }
    }
}

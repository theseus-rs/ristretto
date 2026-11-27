//! Identity lookup without an index reader/writer exclusion interval.
use arc_swap::{ArcSwapAny, ArcSwapOption, RefCnt};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[path = "identity_index_state.rs"]
mod state;
pub(crate) use state::{Cleanup, Index, Removal};
use state::{Edge, Node, Phase};

const LIVE_OFFSET: usize = 2;
const DELETED_OFFSET: usize = 4;
const OFFSET_MASK: usize = 7;

// Unlike integer pointer tags, these are actual immutable bytes with stable,
// distinct addresses. Empty deleted/detached states own no reference count.
#[repr(C, align(8))]
struct EmptyMarkers {
    deleted: [u8; 8],
    detached: [u8; 8],
}
static EMPTY: EmptyMarkers = EmptyMarkers {
    deleted: [1; 8],
    detached: [2; 8],
};

fn empty_pointer(phase: Phase) -> *mut u8 {
    match phase {
        Phase::Live => std::ptr::null_mut(),
        Phase::Deleted => std::ptr::addr_of!(EMPTY.deleted).cast::<u8>().cast_mut(),
        Phase::Detached => std::ptr::addr_of!(EMPTY.detached).cast::<u8>().cast_mut(),
    }
}
fn offset(phase: Phase) -> usize {
    if phase == Phase::Live {
        LIVE_OFFSET
    } else {
        DELETED_OFFSET
    }
}

// SAFETY: Node is repr(C), aligned to eight, and starts with eight immutable
// bytes. Nonempty pointers refer to bytes 2/4 of that SAME Arc allocation. Pointer
// recovery preserves provenance and restores the exact Arc::into_raw address.
// All equal nonempty pointers share the same strong count. TAG stays zero, as
// for Arc. Our offsets are even, so they cannot alias ArcSwap's Weak debt tag (1)
// or its empty-debt sentinel (3). Empty marker addresses are static storage and
// cannot coincide with a live Arc allocation. NULL owns no reference count.
// Cloning preserves phase and increments exactly the underlying Arc count;
// from_ptr reverses into_ptr without incrementing it. None of these conversions
// allocates, panics or dereferences an encoded pointer. Node/Edge remain private
// so no independently counted Arc can be constructed from an interior marker.
unsafe impl RefCnt for Edge {
    type Base = u8;

    fn into_ptr(me: Self) -> *mut u8 {
        match me.next {
            Some(node) => Arc::into_raw(node)
                .cast::<u8>()
                .cast_mut()
                .wrapping_add(offset(me.phase)),
            None => empty_pointer(me.phase),
        }
    }
    fn as_ptr(me: &Self) -> *mut u8 {
        match &me.next {
            Some(node) => <Arc<Node> as RefCnt>::as_ptr(node)
                .cast::<u8>()
                .wrapping_add(offset(me.phase)),
            None => empty_pointer(me.phase),
        }
    }
    unsafe fn from_ptr(pointer: *const u8) -> Self {
        if pointer.is_null() {
            return Self::default();
        }
        if pointer == empty_pointer(Phase::Deleted) {
            return Self {
                next: None,
                phase: Phase::Deleted,
            };
        }
        if pointer == empty_pointer(Phase::Detached) {
            return Self {
                next: None,
                phase: Phase::Detached,
            };
        }
        let offset = pointer.addr() & OFFSET_MASK;
        // SAFETY: ArcSwap supplies only pointers produced above (or borrowed
        // copies while their strong count is protected). Restore that Arc data
        // pointer, without changing its provenance or consuming another owner.
        #[expect(
            clippy::cast_ptr_alignment,
            reason = "subtracting the marker offset restores Node's statically checked alignment"
        )]
        let node = unsafe { Arc::from_raw(pointer.wrapping_sub(offset).cast::<Node>()) };
        Self {
            next: Some(node),
            phase: if offset == LIVE_OFFSET {
                Phase::Live
            } else {
                Phase::Deleted
            },
        }
    }
}

struct Link(ArcSwapAny<Edge>);
impl Link {
    fn new(value: Edge) -> Self {
        Self(ArcSwapAny::from(value))
    }
    fn load(&self) -> Edge {
        self.0.load_full()
    }
    fn swap(&self, value: Edge) -> Edge {
        self.0.swap(value)
    }
    fn compare(&self, expected: &Edge, value: Edge) -> bool {
        let previous = self.0.compare_and_swap(Edge::as_ptr(expected), value);
        previous.same(expected)
    }
}

struct OwnedLink<T>(ArcSwapOption<T>);
impl<T> OwnedLink<T> {
    fn new(value: Option<Arc<T>>) -> Self {
        Self(ArcSwapOption::from(value))
    }
    fn load(&self) -> Option<Arc<T>> {
        self.0.load_full()
    }
    fn swap(&self, value: Option<Arc<T>>) -> Option<Arc<T>> {
        self.0.swap(value)
    }
    fn compare(&self, expected: &Arc<T>, value: Arc<T>) -> bool {
        self.0
            .compare_and_swap(&Some(expected.clone()), Some(value))
            .as_ref()
            .is_some_and(|previous| Arc::ptr_eq(previous, expected))
    }
}
fn take_owned<T>(value: Arc<T>) -> Option<T> {
    Arc::into_inner(value)
}

#[cfg(test)]
#[path = "identity_index_tests.rs"]
mod tests;

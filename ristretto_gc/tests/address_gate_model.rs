#![cfg(not(target_family = "wasm"))]
#![allow(unsafe_code)] // Loom checks conflicting accesses through these pointers.
use loom::sync::Arc;
use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[path = "../src/address_gate.rs"]
mod address_gate;
use address_gate::{Gate, ReadMode, WriteMode};

#[test]
#[expect(
    clippy::unreachable,
    reason = "the first write cannot already have published shared mode"
)]
fn changing_a_slot_excludes_readers_or_hands_cleanup_to_one_owner() {
    loom::model(|| {
        let gate = Arc::new(Gate::default());
        let value = Arc::new(loom::cell::UnsafeCell::new(1));
        let replacement = Arc::new(AtomicBool::new(false));
        let reader = {
            let (gate, value, replacement) = (gate.clone(), value.clone(), replacement.clone());
            loom::thread::spawn(move || {
                match gate.read() {
                    ReadMode::Primary => value.with(|p| {
                        // SAFETY: read admission protects the primary in this scope.
                        let old = unsafe { *p };
                        loom::thread::yield_now();
                        assert!(old == 1 || old == 2);
                        assert_eq!(unsafe { *p }, old);
                    }),
                    ReadMode::Shared => assert!(replacement.load(Ordering::Relaxed)),
                    ReadMode::Vacant => {}
                }
                if gate.release() {
                    value.with_mut(|p| unsafe { *p = 0 });
                }
            })
        };
        match gate.write() {
            WriteMode::Exclusive => {
                value.with_mut(|p| unsafe { *p = 2 });
                gate.finish_exclusive();
            }
            WriteMode::Promote => {
                replacement.store(true, Ordering::Relaxed);
                if gate.publish_shared() {
                    value.with_mut(|p| unsafe { *p = 0 });
                }
            }
            WriteMode::Shared => unreachable!(),
        }
        reader.join().unwrap();
        if replacement.load(Ordering::Relaxed) {
            assert!(gate.cleaned());
            value.with(|p| assert_eq!(unsafe { *p }, 0));
            assert_eq!(gate.write(), WriteMode::Shared);
        } else {
            value.with(|p| assert_eq!(unsafe { *p }, 2));
        }
    });
}

#[test]
fn a_sole_reader_can_take_the_slot_while_a_concurrent_reader_remains_safe() {
    loom::model(|| {
        let gate = Arc::new(Gate::default());
        let cell = Arc::new(loom::cell::UnsafeCell::new(19));
        assert_eq!(gate.read(), ReadMode::Primary);
        let reader = {
            let (gate, cell) = (gate.clone(), cell.clone());
            loom::thread::spawn(move || {
                if gate.read() == ReadMode::Primary {
                    cell.with(|p| {
                        let before = unsafe { *p };
                        loom::thread::yield_now();
                        assert_eq!(before, unsafe { *p });
                    });
                }
                if gate.release() {
                    cell.with_mut(|p| unsafe { *p = 0 });
                }
            })
        };
        if gate.take_reader() {
            cell.with_mut(|p| unsafe { *p = 0 });
            gate.finish_exclusive();
        } else {
            assert!(!gate.release());
            match gate.write() {
                WriteMode::Exclusive => {
                    cell.with_mut(|p| unsafe { *p = 0 });
                    gate.finish_exclusive();
                }
                WriteMode::Promote => {
                    if gate.publish_shared() {
                        cell.with_mut(|p| unsafe { *p = 0 });
                    }
                }
                WriteMode::Shared => panic!("one writer"),
            }
        }
        reader.join().unwrap();
        cell.with(|p| assert_eq!(unsafe { *p }, 0));
    });
}

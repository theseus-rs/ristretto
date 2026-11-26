//! Own explicit monitor entries across suspension, OS-thread migration and cancellation.
//! Java bytecodes retain the existing thread-based (non-structured) exit policy.
//! Only cancelled host execution scopes release outstanding entries automatically;
//! normal Java completion still relies on its generated monitor-exit handlers.
use super::{InternalError, Result, Thread};
use portable_atomic::{AtomicU64, Ordering};
use ristretto_classloader::{Reference, Value};
use ristretto_gc::Gc;
use ristretto_gc::sync::{Mutex, RwLock};
use ristretto_intrinsics::get_monitor_id;
use ristretto_intrinsics::java::lang::thread::{ThreadState, set_thread_status};
use ristretto_types::monitor::Monitor;
use std::sync::Arc;

#[derive(Debug)]
struct HeldMonitor {
    serial: u64,
    monitor: Option<Arc<Monitor>>,
    thread_id: u64,
    // Locks keep their objects alive even if the last Java local is dead.
    _object: Gc<RwLock<Reference>>,
}

impl Drop for HeldMonitor {
    fn drop(&mut self) {
        if let Some(monitor) = self.monitor.take() {
            let _ = monitor.release(self.thread_id);
        }
    }
}

#[derive(Debug, Default)]
pub(super) struct ExplicitMonitors {
    next_serial: AtomicU64,
    held: Mutex<Vec<HeldMonitor>>,
}

impl ExplicitMonitors {
    pub(super) fn checkpoint(&self) -> u64 {
        self.next_serial.load(Ordering::Acquire)
    }

    pub(super) fn cancel_since(&self, checkpoint: u64) {
        if self.checkpoint() == checkpoint {
            return;
        }
        loop {
            let entry = {
                let mut held = self.held.lock();
                if held.last().is_some_and(|entry| entry.serial >= checkpoint) {
                    held.pop()
                } else {
                    None
                }
            };
            let Some(entry) = entry else { break };
            // Waking contenders and releasing GC owners cannot run under the ledger lock.
            drop(entry);
        }
    }
}

struct BlockedStatus(Value);

impl BlockedStatus {
    fn new(object: Value) -> Self {
        let _ = set_thread_status(&object, ThreadState::BLOCKED);
        Self(object)
    }
}

impl Drop for BlockedStatus {
    fn drop(&mut self) {
        let _ = set_thread_status(&self.0, ThreadState::RUNNABLE);
    }
}

impl Thread {
    pub(crate) async fn enter_monitor(&self, reference: &Gc<RwLock<Reference>>) -> Result<()> {
        let vm = self.vm()?;
        let id = get_monitor_id(reference)
            .ok_or_else(|| InternalError("object has no monitor identity".to_owned()))?;
        let monitor = vm.monitor_registry().monitor(id);
        let object = reference.rooted_clone();
        // Reserve a non-reused cancellation identity before acquiring ownership.
        let serial = self
            .explicit_monitors
            .next_serial
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |value| {
                value.checked_add(1)
            })
            .map_err(|_| InternalError("monitor acquisition serial exhausted".to_owned()))?;
        let _status = BlockedStatus::new(self.java_object().await);
        monitor.acquire(self.id()).await?;
        // No await separates successful acquisition from the owning record.
        self.explicit_monitors.held.lock().push(HeldMonitor {
            serial,
            monitor: Some(monitor),
            thread_id: self.id(),
            _object: object,
        });
        Ok(())
    }

    pub(crate) fn exit_monitor(&self, reference: &Gc<RwLock<Reference>>) -> Result<()> {
        let vm = self.vm()?;
        let id = get_monitor_id(reference)
            .ok_or_else(|| InternalError("object has no monitor identity".to_owned()))?;
        let monitor = vm.monitor_registry().monitor(id);
        monitor.release(self.id())?;
        let entry = {
            let mut held = self.explicit_monitors.held.lock();
            held.iter()
                .rposition(|entry| {
                    entry
                        .monitor
                        .as_ref()
                        .is_some_and(|owned| Arc::ptr_eq(owned, &monitor))
                })
                .map(|index| {
                    let mut entry = held.remove(index);
                    entry.monitor = None;
                    entry
                })
        };
        drop(entry);
        Ok(())
    }
}

//! A terminal allocation may wait for one native dispatch opportunity. A running
//! callback remains charged and its return is never required to finish the turn.
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[derive(Debug, Default)]
pub(super) struct Service {
    requested: AtomicUsize,
    examined: AtomicUsize,
    running: AtomicBool,
    stopped: AtomicBool,
}

#[derive(Clone, Debug)]
pub(in crate::collector) struct Turn {
    service: Arc<Service>,
    ticket: usize,
}

impl Turn {
    pub(in crate::collector) fn pending(&self) -> bool {
        !self.service.unavailable() && self.service.examined.load(Ordering::Acquire) < self.ticket
    }
}

impl Service {
    fn unavailable(&self) -> bool {
        self.running.load(Ordering::Acquire) || self.stopped.load(Ordering::Acquire)
    }

    pub(super) fn request(self: &Arc<Self>) -> Option<Turn> {
        if self.unavailable() {
            return None;
        }
        let previous = self.requested.load(Ordering::Acquire);
        let next = previous.checked_add(1)?;
        // A request enrolled since our load covers the same service turn.
        let ticket = match self.requested.compare_exchange(
            previous,
            next,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => next,
            Err(concurrent) => concurrent,
        };
        Some(Turn {
            service: self.clone(),
            ticket,
        })
    }

    pub(super) fn snapshot(&self) -> usize {
        self.requested.load(Ordering::Acquire)
    }

    // Only the assistant writes examined. Sampling before examining demand keeps
    // an older empty probe from acknowledging a subsequently registered turn.
    pub(super) fn examined(&self, ticket: usize) {
        self.examined.store(ticket, Ordering::Release);
    }

    pub(super) fn running(&self, active: bool) {
        self.running.store(active, Ordering::Release);
    }

    pub(super) fn stop(&self) {
        self.stopped.store(true, Ordering::Release);
    }
}

pub(super) struct Assistant(pub(super) Arc<Service>);
impl Drop for Assistant {
    fn drop(&mut self) {
        self.0.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_older_empty_probe_cannot_discharge_later_requests() {
        let service = Arc::new(Service::default());
        let earlier = service.snapshot();
        let first = service.request().expect("live service");
        let second = service.request().expect("live service");
        service.examined(earlier);
        assert!(first.pending() && second.pending());
        service.examined(first.ticket);
        assert!(!first.pending() && second.pending());
        service.running(true);
        assert!(!second.pending() && service.request().is_none());
        service.examined(second.ticket);
        service.running(false);
        assert!(!second.pending());
    }

    #[test]
    fn dropping_the_assistant_releases_outstanding_observers() {
        let service = Arc::new(Service::default());
        let assistant = Assistant(service.clone());
        let turn = service.request().expect("live service");
        drop(assistant);
        assert!(!turn.pending() && service.request().is_none());
    }
}

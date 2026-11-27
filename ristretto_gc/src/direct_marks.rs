//! Synchronous visibility of a committed direct trace while its queued traversal
//! awaits materialization. Each traced descriptor has at most one record per
//! domain. Weak ownership distinguishes epochs even across numeric rollover.
use super::Retention;
use crate::work::{BirthEpoch, MarkQueues};
use arc_swap::ArcSwapOption;
use once_cell::race::OnceBox;
use std::sync::{Arc, Weak};

struct Entry {
    epoch: Weak<()>,
    strength: Retention,
}

#[derive(Default)]
pub(crate) struct DirectMarks(OnceBox<[ArcSwapOption<Entry>; 2]>);

impl DirectMarks {
    pub(crate) fn get(&self, index: usize, number: u64, queues: &MarkQueues) -> Option<Retention> {
        let entry = self.0.get()?.get(index)?.load();
        let entry = entry.as_ref()?;
        let epoch = queues.birth_epoch()?;
        (epoch.number == number && epoch.matches_witness(&entry.epoch)).then_some(entry.strength)
    }

    pub(crate) fn publish(
        &self,
        index: usize,
        epoch: &Arc<BirthEpoch>,
        strength: Retention,
        queues: &MarkQueues,
    ) {
        let slots = self.0.get_or_init(Box::default);
        let Some(slot) = slots.get(index) else { return };
        let replacement = Arc::new(Entry {
            epoch: epoch.direct_witness(),
            strength,
        });
        // Validate the epoch after loading the prior slot on every CAS retry.
        // A resumed publisher cannot replace a mark installed for a later epoch.
        slot.rcu(|prior| {
            if prior.as_ref().is_some_and(|p| {
                Weak::ptr_eq(&p.epoch, &replacement.epoch) && p.strength <= strength
            }) || !queues.current_birth_epoch(epoch)
            {
                prior.clone()
            } else {
                #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
                concurrent_test::after_epoch_check();
                Some(replacement.clone())
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_epoch_cannot_replace_a_new_mark_with_the_same_epoch_number() {
        let queues = MarkQueues::new(0, 1);
        let marks = DirectMarks::default();
        queues.open_graph_inbox(4, false, Weak::new());
        let first = queues.birth_epoch().unwrap();
        marks.publish(1, &first, Retention::Final, &queues);
        queues.close_graph_inbox();
        queues.open_graph_inbox(4, false, Weak::new());
        let second = queues.birth_epoch().unwrap();
        assert_eq!(marks.get(1, 4, &queues), None);
        marks.publish(1, &second, Retention::Phantom, &queues);
        marks.publish(1, &first, Retention::Strong, &queues);
        assert_eq!(marks.get(1, 4, &queues), Some(Retention::Phantom));
        marks.publish(1, &second, Retention::Final, &queues);
        marks.publish(1, &second, Retention::Phantom, &queues);
        assert_eq!(marks.get(1, 4, &queues), Some(Retention::Final));
    }
}

#[cfg(all(test, not(target_family = "wasm"), not(miri)))]
mod concurrent_test {
    use super::*;
    use std::cell::RefCell;
    use std::sync::mpsc::{self, Receiver, Sender};
    use std::time::Duration;

    thread_local! {
        static PAUSE: RefCell<Option<(Sender<()>, Receiver<()>)>> = const { RefCell::new(None) };
    }

    pub(super) fn after_epoch_check() {
        if let Some((entered, resume)) = PAUSE.take() {
            assert!(entered.send(()).is_ok(), "notify paused publisher");
            assert!(
                resume.recv_timeout(Duration::from_secs(5)).is_ok(),
                "resume publisher"
            );
        }
    }

    #[test]
    fn parked_slot_writer_revalidates_after_a_same_number_replacement() {
        for publish_new_first in [false, true] {
            let queues = MarkQueues::new(0, 1);
            let marks = DirectMarks::default();
            queues.open_graph_inbox(4, false, Weak::new());
            let first = queues.birth_epoch().unwrap();
            let (entered, parked) = mpsc::channel();
            let (release, resume) = mpsc::channel();
            std::thread::scope(|scope| {
                let (marks, queues) = (&marks, &queues);
                let worker = scope.spawn(move || {
                    PAUSE.set(Some((entered, resume)));
                    marks.publish(1, &first, Retention::Strong, queues);
                });
                parked.recv_timeout(Duration::from_secs(5)).unwrap();
                queues.close_graph_inbox();
                queues.open_graph_inbox(4, false, Weak::new());
                let second = queues.birth_epoch().unwrap();
                if publish_new_first {
                    marks.publish(1, &second, Retention::Final, queues);
                }
                release.send(()).unwrap();
                worker.join().unwrap();
                assert_eq!(
                    marks.get(1, 4, queues),
                    publish_new_first.then_some(Retention::Final)
                );
                if !publish_new_first {
                    marks.publish(1, &second, Retention::Final, queues);
                    assert_eq!(marks.get(1, 4, queues), Some(Retention::Final));
                }
            });
        }
    }
}

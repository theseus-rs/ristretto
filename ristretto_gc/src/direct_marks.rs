//! Synchronous visibility of a committed direct trace while its queued traversal
//! awaits materialization. Each traced descriptor has at most one record per
//! domain. Weak ownership distinguishes epochs even across numeric rollover.
use super::Retention;
use crate::work::{BirthEpoch, MarkQueues};
use arc_swap::ArcSwapOption;
use once_cell::race::OnceBox;
use std::sync::{Arc, Weak};

struct Entry {
    epoch: Weak<BirthEpoch>,
    strength: Retention,
}

#[derive(Default)]
pub(crate) struct DirectMarks(OnceBox<[ArcSwapOption<Entry>; 2]>);

impl DirectMarks {
    pub(crate) fn get(&self, index: usize, number: u64, queues: &MarkQueues) -> Option<Retention> {
        let entry = self.0.get()?.get(index)?.load();
        let entry = entry.as_ref()?;
        let epoch = queues.birth_epoch()?;
        (epoch.number == number && std::ptr::eq(entry.epoch.as_ptr(), Arc::as_ptr(&epoch)))
            .then_some(entry.strength)
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
            epoch: Arc::downgrade(epoch),
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
        queues.open_graph_inbox(4, false);
        let first = queues.birth_epoch().unwrap();
        marks.publish(1, &first, Retention::Final, &queues);
        queues.close_graph_inbox();
        queues.open_graph_inbox(4, false);
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

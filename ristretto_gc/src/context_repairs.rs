//! Schedule page repair independently of context root scans and barrier closure.
use super::State;
use crate::publication_directory::Directory;
use crate::roots::RootSlots;
use arc_swap::{ArcSwap, ArcSwapOption};
use once_cell::race::OnceBox;
use std::sync::{Arc, Weak};

#[derive(Debug, Default)]
pub(crate) struct Repairs(OnceBox<Table>);

#[derive(Debug, Default)]
struct Table {
    entries: ArcSwap<Directory<Weak<RootSlots>>>,
    pass: ArcSwapOption<Pass>,
}

#[derive(Debug)]
struct Pass {
    after: u64,
    limit: u64,
}

impl Repairs {
    #[expect(
        clippy::expect_used,
        reason = "a live context has a unique pointer-width ID"
    )]
    pub(crate) fn publish(&self, state: &State) -> bool {
        if !state.roots.repair_pending() {
            return false;
        }
        let table = self.0.get_or_init(Box::default);
        let id = state.id as u64;
        table.entries.rcu(|current| {
            // Changing the view identity even for an existing key invalidates a
            // remover that sampled the mask before this new debt was published.
            if current.get(id).is_some() {
                Arc::new((**current).clone())
            } else {
                Arc::new(
                    current
                        .insert(id, Arc::downgrade(&state.roots))
                        .expect("live context fits")
                        .0,
                )
            }
        });
        true
    }

    pub(crate) fn pending(&self) -> bool {
        self.0
            .get()
            .is_some_and(|table| table.entries.load().len() != 0)
    }

    pub(crate) fn step(&self) -> bool {
        let Some(table) = self.0.get() else {
            return false;
        };
        let current = table.entries.load_full();
        let Some((id, weak)) = table.select(&current) else {
            return false;
        };
        let roots = weak.upgrade();
        let repaired = roots.as_ref().is_some_and(|slots| slots.repair_step());
        if roots.as_ref().is_none_or(|slots| !slots.repair_pending()) {
            return table.remove_if_current(&current, id) || repaired;
        }
        repaired
    }
}

impl Table {
    fn select<'a>(
        &self,
        current: &'a Directory<Weak<RootSlots>>,
    ) -> Option<(u64, &'a Weak<RootSlots>)> {
        let first = current.next_after(None)?;
        if current.len() == 1 {
            return Some(first);
        }
        let pass = self.pass.load_full();
        let ((id, weak), limit) = pass
            .as_ref()
            .and_then(|p| {
                current
                    .next_after(Some(p.after))
                    .filter(|(key, _)| *key <= p.limit)
                    .map(|entry| (entry, p.limit))
            })
            .unwrap_or((first, current.last_key()?));
        // Reserve the turn before touching a shard. A suspended helper owns no
        // exclusive cursor, and fresh context IDs cannot extend this pass.
        let previous = self
            .pass
            .compare_and_swap(&pass, Some(Arc::new(Pass { after: id, limit })));
        (previous.as_ref().map(Arc::as_ptr) == pass.as_ref().map(Arc::as_ptr)).then_some((id, weak))
    }

    fn remove_if_current(&self, view: &Arc<Directory<Weak<RootSlots>>>, id: u64) -> bool {
        let seen = self
            .entries
            .compare_and_swap(view, Arc::new(view.remove(id).0));
        Arc::ptr_eq(&seen, view)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Configuration, GarbageCollector, RootStorage};

    #[test]
    fn stale_cleanup_cannot_erase_new_debt_for_the_same_context() {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .root_storage(RootStorage::Contexts)
                .build(),
        );
        let context = gc.register_context();
        let reserve = || context.with(|| gc.reserve_owned_root(0x1000));
        let keep: Vec<_> = (0..crate::roots::SLOTS).map(|_| reserve()).collect();
        let cancel = reserve();
        context.0.roots.with_all_writers_held(|| drop(cancel));
        let table = gc.contexts.repairs.0.get().unwrap();
        // A helper captures the view and observes the final clean shard.
        let stale = table.entries.load_full();
        while context.0.roots.repair_step() {}
        assert!(!context.0.roots.repair_pending());
        let cancel = reserve();
        context.0.roots.with_all_writers_held(|| drop(cancel));
        assert!(!table.remove_if_current(&stale, context.0.id as u64));
        assert!(gc.contexts.repairs.pending());
        let weak_context = Arc::downgrade(&context.0);
        let registry = Arc::downgrade(&context.0.roots);
        drop((keep, context));
        // Outstanding directory views can own neither a context nor its pages.
        assert!(weak_context.upgrade().is_none() && registry.upgrade().is_none());
        assert!((0..256).any(|_| gc.step()));
        assert!(!gc.contexts.repairs.pending());
    }
}

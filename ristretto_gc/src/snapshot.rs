//! Reuse a displaced immutable version only after all of its readers release it.
use arc_swap::ArcSwap;
use std::cell::Cell;
use std::sync::Arc;
use std::thread::LocalKey;

/// One retired allocation per updating thread. Use for value-only observations
/// so an idle thread cannot retain a collector or managed object.
pub(crate) type Spare<T> = Cell<Option<Arc<T>>>;

/// Apply a retry-safe edit to the current version, returning information from
/// the version displaced by the winning commit.
pub(crate) fn update<T: Clone + 'static, R>(
    published: &ArcSwap<T>,
    spare: &'static LocalKey<Spare<T>>,
    change: impl Fn(&mut T),
    displaced: impl FnOnce(&T) -> R,
) -> R {
    let prior = published.rcu(|previous| {
        if let Some(mut retired) = spare.try_with(Cell::take).ok().flatten()
            && let Some(next) = Arc::get_mut(&mut retired)
        {
            // This allocation may have belonged to a different collector.
            next.clone_from(previous.as_ref());
            change(next);
            return retired;
        }
        let mut next = previous.as_ref().clone();
        change(&mut next);
        Arc::new(next)
    });
    let result = displaced(&prior);
    // TLS can be unavailable during thread teardown.
    let _ = spare.try_with(|slot| slot.set(Some(prior)));
    result
}

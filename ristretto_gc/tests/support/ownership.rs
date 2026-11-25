//! Configurations used by the owner-binding regressions.
use ristretto_gc::{Configuration, GarbageCollector, RootStorage};
use std::sync::Arc;

pub fn collectors() -> impl Iterator<Item = Arc<GarbageCollector>> {
    [false, true].into_iter().flat_map(|concurrent| {
        [false, true].into_iter().flat_map(move |independent| {
            [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts]
                .into_iter()
                .map(move |storage| {
                    GarbageCollector::with_config(
                        Configuration::builder()
                            .concurrent_publication(concurrent)
                            .independent_phases(independent)
                            .independent_marking(independent)
                            .independent_relocation(independent)
                            .overlapping_generations(independent)
                            .root_storage(storage)
                            .diagnostics(true)
                            .allocation_threshold(usize::MAX)
                            .max_pause_time_us(0)
                            .build(),
                    )
                })
        })
    })
}

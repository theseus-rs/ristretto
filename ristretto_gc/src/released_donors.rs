//! Keep returned, directly transferable storage visible until a helper takes
//! the exclusive physical destruction claim. Scanning never takes that claim.
use super::{Arc, ArcSwap, ArcSwapOption, Attempt, Directory, Donation, Registry, Selection};
use crate::metadata::MetadataRef;
use portable_atomic::AtomicU64;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
enum Cursor {
    Scan {
        next: u64,
        remaining: usize,
    },
    Bind {
        next: u64,
        remaining: usize,
        attempt: Attempt,
    },
    Done {
        remaining: usize,
    },
}

#[derive(Debug)]
struct SharedSelection {
    cohort: Selection,
    donor: Arc<Donation>,
    cursor: ArcSwap<Cursor>,
}

impl SharedSelection {
    fn new(registry: &Registry, metadata: &MetadataRef) -> Self {
        let mut cohort = registry.selection(metadata.collector.clone(), metadata.size());
        if registry.active.load(Ordering::Acquire) == 0 {
            cohort.limit = 0;
        }
        Self::from_cohort(cohort)
    }

    fn from_cohort(cohort: Selection) -> Self {
        let cursor = ArcSwap::from_pointee(Cursor::Scan {
            next: 0,
            remaining: cohort.size,
        });
        Self {
            cohort,
            donor: Arc::new(Donation::default()),
            cursor,
        }
    }

    // A cursor publishes the exact request generation before any helper binds
    // it. Replaying an already won binding charges the shared cursor once. A
    // competing donor or a closed generation keeps its winner indefinitely.
    fn step(&self, budget: usize) -> Option<usize> {
        for _ in 0..budget {
            let old = self.cursor.load();
            let new = match &**old {
                Cursor::Done { remaining } => return Some(*remaining),
                Cursor::Scan { next, remaining } => {
                    if let Some((next, request)) = self.cohort.next_request(*next) {
                        let fits = request.entry().bytes.load(Ordering::Relaxed) <= *remaining
                            && request.entry().stage.load(Ordering::Acquire) != super::CLOSED;
                        if fits {
                            request.prepare();
                        }
                        if fits && let Some(attempt) = request.activate() {
                            Cursor::Bind {
                                next,
                                remaining: *remaining,
                                attempt,
                            }
                        } else {
                            Cursor::Scan {
                                next,
                                remaining: *remaining,
                            }
                        }
                    } else {
                        Cursor::Done {
                            remaining: *remaining,
                        }
                    }
                }
                Cursor::Bind {
                    next,
                    remaining,
                    attempt,
                } => {
                    let mut remaining = *remaining;
                    #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
                    crate::refund_tests::park_selection(
                        crate::refund_tests::SelectionPoint::BeforeBinding,
                    );
                    if attempt.generation().claim(&self.donor) {
                        remaining -= attempt.request.entry().bytes.load(Ordering::Relaxed);
                    }
                    #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
                    crate::refund_tests::park_selection(
                        crate::refund_tests::SelectionPoint::AfterBinding,
                    );
                    if remaining == 0 || *next == self.cohort.limit {
                        Cursor::Done { remaining }
                    } else {
                        Cursor::Scan {
                            next: *next,
                            remaining,
                        }
                    }
                }
            };
            self.cursor.compare_and_swap(&*old, Arc::new(new));
        }
        match **self.cursor.load() {
            Cursor::Done { remaining } => Some(remaining),
            _ => None,
        }
    }

    fn settlement(&self, remaining: usize) -> Selection {
        let selection = Selection {
            collector: self.cohort.collector.clone(),
            pages: self.cohort.pages.clone(),
            donor: Some(self.donor.clone()),
            cursor: self.cohort.limit,
            limit: self.cohort.limit,
            remaining,
            size: self.cohort.size,
            settled: false,
        };
        selection.mark_private();
        selection
    }
}

#[derive(Debug)]
struct Job {
    metadata: MetadataRef,
    selection: ArcSwapOption<SharedSelection>,
}

impl Job {
    fn claimed(&self) -> bool {
        self.selection
            .load()
            .as_ref()
            .is_some_and(|plan| plan.donor.private.load(Ordering::Acquire))
    }
}

pub(crate) enum ReleasedWork {
    Progress,
    Claimed(MetadataRef, Selection),
}

#[derive(Debug, Default)]
pub(crate) struct ReleasedDonors {
    published: AtomicBool,
    after: AtomicU64,
    jobs: ArcSwap<Directory<Arc<Job>>>,
}

/// A terminal allocation examines just the jobs published when it exhausted
/// its normal retry budget. Later releases cannot indefinitely delay an OOM.
pub(crate) struct ReleasedCheck {
    jobs: Option<Arc<Directory<Arc<Job>>>>,
    previous: Option<u64>,
}

impl ReleasedCheck {
    pub(crate) fn pending(&mut self) -> bool {
        if self
            .jobs
            .as_ref()
            .is_some_and(|jobs| jobs.next_after(self.previous).is_some())
        {
            true
        } else {
            self.jobs = None;
            false
        }
    }

    pub(crate) fn step(
        &mut self,
        active: &ReleasedDonors,
        registry: &Registry,
        budget: usize,
    ) -> Option<ReleasedWork> {
        let frozen = self.jobs.as_ref()?;
        let Some((id, expected)) = frozen.next_after(self.previous) else {
            self.jobs = None;
            return None;
        };
        let current = active.jobs.load_full();
        let result = if current
            .get(id)
            .is_some_and(|job| Arc::ptr_eq(job, expected))
        {
            active.help_job(&current, id, expected, registry, budget)
        } else {
            self.previous = Some(id);
            return Some(ReleasedWork::Progress);
        };
        if expected.claimed() {
            self.previous = Some(id);
        }
        Some(result)
    }
}

impl ReleasedDonors {
    #[expect(
        clippy::expect_used,
        reason = "directory cardinality is bounded by live descriptors"
    )]
    pub(crate) fn publish(&self, metadata: MetadataRef) {
        let id = metadata.id.to_i64().cast_unsigned();
        let job = Arc::new(Job {
            metadata,
            selection: ArcSwapOption::empty(),
        });
        self.published.store(true, Ordering::Release);
        self.jobs.rcu(|jobs| {
            jobs.insert(id, job.clone())
                .expect("one key per released descriptor")
                .0
        });
    }

    // A losing unlink still owns a housekeeping obligation after physical
    // retirement completes. A quiescent heap must release its snapshot pages.
    pub(crate) fn has_jobs(&self) -> bool {
        self.published.load(Ordering::Acquire) && self.jobs.load().len() != 0
    }

    pub(crate) fn snapshot(&self) -> ReleasedCheck {
        ReleasedCheck {
            jobs: Some(self.jobs.load_full()),
            previous: None,
        }
    }

    pub(crate) fn help(&self, registry: &Registry, budget: usize) -> Option<ReleasedWork> {
        let jobs = self.jobs.load();
        let after = self.after.load(Ordering::Relaxed);
        let (key, job) = jobs
            .next_after(Some(after))
            .or_else(|| jobs.next_after(None))?;
        self.after.store(key, Ordering::Relaxed);
        Some(self.help_job(&jobs, key, job, registry, budget))
    }

    fn help_job(
        &self,
        jobs: &Arc<Directory<Arc<Job>>>,
        key: u64,
        job: &Job,
        registry: &Registry,
        budget: usize,
    ) -> ReleasedWork {
        let selection = job.selection.load_full().unwrap_or_else(|| {
            let fresh = Arc::new(SharedSelection::new(registry, &job.metadata));
            let prior = job
                .selection
                .compare_and_swap(std::ptr::null::<SharedSelection>(), Some(fresh.clone()));
            prior.as_ref().cloned().unwrap_or(fresh)
        });
        let Some(remaining) = selection.step(budget) else {
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            crate::refund_tests::park_selection(crate::refund_tests::SelectionPoint::Progress);
            return ReleasedWork::Progress;
        };
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        crate::refund_tests::park_selection(crate::refund_tests::SelectionPoint::BeforeClaim);
        // The per-donor transition elects exactly one physical owner. An
        // unrelated insertion can delay unlinking but cannot revoke this claim.
        let won = selection
            .donor
            .private
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok();
        let (next, _) = jobs.remove(key);
        self.jobs.compare_and_swap(jobs, Arc::new(next));
        if !won {
            return ReleasedWork::Progress;
        }
        ReleasedWork::Claimed(job.metadata.clone(), selection.settlement(remaining))
    }

    pub(crate) fn cleanup(&self) {
        let jobs = self.jobs.swap(Arc::default());
        let mut cursor = None;
        while let Some((id, job)) = jobs.next_after(cursor) {
            cursor = Some(id);
            if !job.claimed() {
                job.metadata.run_finalizer();
                job.metadata.run_drop_fn();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Configuration, GarbageCollector, Result};

    fn heap(size: usize) -> Result<Arc<GarbageCollector>> {
        let gc =
            GarbageCollector::with_config(Configuration::builder().max_heap_bytes(size).build());
        gc.reserve_allocation(size)?;
        Ok(gc)
    }

    fn shared(gc: &Arc<GarbageCollector>, size: usize) -> SharedSelection {
        SharedSelection::from_cohort(gc.grant_registry.selection(Arc::downgrade(gc), size))
    }

    fn settle(selection: &SharedSelection, gc: &GarbageCollector) -> usize {
        let remaining = (0..64)
            .find_map(|_| selection.step(1))
            .expect("bounded three-entry scan");
        let mut settlement = selection.settlement(remaining);
        let paid = settlement.pay();
        gc.cancel_allocation(settlement.size - paid);
        paid
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "verify exact charges for two competing donors"
    )]
    fn a_competing_donor_backfills_the_second_fitting_request() -> Result<()> {
        let gc = heap(40)?;
        let first = gc.grant_registry.register(&gc, 16)?;
        let second = gc.grant_registry.register(&gc, 16)?;
        let third = gc.grant_registry.register(&gc, 8)?;
        let large = shared(&gc, 24);
        assert!(large.step(1).is_none());
        let small = shared(&gc, 16);
        let late = gc.grant_registry.register(&gc, 8)?;
        assert_eq!(settle(&small, &gc), 16);
        assert_eq!(settle(&large, &gc), 24);
        assert!(late.finish_uncovered());
        assert_eq!(gc.managed_bytes(), 40);
        for request in [&first, &second, &third] {
            drop(request.take().ok_or(crate::Error::OutOfMemory)?);
        }
        assert_eq!(gc.managed_bytes(), 0);
        Ok(())
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "verify stale intent from a previous generation"
    )]
    fn a_published_bind_cannot_claim_a_reopened_generation() -> Result<()> {
        let gc = heap(32)?;
        let first = gc.grant_registry.register(&gc, 16)?;
        let second = gc.grant_registry.register(&gc, 16)?;
        let released = shared(&gc, 16);
        assert!(released.step(1).is_none());
        let mut ordinary = gc.grant_registry.selection(Arc::downgrade(&gc), 16);
        assert!(ordinary.advance(&mut 0));
        drop(ordinary);
        assert!(first.take().is_none());
        assert_eq!(settle(&released, &gc), 16);
        assert!(first.request.waiting());
        drop(second.take().ok_or(crate::Error::OutOfMemory)?);
        assert_eq!(settle(&shared(&gc, 16), &gc), 16);
        drop(first.take().ok_or(crate::Error::OutOfMemory)?);
        assert_eq!(gc.managed_bytes(), 0);
        Ok(())
    }
}

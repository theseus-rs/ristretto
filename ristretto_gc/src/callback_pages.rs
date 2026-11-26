//! A native sweep worker publishes jobs to its own lane. Claiming a job never
//! locks the publisher. A full bank gains a successor, leaving each stopped
//! claimant with one job or one bank while peers advance and reclaim the rest.
use arc_swap::{ArcSwap, ArcSwapOption};
use parking_lot::Mutex;
use std::cell::{Cell, UnsafeCell};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};

const SLOTS: usize = 32;

const MAX_SLOTS: usize = 1024;

struct Slot<T> {
    value: UnsafeCell<Option<T>>,
    occupied: AtomicBool,
}

// SAFETY: the sole publisher checks the previous owner's release before writing.
// Only the owner of the non-repeating claim number may take the published value.
#[expect(unsafe_code)]
unsafe impl<T: Send> Sync for Slot<T> {}

struct Page<T> {
    slots: Box<[Slot<T>]>,
    published: AtomicUsize,
    claimed: AtomicUsize,
    next: ArcSwapOption<Self>,
}

impl<T> Page<T> {
    fn new(capacity: usize) -> Arc<Self> {
        Arc::new(Self {
            slots: (0..capacity)
                .map(|_| Slot {
                    value: UnsafeCell::new(None),
                    occupied: AtomicBool::new(false),
                })
                .collect(),
            published: AtomicUsize::new(0),
            claimed: AtomicUsize::new(0),
            next: ArcSwapOption::empty(),
        })
    }
}

impl<T> Page<T> {
    fn has_available(&self) -> bool {
        self.claimed.load(Ordering::Relaxed) < self.published.load(Ordering::Acquire)
    }

    fn take(&self) -> Option<T> {
        let mut index = self.claimed.load(Ordering::Relaxed);
        while index < self.published.load(Ordering::Acquire) {
            #[cfg(all(test, not(miri), panic = "unwind"))]
            super::claim_pause::at(false);
            match self.claimed.compare_exchange_weak(
                index,
                index + 1,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    #[cfg(all(test, not(miri), panic = "unwind"))]
                    super::claim_pause::at(true);
                    let slot = self.slots.get(index % self.slots.len())?;
                    // SAFETY: tail acquisition sees initialization, the head
                    // CAS gives one owner, and the publisher requires release.
                    #[expect(unsafe_code)]
                    let value = unsafe { (*slot.value.get()).take() };
                    slot.occupied.store(false, Ordering::Release);
                    return value;
                }
                Err(now) => index = now,
            }
        }
        None
    }
}

impl<T> Drop for Page<T> {
    fn drop(&mut self) {
        // Shutdown can own an undrained suffix. Avoid recursive destruction
        // even when the last consumer disappeared with a long queue.
        let mut next = self.next.swap(None);
        while let Some(page) = next.and_then(Arc::into_inner) {
            next = page.next.swap(None);
        }
    }
}

struct Lane<T> {
    head: ArcSwap<Page<T>>,
    tail: Mutex<Arc<Page<T>>>,
}

impl<T> Lane<T> {
    fn new() -> Self {
        let page = Page::new(SLOTS);
        Self {
            head: ArcSwap::from(page.clone()),
            tail: Mutex::new(page),
        }
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "the index is reduced modulo nonzero capacity"
    )]
    fn push(&self, jobs: impl Iterator<Item = T>) {
        let mut tail = self.tail.lock();
        for job in jobs {
            let mut index = tail.published.load(Ordering::Relaxed);
            if index == usize::MAX
                || tail.slots[index % tail.slots.len()]
                    .occupied
                    .load(Ordering::Acquire)
            {
                // Publish a successor instead of waiting for a returned slot.
                let next = Page::new((2 * tail.slots.len()).min(MAX_SLOTS));
                tail.next.store(Some(next.clone()));
                *tail = next;
                index = 0;
            }
            let slot = &tail.slots[index % tail.slots.len()];
            // SAFETY: the publisher lock and the prior owner's release give
            // exclusive access until this publisher exposes the ticket.
            #[expect(unsafe_code)]
            unsafe {
                *slot.value.get() = Some(job);
            }
            slot.occupied.store(true, Ordering::Relaxed);
            tail.published.store(index + 1, Ordering::Release);
        }
    }

    #[cfg(test)]
    fn claim(&self) -> Option<T> {
        self.claim_observed(|_| {})
    }

    fn claim_observed(&self, observed: impl FnOnce(&Arc<Page<T>>)) -> Option<T> {
        loop {
            let page = self.head.load();
            if let Some(value) = page.take() {
                observed(&page);
                return Some(value);
            }
            if let Some(next) = page.next.load_full() {
                // Observing the seal acquires every earlier publication to this
                // page. The preceding empty check could have read an older tail.
                if page.claimed.load(Ordering::Relaxed) < page.published.load(Ordering::Acquire) {
                    continue;
                }
                if Arc::ptr_eq(&self.head.compare_and_swap(&page, next), &page) {
                    // A stopped claimant can keep its old page, but no live chain.
                    page.next.store(None);
                }
            } else if Arc::ptr_eq(&self.head.load(), &page) {
                return None;
            }
        }
    }
}

// A stopped consumer pins at most one bank and never reserves its next job.
// Rotate after 16 successful claims when other lanes can also have work.
pub(super) struct Cursor<'a, T> {
    queues: &'a Pages<T>,
    page: Option<Arc<Page<T>>>,
    next_lane: usize,
    left: usize,
}

impl<T> Cursor<'_, T> {
    pub(super) fn claim(&mut self) -> Option<T> {
        if (self.queues.lanes.len() == 1 || self.left != 0)
            && let Some(page) = &self.page
            && let Some(value) = page.take()
        {
            self.left = self.left.saturating_sub(1);
            return Some(value);
        }
        self.page = None;
        self.queues.scan(self.next_lane, |next_lane, page| {
            self.next_lane = next_lane;
            self.page = Some(page.clone());
            self.left = 15;
        })
    }
}

thread_local! {
    static NEXT_LANE: Cell<usize> = const { Cell::new(0) };
}

pub(super) struct Pages<T> {
    lanes: Box<[OnceLock<Lane<T>>]>,
}

impl<T> Pages<T> {
    pub(super) fn new(workers: usize) -> Self {
        Self {
            lanes: (0..workers.max(1)).map(|_| OnceLock::new()).collect(),
        }
    }

    pub(super) fn push(&self, producer: usize, first: T, rest: impl Iterator<Item = T>) {
        #[expect(
            clippy::indexing_slicing,
            reason = "the worker index belongs to this pool"
        )]
        let lane = &self.lanes[producer];
        lane.get_or_init(Lane::new)
            .push(std::iter::once(first).chain(rest));
    }

    pub(super) fn has_available(&self) -> bool {
        self.lanes.iter().filter_map(OnceLock::get).any(|lane| {
            let page = lane.head.load();
            if page.has_available() {
                return true;
            }
            // A publisher may stop after sealing the old bank but before
            // publishing its first successor job. Acquiring the link also
            // orders the final old-bank publication before this recheck.
            let next = page.next.load();
            next.as_ref()
                .is_some_and(|bank| page.has_available() || bank.has_available())
                || !Arc::ptr_eq(&page, &lane.head.load())
        })
    }

    pub(super) fn cursor(&self) -> Cursor<'_, T> {
        Cursor {
            queues: self,
            page: None,
            next_lane: 0,
            left: 0,
        }
    }

    pub(super) fn claim(&self) -> Option<T> {
        NEXT_LANE
            .with(|cursor| self.scan(cursor.get() % self.lanes.len(), |next, _| cursor.set(next)))
    }

    fn scan(&self, start: usize, mut observed: impl FnMut(usize, &Arc<Page<T>>)) -> Option<T> {
        if let [only] = self.lanes.as_ref() {
            return only.get()?.claim_observed(|page| observed(0, page));
        }
        let (before, after) = self.lanes.split_at(start);
        for (offset, slot) in after.iter().chain(before).enumerate() {
            let next = (start + offset + 1) % self.lanes.len();
            if let Some(job) = slot
                .get()
                .and_then(|lane| lane.claim_observed(|page| observed(next, page)))
            {
                return Some(job);
            }
        }
        None
    }

    #[cfg(all(test, not(miri), panic = "unwind"))]
    pub(super) fn remaining_in_head(&self) -> usize {
        self.lanes
            .iter()
            .filter_map(OnceLock::get)
            .map(|lane| {
                let head = lane.head.load();
                head.published.load(Ordering::Acquire) - head.claimed.load(Ordering::Acquire)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn a_successor_link_becomes_dispatchable_only_after_a_job_is_published() {
        let jobs = Pages::new(1);
        jobs.push(0, 1, std::iter::empty());
        assert_eq!(jobs.claim(), Some(1));
        let lane = jobs.lanes.first().unwrap().get().unwrap();
        let next = Page::new(2 * SLOTS);
        lane.head.load().next.store(Some(next.clone()));
        *lane.tail.lock() = next;
        assert!(!jobs.has_available());
        jobs.push(0, 2, std::iter::empty());
        assert!(jobs.has_available());
        assert_eq!(jobs.claim(), Some(2));
        assert!(!jobs.has_available());
    }

    #[test]
    fn a_cached_consumer_rotates_between_busy_lanes() {
        let jobs = Pages::new(2);
        jobs.push(0, 0, 1..48);
        jobs.push(1, 100, 101..148);
        let mut cursor = jobs.cursor();
        for first in [0, 100, 16, 116, 32, 132] {
            assert!((first..first + 16).all(|value| cursor.claim() == Some(value)));
        }
        assert_eq!(cursor.claim(), None);
    }

    #[test]
    fn a_paused_cursor_cannot_replay_old_jobs_or_pin_intermediate_banks() {
        let jobs = Pages::new(1);
        jobs.push(0, 0, std::iter::empty());
        let mut paused = jobs.cursor();
        assert_eq!(paused.claim(), Some(0));
        let first = Arc::downgrade(paused.page.as_ref().unwrap());
        let lane = jobs.lanes.first().unwrap().get().unwrap();
        let mut retired = Vec::new();
        let mut next = 1;
        for _ in 0..6 {
            let old = Arc::downgrade(&lane.tail.lock());
            let count = lane.tail.lock().slots.len() + 1;
            lane.push(next..next + count);
            if !old.ptr_eq(&first) {
                retired.push(old);
            }
            next += count;
        }
        assert!(std::iter::from_fn(|| jobs.claim()).eq(1..next));
        assert!(retired.iter().all(|page| page.upgrade().is_none()));
        assert!(first.upgrade().unwrap().next.load().is_none());
        assert!(lane.tail.lock().slots.len() <= MAX_SLOTS);
        jobs.push(0, next, std::iter::empty());
        assert_eq!(paused.claim(), Some(next));
        assert!(first.upgrade().is_none());
    }

    #[test]
    fn a_full_ticket_range_rolls_over_to_a_new_bank() {
        let lane = Lane::new();
        let first = lane.head.load_full();
        first.published.store(usize::MAX - 1, Ordering::Relaxed);
        first.claimed.store(usize::MAX - 1, Ordering::Relaxed);
        lane.push(0..3);
        assert!(std::iter::from_fn(|| lane.claim()).eq(0..3));
        assert!(!Arc::ptr_eq(&first, &lane.head.load_full()));
    }

    #[test]
    fn racing_takers_return_each_owned_value_once_across_reuse_and_growth() {
        let lane = Lane::new();
        let done = AtomicBool::new(false);
        let take = || {
            let mut owned = Vec::new();
            loop {
                if let Some(value) = lane.claim() {
                    owned.push(value);
                } else if done.load(Ordering::Acquire) {
                    owned.extend(std::iter::from_fn(|| lane.claim()));
                    return owned;
                } else {
                    thread::yield_now();
                }
            }
        };
        let mut collected = thread::scope(|scope| {
            let a = scope.spawn(take);
            let b = scope.spawn(take);
            for index in 0..160 {
                lane.push(std::iter::once(Box::new(index)));
                if index % 16 == 0 {
                    thread::yield_now();
                }
            }
            done.store(true, Ordering::Release);
            let mut values = a.join().unwrap();
            values.extend(b.join().unwrap());
            values
        });
        collected.sort_unstable();
        assert!(collected.into_iter().map(|value| *value).eq(0..160));
    }

    #[test]
    fn parked_producer_leaves_its_published_prefix_and_other_producers_available() {
        let queues = Pages::new(2);
        let (park, observed) = mpsc::channel();
        let (resume, release) = mpsc::channel();
        thread::scope(|scope| {
            let queue = &queues;
            let publisher = scope.spawn(move || {
                queue.push(
                    0,
                    0,
                    std::iter::once_with(|| {
                        park.send(()).unwrap();
                        release.recv_timeout(Duration::from_secs(5)).unwrap();
                        1
                    }),
                );
            });
            observed.recv_timeout(Duration::from_secs(5)).unwrap();
            queues.push(1, 2, 3..(3 * SLOTS));
            let jobs: Vec<_> = std::iter::from_fn(|| queues.claim()).collect();
            // The peer lane stays FIFO even if the consumer rotates lanes.
            assert_eq!(jobs.iter().filter(|&&n| n == 0).count(), 1);
            assert!(jobs.into_iter().filter(|&n| n != 0).eq(2..3 * SLOTS));
            resume.send(()).unwrap();
            publisher.join().unwrap();
        });
        assert_eq!(queues.claim(), Some(1));
        assert_eq!(queues.claim(), None);
    }

    #[cfg(all(not(miri), panic = "unwind"))]
    #[test]
    fn a_parked_claim_at_the_page_boundary_keeps_only_its_own_page() {
        use crate::collector::native_callbacks::claim_pause::{PAUSE, Pause};
        for after in [false, true] {
            let lane = Lane::new();
            lane.push(0..SLOTS);
            for index in 0..SLOTS - 1 {
                assert_eq!(lane.claim(), Some(index));
            }
            let first = Arc::downgrade(&lane.head.load_full());
            let (park, observed) = mpsc::channel();
            let (resume, release) = mpsc::channel();
            thread::scope(|scope| {
                let queue = &lane;
                let claimant = scope.spawn(move || {
                    PAUSE.with_borrow_mut(|hook| {
                        *hook = Some(Pause {
                            after,
                            entered: park,
                            release,
                        });
                    });
                    queue.claim()
                });
                observed.recv_timeout(Duration::from_secs(5)).unwrap();
                let mut pages = Vec::new();
                let mut end = SLOTS;
                // Fill and cross multiple banks, even when the initial one
                // can still reuse slots returned before the paused claim.
                for _ in 0..6 {
                    let old = Arc::downgrade(&lane.tail.lock());
                    let count = lane.tail.lock().slots.len() + 1;
                    lane.push(end..end + count);
                    end += count;
                    if !old.ptr_eq(&first) {
                        pages.push(old);
                    }
                }
                let jobs = std::iter::from_fn(|| lane.claim());
                assert!(jobs.eq((SLOTS - usize::from(!after))..end));
                assert!(first.upgrade().is_some());
                assert!(pages.iter().all(|p| p.upgrade().is_none()));
                resume.send(()).unwrap();
                assert_eq!(claimant.join().unwrap(), after.then_some(SLOTS - 1));
            });
            assert!(first.upgrade().is_none());
        }
    }
}

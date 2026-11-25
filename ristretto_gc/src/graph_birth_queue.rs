//! Reusable slots for graph births in one strong epoch, with linked overflow.
//! A stalled producer owns only an unpublished slot. Consumers acquire a ready
//! bit before touching the value, then release the free bit after unlocking.
use crate::publication_queue;
use once_cell::race::OnceBox;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const WIDTH: usize = usize::BITS as usize;
const SLOTS: usize = 512;
const PAGES: usize = SLOTS / WIDTH;

#[derive(Debug)]
struct Page<T> {
    free: AtomicUsize,
    ready: AtomicUsize,
    slots: [Mutex<Option<T>>; WIDTH],
}

impl<T> Page<T> {
    fn new() -> Self {
        Self {
            free: AtomicUsize::new(usize::MAX),
            ready: AtomicUsize::new(0),
            slots: std::array::from_fn(|_| Mutex::new(None)),
        }
    }
}

#[derive(Debug)]
pub(super) struct Queue<T> {
    pages: [OnceBox<Page<T>>; PAGES],
    producer: AtomicUsize,
    consumer: AtomicUsize,
    reserved: AtomicUsize,
    overflow: OnceBox<publication_queue::Queue<T>>,
    overflow_turn: AtomicBool,
}

#[expect(
    clippy::indexing_slicing,
    reason = "indices are masked to the fixed power-of-two tables"
)]
impl<T> Queue<T> {
    pub(super) fn new() -> Self {
        Self {
            pages: std::array::from_fn(|_| OnceBox::new()),
            producer: AtomicUsize::new(0),
            consumer: AtomicUsize::new(0),
            reserved: AtomicUsize::new(0),
            overflow: OnceBox::new(),
            overflow_turn: AtomicBool::new(false),
        }
    }

    pub(super) fn push(&self, value: T) {
        self.push_with(value, || {});
    }

    fn push_with(&self, value: T, before_ready: impl FnOnce()) {
        self.reserved.fetch_add(1, Ordering::Relaxed);
        let pos = self.producer.fetch_add(1, Ordering::Relaxed) & (SLOTS - 1);
        let page = self.pages[pos / WIDTH].get_or_init(|| Box::new(Page::new()));
        let slot = pos % WIDTH;
        let bit = 1 << slot;
        if page.free.fetch_and(!bit, Ordering::Acquire) & bit == 0 {
            self.overflow
                .get_or_init(|| Box::new(publication_queue::Queue::new()))
                .push(value);
            return;
        }
        // Only the free-bit winner can acquire this mutex until ready is set.
        {
            let mut value_slot = super::lock(&page.slots[slot]);
            *value_slot = Some(value);
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            crate::collector::pause_while_graph_slot_owned();
        }
        before_ready();
        page.ready.fetch_or(bit, Ordering::Release);
    }

    pub(super) fn pop(&self) -> Option<T> {
        if self.pending() == 0 {
            return None;
        }
        let result = self.pop_reserved();
        if result.is_some() {
            self.reserved.fetch_sub(1, Ordering::Relaxed);
        }
        result
    }

    fn pop_reserved(&self) -> Option<T> {
        let overflow = self.overflow.get().filter(|q| q.pending() != 0);
        if let Some(queue) = overflow
            && !self.overflow_turn.fetch_xor(true, Ordering::Relaxed)
            && let Some(value) = queue.pop()
        {
            return Some(value);
        }
        let start = self.consumer.fetch_add(1, Ordering::Relaxed) & (SLOTS - 1);
        for offset in 0..PAGES {
            let Some(page) = self.pages[(start / WIDTH + offset) % PAGES].get() else {
                continue;
            };
            let ready = page.ready.load(Ordering::Acquire);
            if ready == 0 {
                continue;
            }
            let from = u32::from((start % WIDTH).to_le_bytes()[0]);
            let slot = (ready.rotate_right(from).trailing_zeros() + from) as usize % WIDTH;
            let bit = 1 << slot;
            if page.ready.fetch_and(!bit, Ordering::Acquire) & bit == 0 {
                continue;
            }
            // No producer can reserve this slot until the value has been taken
            // and the mutex unlocked. Each reservation has exactly one consumer.
            let value = super::lock(&page.slots[slot]).take();
            page.free.fetch_or(bit, Ordering::Release);
            return value;
        }
        overflow.and_then(publication_queue::Queue::pop)
    }

    pub(super) fn pending(&self) -> usize {
        self.reserved.load(Ordering::Acquire)
    }

    pub(super) fn capacity(&self) -> usize {
        self.pages.iter().filter_map(OnceBox::get).count() * WIDTH
            + self
                .overflow
                .get()
                .map_or(0, |queue| queue.retained_nodes().saturating_sub(2))
    }

    pub(super) fn is_empty(&self) -> bool {
        // A reservation can precede its link indefinitely. Closure checks the
        // links under the phase observation that a successful graph must stamp.
        self.pending() == 0
            || self
                .pages
                .iter()
                .filter_map(OnceBox::get)
                .all(|p| p.ready.load(Ordering::Acquire) == 0)
                && self
                    .overflow
                    .get()
                    .is_none_or(publication_queue::Queue::is_empty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unfinished_slot_allows_wraparound_overflow_and_complete_drain() {
        let queue = Queue::new();
        assert_eq!(queue.capacity(), 0);
        queue.push_with(0, || {
            assert_eq!(queue.capacity(), WIDTH);
            assert!(queue.is_empty());
            assert!(queue.pop().is_none());
            for i in 1..=2 * SLOTS {
                queue.push(i);
            }
            assert!(queue.capacity() >= 2 * SLOTS);
            let mut seen = vec![false; 2 * SLOTS];
            for _ in 0..seen.len() {
                let i = queue.pop().expect("one linked entry");
                let entry = seen.get_mut(i - 1).expect("private slot was claimed");
                assert!(!std::mem::replace(entry, true));
            }
            assert!(seen.into_iter().all(|v| v));
            assert!(queue.is_empty());
        });
        assert_eq!(queue.pop(), Some(0));
        assert_eq!(queue.pending(), 0);
        assert_eq!(queue.capacity(), SLOTS);
    }
    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn producers_and_consumers_transfer_each_reused_slot_exactly_once() {
        const THREADS: usize = 4;
        const EACH: usize = 4 * SLOTS;
        let queue = Queue::new();
        let seen = (0..THREADS * EACH)
            .map(|_| AtomicBool::new(false))
            .collect::<Vec<_>>();
        let done = AtomicUsize::new(0);
        std::thread::scope(|scope| {
            for producer in 0..THREADS {
                let producer_queue = &queue;
                let finished = &done;
                scope.spawn(move || {
                    for n in 0..EACH {
                        producer_queue.push(producer * EACH + n);
                    }
                    finished.fetch_add(1, Ordering::Release);
                });
                scope.spawn(|| {
                    loop {
                        if let Some(n) = queue.pop() {
                            assert!(
                                !seen
                                    .get(n)
                                    .expect("in-range ticket")
                                    .swap(true, Ordering::Relaxed)
                            );
                        } else if done.load(Ordering::Acquire) == THREADS && queue.pending() == 0 {
                            break;
                        } else {
                            std::thread::yield_now();
                        }
                    }
                });
            }
        });
        assert!(queue.is_empty());
        assert!(seen.into_iter().all(AtomicBool::into_inner));
    }
}

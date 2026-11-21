use super::*;
use std::collections::HashSet;

fn source(next: u64) -> Source {
    Source {
        next: AtomicU64::new(next),
    }
}

#[expect(clippy::expect_used, reason = "test-only literal nonzero lease sizes")]
fn count(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).expect("test requests are nonzero")
}

// Test-only retry driver. Production callers must expose Pending and yield
// outside the native frame/table borrow instead of copying this bounded loop.
fn acquire(source: &Source, amount: u64) -> Result<TokenRange, Box<dyn std::error::Error>> {
    for _ in 0..10_000 {
        if let Poll::Ready(result) = source.try_acquire(count(amount)) {
            return result.map_err(Into::into);
        }
        std::hint::spin_loop();
    }
    Err("test reservation retry budget exceeded".into())
}

#[test]
fn zero_is_null_and_local_issuance_does_not_touch_source() {
    let source = source(1);
    let mut range = acquire(&source, 3).expect("lease");
    assert_eq!(range.remaining(), 3);
    assert_eq!(source.next.load(Ordering::Relaxed), 4);
    assert_eq!(range.issue(), Some(1));
    assert_eq!(range.issue(), Some(2));
    assert_eq!(range.issue(), Some(3));
    assert_eq!(range.issue(), None);
    assert_eq!(range.issue(), None);
    assert_eq!(range.remaining(), 0);
    assert_eq!(source.next.load(Ordering::Relaxed), 4);
}

#[test]
fn final_partial_range_issues_i64_max_then_exhausts_without_wrapping() {
    let source = source(END_EXCLUSIVE - 2);
    let mut range = acquire(&source, u64::MAX).expect("partial lease");
    assert_eq!(range.remaining(), 2);
    assert_eq!(range.issue(), Some(i64::MAX - 1));
    assert_eq!(range.issue(), Some(i64::MAX));
    assert_eq!(range.issue(), None);
    for _ in 0..3 {
        assert!(matches!(
            source.try_acquire(count(1)),
            Poll::Ready(Err(LeaseError::Exhausted))
        ));
        assert_eq!(source.next.load(Ordering::Relaxed), END_EXCLUSIVE);
    }
}

#[test]
fn a_single_invocation_can_cross_the_previous_u32_ceiling() {
    let source = source(u64::from(u32::MAX) - 1);
    let mut first = acquire(&source, 2).expect("first lease");
    assert_eq!(first.issue(), Some(i64::from(u32::MAX) - 1));
    assert_eq!(first.issue(), Some(i64::from(u32::MAX)));
    let mut second = acquire(&source, 2).expect("refill lease");
    assert_eq!(second.issue(), Some(i64::from(u32::MAX) + 1));
    assert_eq!(second.issue(), Some(i64::from(u32::MAX) + 2));
}

#[test]
fn abandoned_and_issued_ids_are_never_leased_again() {
    let source = source(1);
    let stale = {
        let mut first = acquire(&source, 8).expect("first lease");
        first.issue().expect("first token")
    };
    let mut second = acquire(&source, 2).expect("second lease");
    assert_eq!(second.issue(), Some(9));
    assert_eq!(second.issue(), Some(10));
    assert_eq!(source.next.load(Ordering::Relaxed), 11);
    assert_ne!(stale, 9);
    // IDs 2..=8 were never issued, but dropping their lease did not recycle them.
}

#[test]
fn stale_observation_returns_pending_without_consuming_a_second_range() {
    let source = source(1);
    let stale_observation = source.next.load(Ordering::Relaxed);
    let _winner = acquire(&source, 4).expect("winning lease");
    assert!(
        source
            .try_acquire_observed(stale_observation, count(9))
            .is_pending()
    );
    assert_eq!(source.next.load(Ordering::Relaxed), 5);
    let mut retry = acquire(&source, 2).expect("host retries");
    assert_eq!(retry.issue(), Some(5));
    assert_eq!(retry.issue(), Some(6));
}

#[test]
fn contender_losing_to_final_lease_observes_pending_then_terminal_exhaustion() {
    let source = source(END_EXCLUSIVE - 1);
    let observed = source.next.load(Ordering::Relaxed);
    let mut winner = acquire(&source, 1).expect("last lease");
    assert!(source.try_acquire_observed(observed, count(1)).is_pending());
    assert!(matches!(
        source.try_acquire(count(1)),
        Poll::Ready(Err(LeaseError::Exhausted))
    ));
    assert_eq!(winner.issue(), Some(i64::MAX));
}

#[test]
fn enormous_lease_uses_constant_storage_and_preserves_the_positive_domain() {
    let source = source(1);
    let mut range = acquire(&source, u64::MAX).expect("entire domain");
    assert_eq!(size_of_val(&range), 2 * size_of::<u64>());
    assert_eq!(range.remaining(), END_EXCLUSIVE - 1);
    assert_eq!(range.issue(), Some(1));
    assert!(matches!(
        source.try_acquire(count(1)),
        Poll::Ready(Err(LeaseError::Exhausted))
    ));
}

#[test]
fn separate_owners_and_reused_live_slots_cannot_alias_foreign_or_stale_tokens() {
    let source = source(1);
    let mut owner_a = acquire(&source, 2).expect("a lease");
    let mut owner_b = acquire(&source, 2).expect("b lease");
    let stale = owner_a.issue().expect("a first");
    let foreign = owner_b.issue().expect("b first");
    // Model only exact-entry lookup. Real root liveness and frame identity are
    // still the frame table's responsibility, not an allocator guarantee.
    let mut live_slot = Some(stale);
    assert_ne!(live_slot, Some(foreign));
    live_slot = owner_a.issue();
    assert_ne!(live_slot, Some(stale));
    assert_ne!(live_slot, Some(foreign));
    let mut refill = acquire(&source, 2).expect("a refill");
    live_slot = refill.issue();
    assert_ne!(live_slot, Some(stale));
    assert_ne!(live_slot, Some(foreign));
}

#[test]
fn process_source_is_shared_across_reservations() {
    fn global_lease() -> TokenRange {
        for _ in 0..10_000 {
            match TokenRange::try_acquire(count(3)) {
                Poll::Ready(Ok(range)) => return range,
                Poll::Ready(Err(error)) => panic!("unexpected exhaustion: {error}"),
                Poll::Pending => std::hint::spin_loop(),
            }
        }
        panic!("global source retry budget exceeded");
    }
    let mut first = global_lease();
    let mut second = global_lease();
    let first_ids: HashSet<_> = std::iter::from_fn(|| first.issue()).collect();
    let second_ids: HashSet<_> = std::iter::from_fn(|| second.issue()).collect();
    assert_eq!(first_ids.len(), 3);
    assert_eq!(second_ids.len(), 3);
    assert!(first_ids.is_disjoint(&second_ids));
    assert!(first_ids.iter().chain(&second_ids).all(|value| *value > 0));
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn concurrent_range_owners_issue_unique_positive_tokens() {
    let source = source(1);
    let barrier = std::sync::Barrier::new(4);
    let tokens = std::thread::scope(|scope| {
        let threads: Vec<_> = (0..4)
            .map(|_| {
                scope.spawn(|| {
                    barrier.wait();
                    let mut tokens = Vec::new();
                    for _ in 0..32 {
                        let mut range = acquire(&source, 7).expect("concurrent lease");
                        tokens.extend(std::iter::from_fn(|| range.issue()));
                    }
                    tokens
                })
            })
            .collect();
        threads
            .into_iter()
            .flat_map(|thread| thread.join().expect("worker"))
            .collect::<Vec<_>>()
    });
    let unique: HashSet<_> = tokens.iter().copied().collect();
    assert_eq!(tokens.len(), 4 * 32 * 7);
    assert_eq!(tokens.len(), unique.len());
    assert!(unique.iter().all(|token| *token > 0));
    assert_eq!(source.next.load(Ordering::Relaxed), 4 * 32 * 7 + 1);
}

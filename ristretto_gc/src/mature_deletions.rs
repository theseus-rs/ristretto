//! Overflow traversal for mature numeric edges in a strong old-only snapshot.
//! Each stripe belongs to one group of nonrecycled object identities for the
//! lifetime of this epoch. Setting its bit publishes all the work for that ID.
use crate::ObjectId;
use once_cell::race::OnceBox;
use portable_atomic::AtomicU64;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};

const STRIPES: usize = 64;
const WIDTH: u8 = 63;
const SEALED: u64 = 1 << WIDTH;
const UNUSED: u8 = 0;
const ACTIVE: u8 = 1;
const CLOSED: u8 = 2;

#[derive(Debug, Default)]
struct Stripe {
    group: AtomicU64,
    bits: AtomicU64,
    // This optional index uses a collision chain. Save an address hint on that
    // build so the marking claim need not traverse it while holding admission.
    #[cfg(feature = "experimental-identity-index")]
    addresses: OnceBox<[AtomicUsize; WIDTH as usize]>,
}

#[derive(Debug)]
struct Table {
    stripes: [Stripe; STRIPES],
    cursor: AtomicUsize,
    claim_turn: AtomicBool,
}

impl Table {
    fn stripe(&self, group: u64) -> Option<&Stripe> {
        let start = usize::try_from(group % STRIPES as u64).ok()?;
        // Stable tags and a common probe order give concurrent publishers of
        // the same ID a single slot. Only a full table needs the slow path.
        let tag = group + 1;
        for offset in 0..STRIPES {
            let stripe = self.stripes.get((start + offset) % STRIPES)?;
            match stripe.group.load(Ordering::Acquire) {
                existing if existing == tag => return Some(stripe),
                0 if !stripe
                    .group
                    .compare_exchange(0, tag, Ordering::AcqRel, Ordering::Acquire)
                    .is_err_and(|existing| existing != tag) =>
                {
                    return Some(stripe);
                }
                _ => {}
            }
        }
        None
    }
}

impl Default for Table {
    fn default() -> Self {
        Self {
            stripes: std::array::from_fn(|_| Stripe::default()),
            cursor: AtomicUsize::new(0),
            claim_turn: AtomicBool::new(false),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct MatureDeletions {
    state: AtomicU8,
    table: OnceBox<Table>,
}

pub(crate) struct Target {
    pub(crate) id: ObjectId,
    #[cfg(feature = "experimental-identity-index")]
    pub(crate) address: usize,
}

impl MatureDeletions {
    pub(crate) fn offer(&self, id: ObjectId, address: usize) -> bool {
        self.prepare(id, address)
            .is_some_and(|(word, bit)| Self::publish(word, bit))
    }

    fn prepare(&self, id: ObjectId, address: usize) -> Option<(&AtomicU64, u64)> {
        #[cfg(not(feature = "experimental-identity-index"))]
        let _ = address;
        if self.state.load(Ordering::Acquire) == CLOSED {
            return None;
        }
        let table = self.table.get_or_init(Box::default);
        // Install the table before advertising ACTIVE. A closer can seal an
        // unused epoch while a producer is stopped inside table initialization.
        if self.state.load(Ordering::Acquire) != ACTIVE
            && self
                .state
                .compare_exchange(UNUSED, ACTIVE, Ordering::AcqRel, Ordering::Acquire)
                == Err(CLOSED)
        {
            return None;
        }
        let raw = id.to_i64().cast_unsigned();
        let group = raw / u64::from(WIDTH);
        let stripe = table.stripe(group)?;
        let offset = raw % u64::from(WIDTH);
        #[cfg(feature = "experimental-identity-index")]
        {
            let addresses = stripe
                .addresses
                .get_or_init(|| Box::new(std::array::from_fn(|_| AtomicUsize::new(0))));
            addresses
                .get(usize::try_from(offset).ok()?)?
                .store(address, Ordering::Relaxed);
        }
        Some((&stripe.bits, 1 << offset))
    }

    fn publish(word: &AtomicU64, bit: u64) -> bool {
        word.fetch_update(Ordering::AcqRel, Ordering::Acquire, |bits| {
            (bits & SEALED == 0).then_some(bits | bit)
        })
        .is_ok()
    }

    pub(crate) fn prefer_claim(&self) -> bool {
        self.state.load(Ordering::Acquire) == ACTIVE
            && self
                .table
                .get()
                .is_some_and(|t| t.claim_turn.fetch_xor(true, Ordering::Relaxed))
    }

    /// Claim at most one identity. Only marking claimants may call this method.
    pub(crate) fn take(&self) -> Option<Target> {
        if self.state.load(Ordering::Acquire) != ACTIVE {
            return None;
        }
        let table = self.table.get()?;
        let start = table.cursor.fetch_add(1, Ordering::Relaxed);
        for offset in 0..STRIPES {
            let stripe = table.stripes.get(start.wrapping_add(offset) % STRIPES)?;
            let bits = stripe.bits.load(Ordering::Acquire) & !SEALED;
            if bits == 0 {
                continue;
            }
            let slot = bits.trailing_zeros();
            let bit = 1 << slot;
            if stripe.bits.fetch_and(!bit, Ordering::AcqRel) & bit == 0 {
                continue;
            }
            let raw =
                (stripe.group.load(Ordering::Acquire) - 1) * u64::from(WIDTH) + u64::from(slot);
            return Some(Target {
                id: ObjectId::from_i64(raw.cast_signed()),
                #[cfg(feature = "experimental-identity-index")]
                address: stripe
                    .addresses
                    .get()?
                    .get(slot as usize)?
                    .load(Ordering::Relaxed),
            });
        }
        None
    }

    /// Seal each empty stripe against a racing final publication. A failed
    /// phase commit may leave stripes sealed, which is safe for the fallback.
    pub(crate) fn seal_if_empty(&self) -> bool {
        match self
            .state
            .compare_exchange(UNUSED, CLOSED, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) | Err(CLOSED) => true,
            Err(_) => {
                let Some(table) = self.table.get() else {
                    return false;
                };
                if table
                    .stripes
                    .iter()
                    .any(|s| s.bits.load(Ordering::Acquire) & !SEALED != 0)
                {
                    return false;
                }
                for stripe in &table.stripes {
                    if stripe
                        .bits
                        .compare_exchange(0, SEALED, Ordering::AcqRel, Ordering::Acquire)
                        .is_err_and(|observed| observed != SEALED)
                    {
                        return false;
                    }
                }
                self.state.store(CLOSED, Ordering::Release);
                true
            }
        }
    }

    /// The normal terminal path has sealed every stripe. Activation rollback
    /// and shutdown can discard an unpublished epoch without tracing it.
    pub(crate) fn abandon(&self) {
        if self.state.swap(CLOSED, Ordering::AcqRel) == ACTIVE
            && let Some(table) = self.table.get()
        {
            for stripe in &table.stripes {
                stripe.bits.fetch_or(SEALED, Ordering::AcqRel);
            }
        }
    }
}

#[cfg(test)]
impl MatureDeletions {
    pub(crate) fn contains(&self, id: ObjectId) -> bool {
        let raw = id.to_i64().cast_unsigned();
        let group = raw / u64::from(WIDTH);
        self.table.get().is_some_and(|t| {
            t.stripes.iter().any(|s| {
                s.group.load(Ordering::Acquire) == group + 1
                    && s.bits.load(Ordering::Acquire) & (1 << (raw % u64::from(WIDTH))) != 0
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_bit_and_close_order_both_ways() {
        let id = ObjectId::from_i64(-2);
        let old = MatureDeletions::default();
        let (word, bit) = old.prepare(id, 8).expect("reserved group");
        assert!(old.seal_if_empty());
        let replacement = MatureDeletions::default();
        assert!(replacement.offer(id, 16));
        assert!(!MatureDeletions::publish(word, bit));
        assert!(!replacement.seal_if_empty());
        let target = replacement.take().expect("original exact id");
        assert_eq!(target.id, id);
        #[cfg(feature = "experimental-identity-index")]
        assert_eq!(target.address, 16);
        assert!(replacement.seal_if_empty());
        assert!(!replacement.offer(id, 16));
    }

    #[test]
    fn collisions_and_detach_keep_the_first_group_identity() {
        let epoch = MatureDeletions::default();
        let id = ObjectId::from_i64(1);
        let collision = ObjectId::from_i64(1 + i64::from(WIDTH) * i64::try_from(STRIPES).unwrap());
        for group in 0..STRIPES {
            let alias =
                ObjectId::from_i64(1 + i64::try_from(group).unwrap() * (collision.to_i64() - 1));
            assert!(epoch.offer(alias, 8));
        }
        let beyond =
            ObjectId::from_i64(1 + i64::try_from(STRIPES).unwrap() * (collision.to_i64() - 1));
        assert!(!epoch.offer(beyond, 16));
        let mut seen = Vec::new();
        while let Some(target) = epoch.take() {
            seen.push(target.id);
        }
        assert_eq!(seen.len(), STRIPES);
        assert!(seen.contains(&id) && seen.contains(&collision) && !seen.contains(&beyond));
        let (word, bit) = epoch.prepare(id, 8).expect("original group still bound");
        epoch.abandon();
        assert!(!MatureDeletions::publish(word, bit));
        assert!(epoch.take().is_none());
    }
}

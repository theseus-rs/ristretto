//! The first 64 cards (2048 Java slots) are inline. Larger indices use sparse words.
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub(crate) struct CardSet {
    first: u64,
    #[expect(
        clippy::box_collection,
        reason = "keep common inline-only sets to two words on native"
    )]
    overflow: Option<Box<BTreeMap<usize, u64>>>,
}

impl CardSet {
    pub(crate) fn insert(&mut self, card: usize) {
        let mask = 1 << (card % 64);
        if card < 64 {
            self.first |= mask;
        } else {
            *self
                .overflow
                .get_or_insert_with(Box::default)
                .entry(card / 64)
                .or_default() |= mask;
        }
    }

    pub(crate) fn remove(&mut self, card: usize) {
        let mask = 1 << (card % 64);
        if card < 64 {
            self.first &= !mask;
            return;
        }
        if let Some(words) = &mut self.overflow {
            let word = card / 64;
            if let Some(bits) = words.get_mut(&word) {
                *bits &= !mask;
                if *bits == 0 {
                    words.remove(&word);
                }
            }
            if words.is_empty() {
                self.overflow = None;
            }
        }
    }

    pub(crate) fn next_from(&self, card: usize) -> Option<usize> {
        let cutoff = u64::MAX << (card % 64);
        if card < 64 {
            let bits = self.first & cutoff;
            if bits != 0 {
                return Some(bits.trailing_zeros() as usize);
            }
        }
        let start = card / 64;
        self.overflow
            .as_ref()?
            .range(start..)
            .find_map(|(&word, &bits)| {
                let bits = if word == start { bits & cutoff } else { bits };
                (bits != 0).then(|| word * 64 + bits.trailing_zeros() as usize)
            })
    }

    pub(crate) fn pop_last(&mut self) -> Option<(usize, u64)> {
        if let Some(words) = &mut self.overflow {
            let last = words.pop_last();
            if words.is_empty() {
                self.overflow = None;
            }
            return last;
        }
        let bits = std::mem::take(&mut self.first);
        (bits != 0).then_some((0, bits))
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.first == 0 && self.overflow.is_none()
    }

    #[cfg(test)]
    pub(crate) fn contains(&self, card: usize) -> bool {
        self.next_from(card) == Some(card)
    }
}

impl Extend<usize> for CardSet {
    fn extend<T: IntoIterator<Item = usize>>(&mut self, cards: T) {
        for card in cards {
            self.insert(card);
        }
    }
}

impl FromIterator<usize> for CardSet {
    fn from_iter<T: IntoIterator<Item = usize>>(cards: T) -> Self {
        let mut set = Self::default();
        set.extend(cards);
        set
    }
}

#[cfg(test)]
mod tests {
    use super::CardSet;

    #[test]
    fn sparse_scan_skips_empty_words_and_respects_the_start_bit() {
        let mut set: CardSet = [1, 63, 64, 2000].into_iter().collect();
        assert_eq!(set.overflow.as_ref().map(|words| words.len()), Some(2));
        assert_eq!(set.next_from(2), Some(63));
        assert_eq!(set.next_from(64), Some(64));
        assert_eq!(set.next_from(65), Some(2000));
        set.remove(63);
        assert_eq!(set.next_from(2), Some(64));
        set.remove(64);
        assert_eq!(set.next_from(64), Some(2000));
        assert_eq!(set.next_from(2001), None);
        assert_eq!(set.pop_last(), Some((2000 / 64, 1 << (2000 % 64))));
        assert!(set.overflow.is_none());
        assert_eq!(set.pop_last(), Some((0, 2)));
        assert!(set.is_empty());
        set.extend([0, 63, 64, 127, 128, usize::MAX]);
        assert_eq!(set.next_from(127), Some(127));
        assert_eq!(set.next_from(129), Some(usize::MAX));
        assert_eq!(set.next_from(usize::MAX), Some(usize::MAX));
        for card in [64, 127, 128, usize::MAX] {
            set.remove(card);
        }
        assert!(set.overflow.is_none());
        assert_eq!(set.pop_last(), Some((0, 1 | (1 << 63))));
    }
}

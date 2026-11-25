//! Exact-key preconditions shared by ordinary registration and its image migration.
//!
//! An identity hit is not a publication-completion certificate. In particular,
//! replay must not overwrite an address occupied by a different allocation, or
//! treat a partially installed write set as a completed mark/traversal obligation.
use crate::ObjectId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Key {
    pub(crate) address: usize,
    pub(crate) id: ObjectId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Observed {
    pub(crate) object: Option<ObjectId>,
    pub(crate) identity: Option<usize>,
    pub(crate) young: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Conflict {
    NullKey,
    ForeignCollector,
    AlreadyPresent,
    Address,
    Identity,
    Young,
}

impl Key {
    /// Validate the observed keys. Concurrently writable slots must also be
    /// claimed conditionally before applying any other admitted effect.
    pub(crate) fn vacant(self, observed: Observed) -> Result<(), Conflict> {
        if self.address == 0 || self.id.to_i64() == 0 {
            return Err(Conflict::NullKey);
        }
        if observed.object == Some(self.id)
            && observed.identity == Some(self.address)
            && observed.young
        {
            return Err(Conflict::AlreadyPresent);
        }
        if observed.object.is_some() {
            return Err(Conflict::Address);
        }
        if observed.identity.is_some() {
            return Err(Conflict::Identity);
        }
        if observed.young {
            return Err(Conflict::Young);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_complete_vacant_write_set_accepts_a_nonnull_key() {
        let id = ObjectId::from_i64(1);
        let other = ObjectId::from_i64(2);
        for object in [None, Some(id), Some(other)] {
            for identity in [None, Some(8), Some(16)] {
                for young in [false, true] {
                    let observed = Observed {
                        object,
                        identity,
                        young,
                    };
                    assert_eq!(
                        Key { address: 8, id }.vacant(observed).is_ok(),
                        object.is_none() && identity.is_none() && !young
                    );
                    for key in [
                        Key { address: 0, id },
                        Key {
                            address: 8,
                            id: ObjectId::from_i64(0),
                        },
                    ] {
                        assert_eq!(key.vacant(observed), Err(Conflict::NullKey));
                    }
                }
            }
        }
    }
}

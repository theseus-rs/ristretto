//! Validation for an ordinary birth after reserving its address slot.
use crate::ObjectId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Conflict {
    NullKey,
    ForeignCollector,
    AlreadyPresent,
    Address,
    Identity,
}

pub(crate) fn validate(
    address: usize,
    id: ObjectId,
    previous_identity: Option<usize>,
) -> Result<(), Conflict> {
    if address == 0 || id.to_i64() == 0 {
        return Err(Conflict::NullKey);
    }
    if previous_identity.is_some() {
        return Err(Conflict::Identity);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserved_address_requires_a_nonnull_key_and_vacant_identity() {
        let id = ObjectId::from_i64(1);
        assert_eq!(validate(8, id, None), Ok(()));
        for occupied in [Some(8), Some(16)] {
            assert_eq!(validate(8, id, occupied), Err(Conflict::Identity));
        }
        for identity in [None, Some(8)] {
            assert_eq!(validate(0, id, identity), Err(Conflict::NullKey));
            assert_eq!(
                validate(8, ObjectId::from_i64(0), identity),
                Err(Conflict::NullKey)
            );
        }
    }
}

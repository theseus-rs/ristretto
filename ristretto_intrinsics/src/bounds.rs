use ristretto_types::Error::InternalError;
use ristretto_types::Result;
use std::ops::{Range, RangeFrom, RangeTo};

/// Returns the element at `index`, or an [`InternalError`] with caller context when out of bounds.
pub(crate) fn index<'a, T>(values: &'a [T], index: usize, context: &str) -> Result<&'a T> {
    values
        .get(index)
        .ok_or_else(|| InternalError(format!("{context}: index {index} out of bounds")))
}

/// Returns a mutable element at `index`, or an [`InternalError`] with caller context when out of
/// bounds.
pub(crate) fn index_mut<'a, T>(
    values: &'a mut [T],
    index: usize,
    context: &str,
) -> Result<&'a mut T> {
    values
        .get_mut(index)
        .ok_or_else(|| InternalError(format!("{context}: index {index} out of bounds")))
}

/// Returns the requested exclusive range, or an [`InternalError`] with caller context when out of
/// bounds.
pub(crate) fn range<'a, T>(values: &'a [T], range: Range<usize>, context: &str) -> Result<&'a [T]> {
    let Range { start, end } = range;
    values
        .get(start..end)
        .ok_or_else(|| InternalError(format!("{context}: range {start}..{end} out of bounds")))
}

/// Returns the requested mutable exclusive range, or an [`InternalError`] with caller context when
/// out of bounds.
pub(crate) fn range_mut<'a, T>(
    values: &'a mut [T],
    range: Range<usize>,
    context: &str,
) -> Result<&'a mut [T]> {
    let Range { start, end } = range;
    values
        .get_mut(start..end)
        .ok_or_else(|| InternalError(format!("{context}: range {start}..{end} out of bounds")))
}

/// Returns the slice up to `range.end`, or an [`InternalError`] with caller context when out of
/// bounds.
pub(crate) fn range_to<'a, T>(
    values: &'a [T],
    range: RangeTo<usize>,
    context: &str,
) -> Result<&'a [T]> {
    values
        .get(range)
        .ok_or_else(|| InternalError(format!("{context}: range {range:?} out of bounds")))
}

/// Returns the mutable slice up to `range.end`, or an [`InternalError`] with caller context when
/// out of bounds.
pub(crate) fn range_to_mut<'a, T>(
    values: &'a mut [T],
    range: RangeTo<usize>,
    context: &str,
) -> Result<&'a mut [T]> {
    values
        .get_mut(range)
        .ok_or_else(|| InternalError(format!("{context}: range {range:?} out of bounds")))
}

/// Returns the slice from `range.start` to the end, or an [`InternalError`] with caller context when
/// out of bounds.
pub(crate) fn range_from<'a, T>(
    values: &'a [T],
    range: RangeFrom<usize>,
    context: &str,
) -> Result<&'a [T]> {
    let RangeFrom { start } = range;
    values
        .get(start..)
        .ok_or_else(|| InternalError(format!("{context}: range {start}.. out of bounds")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    fn assert_error<T: Debug>(result: Result<T>, expected: &str) {
        let error = result.expect_err("expected bounds check to fail");
        assert_eq!(error.to_string(), format!("Internal error: {expected}"));
    }

    #[test]
    fn test_index() {
        let values = [10, 20, 30];

        assert_eq!(&20, index(&values, 1, "index read").expect("valid index"));
        assert_error(
            index(&values, 3, "index read"),
            "index read: index 3 out of bounds",
        );
    }

    #[test]
    fn test_index_mut() {
        let mut values = [10, 20, 30];

        *index_mut(&mut values, 1, "index write").expect("valid mutable index") = 25;
        assert_eq!([10, 25, 30], values);
        assert_error(
            index_mut(&mut values, 3, "index write"),
            "index write: index 3 out of bounds",
        );
    }

    #[test]
    fn test_range() {
        let values = [10, 20, 30, 40];

        assert_eq!(
            &[20, 30],
            range(&values, 1..3, "range read").expect("valid range")
        );
        assert_error(
            range(&values, 2..5, "range read"),
            "range read: range 2..5 out of bounds",
        );
    }

    #[test]
    fn test_range_mut() {
        let mut values = [10, 20, 30, 40];

        range_mut(&mut values, 1..3, "range write")
            .expect("valid mutable range")
            .copy_from_slice(&[25, 35]);
        assert_eq!([10, 25, 35, 40], values);
        assert_error(
            range_mut(&mut values, 2..5, "range write"),
            "range write: range 2..5 out of bounds",
        );
    }

    #[test]
    fn test_range_to() {
        let values = [10, 20, 30, 40];

        assert_eq!(
            &[10, 20],
            range_to(&values, ..2, "range_to read").expect("valid range_to")
        );
        assert_error(
            range_to(&values, ..5, "range_to read"),
            "range_to read: range ..5 out of bounds",
        );
    }

    #[test]
    fn test_range_to_mut() {
        let mut values = [10, 20, 30, 40];

        range_to_mut(&mut values, ..2, "range_to write")
            .expect("valid mutable range_to")
            .copy_from_slice(&[15, 25]);
        assert_eq!([15, 25, 30, 40], values);
        assert_error(
            range_to_mut(&mut values, ..5, "range_to write"),
            "range_to write: range ..5 out of bounds",
        );
    }

    #[test]
    fn test_range_from() {
        let values = [10, 20, 30, 40];

        assert_eq!(
            &[30, 40],
            range_from(&values, 2.., "range_from read").expect("valid range_from")
        );
        assert_error(
            range_from(&values, 5.., "range_from read"),
            "range_from read: range 5.. out of bounds",
        );
    }
}

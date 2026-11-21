//! Dense radix-eight reference arrays. Only the catalog owns their edges.
use super::Error;

pub(super) const FANOUT: usize = 8;
pub(super) const MAX_HEIGHT: usize = 11;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Component {
    Object,
    Class(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Layout {
    pub(super) length: u32,
    pub(super) height: u8,
    /// Includes the Java header, which doubles as the tree's top branch.
    pub(super) nodes: usize,
}
impl Layout {
    pub(super) fn new(length: i32) -> Result<Self, Error> {
        let length =
            u32::try_from(length).map_err(|_| Error::ArrayFault("NegativeArraySizeException"))?;
        let mut width = u64::from(length).div_ceil(FANOUT as u64);
        let mut nodes = 1_u64;
        let mut height = 1_u8;
        while width > 1 {
            nodes = nodes
                .checked_add(width)
                .ok_or(Error::Limit("reference array nodes"))?;
            width = width.div_ceil(FANOUT as u64);
            height = height
                .checked_add(1)
                .ok_or(Error::Limit("reference array height"))?;
        }
        if usize::from(height) > MAX_HEIGHT {
            return Err(Error::Limit("reference array height"));
        }
        Ok(Self {
            length,
            height,
            nodes: usize::try_from(nodes).map_err(|_| Error::Limit("reference array nodes"))?,
        })
    }
    pub(super) fn index(self, index: i32) -> Result<u32, Error> {
        let index = u32::try_from(index)
            .map_err(|_| Error::ArrayFault("ArrayIndexOutOfBoundsException"))?;
        if index >= self.length {
            return Err(Error::ArrayFault("ArrayIndexOutOfBoundsException"));
        }
        Ok(index)
    }
}

/// Number of logical elements under one edge at the given height.
pub(super) fn span(height: u8) -> Result<u32, Error> {
    if height == 0 || usize::from(height) > MAX_HEIGHT {
        return Err(Error::Boundary("reference array height"));
    }
    8_u32
        .checked_pow(u32::from(height - 1))
        .ok_or(Error::Limit("reference array span"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_dense_shape_and_checked_indices() -> Result<(), Box<dyn std::error::Error>> {
        for (length, height, nodes) in [
            (0, 1, 1),
            (1, 1, 1),
            (8, 1, 1),
            (9, 2, 3),
            (64, 2, 9),
            (65, 3, 12),
            (2048, 4, 293),
        ] {
            let layout = Layout::new(length)?;
            assert_eq!((layout.height, layout.nodes), (height, nodes));
            assert!(layout.index(-1).is_err());
            assert!(layout.index(length).is_err());
            if length > 0 {
                assert_eq!(layout.index(length - 1)?, u32::try_from(length - 1)?);
            }
        }
        assert!(Layout::new(-1).is_err());
        let maximum = Layout::new(i32::MAX)?;
        assert_eq!(maximum.height, 11);
        assert_eq!(span(maximum.height)?, 1 << 30);
        assert!(span(0).is_err());
        assert!(span(12).is_err());
        Ok(())
    }
}

//! Per-VM registry of memory-mapped file regions.
//!
//! Ristretto's `NativeMemory` is a virtualized allocator (a `BTreeMap` of `Vec<u8>` blocks),
//! not a real OS-level memory mapping. To expose a `MappedByteBuffer` that behaves correctly to
//! Java code, `sun/nio/ch/FileChannelImpl.map0` allocates a `NativeMemory` block, copies the
//! file's bytes into it, and registers the (address -> region) entry here. `force0` looks the
//! region up to write the bytes back to the file and `unmap0` removes the registration.

use ahash::AHashMap;
use parking_lot::Mutex;

/// The protection / access mode for a memory-mapped region.
///
/// Mirrors the constants used by `sun.nio.ch.FileChannelImpl`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MapMode {
    /// Read-only mapping (`MAP_RO = 0`).
    ReadOnly,
    /// Read/write shared mapping (`MAP_RW = 1`). Changes are flushed back to the file by
    /// `force`.
    ReadWrite,
    /// Private (copy-on-write) mapping (`MAP_PV = 2`). Changes are not flushed back to the
    /// file.
    Private,
}

impl MapMode {
    /// Decodes the integer mode passed by the JDK (`MAP_RO=0`, `MAP_RW=1`, `MAP_PV=2`).
    #[must_use]
    pub fn from_int(prot: i32) -> Option<Self> {
        match prot {
            0 => Some(Self::ReadOnly),
            1 => Some(Self::ReadWrite),
            2 => Some(Self::Private),
            _ => None,
        }
    }

    /// Returns true if changes to this region should be flushed back to the underlying file by
    /// `force0`.
    #[must_use]
    pub fn is_writable_back_to_file(self) -> bool {
        matches!(self, Self::ReadWrite)
    }
}

/// Metadata describing a single memory-mapped file region.
#[derive(Clone, Debug)]
pub struct MappedRegion {
    /// Underlying file descriptor (as managed by `vm.file_handles()`).
    pub fd: i64,
    /// File offset of the start of the mapping (always non-negative).
    pub position: u64,
    /// Size of the mapping in bytes.
    pub length: usize,
    /// Mode the region was mapped with.
    pub mode: MapMode,
}

/// Per-VM registry storing all currently-active memory-mapped file regions, keyed by their base
/// address in `NativeMemory`.
#[derive(Debug, Default)]
pub struct MappedRegions {
    regions: Mutex<AHashMap<i64, MappedRegion>>,
}

impl MappedRegions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a new mapped region. Any existing entry for `address` is overwritten (this
    /// shouldn't happen in practice because `NativeMemory` allocates monotonically-increasing
    /// addresses).
    pub fn insert(&self, address: i64, region: MappedRegion) {
        self.regions.lock().insert(address, region);
    }

    /// Returns a copy of the region metadata for `address`, if any.
    pub fn get(&self, address: i64) -> Option<MappedRegion> {
        self.regions.lock().get(&address).cloned()
    }

    /// Removes and returns the region metadata for `address`, if any.
    pub fn remove(&self, address: i64) -> Option<MappedRegion> {
        self.regions.lock().remove(&address)
    }

    /// Returns the region whose mapping covers `[address, address+length)` if any. The address
    /// passed by Java may point into the middle of a region (when the buffer was sliced).
    pub fn find_containing(&self, address: i64, length: usize) -> Option<(i64, MappedRegion)> {
        let regions = self.regions.lock();
        let length_i64 = i64::try_from(length).ok()?;
        for (&base, region) in regions.iter() {
            let region_len_i64 = i64::try_from(region.length).ok()?;
            if address >= base
                && address.saturating_add(length_i64) <= base.saturating_add(region_len_i64)
            {
                return Some((base, region.clone()));
            }
        }
        None
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.regions.lock().len()
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.regions.lock().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_mode_from_int() {
        assert_eq!(MapMode::from_int(0), Some(MapMode::ReadOnly));
        assert_eq!(MapMode::from_int(1), Some(MapMode::ReadWrite));
        assert_eq!(MapMode::from_int(2), Some(MapMode::Private));
        assert_eq!(MapMode::from_int(7), None);
    }

    #[test]
    fn test_writable_back() {
        assert!(!MapMode::ReadOnly.is_writable_back_to_file());
        assert!(MapMode::ReadWrite.is_writable_back_to_file());
        assert!(!MapMode::Private.is_writable_back_to_file());
    }

    #[test]
    fn test_insert_get_remove() {
        let regions = MappedRegions::new();
        regions.insert(
            0x1000,
            MappedRegion {
                fd: 5,
                position: 0,
                length: 64,
                mode: MapMode::ReadWrite,
            },
        );
        assert_eq!(regions.len(), 1);
        let r = regions.get(0x1000).expect("present");
        assert_eq!(r.fd, 5);
        let r2 = regions.remove(0x1000).expect("present");
        assert_eq!(r2.fd, 5);
        assert_eq!(regions.len(), 0);
    }

    #[test]
    fn test_find_containing() {
        let regions = MappedRegions::new();
        regions.insert(
            0x1000,
            MappedRegion {
                fd: 1,
                position: 0,
                length: 100,
                mode: MapMode::ReadWrite,
            },
        );
        let (base, _) = regions.find_containing(0x1010, 16).expect("found");
        assert_eq!(base, 0x1000);
        assert!(regions.find_containing(0x2000, 1).is_none());
        assert!(regions.find_containing(0x1050, 100).is_none());
    }
}

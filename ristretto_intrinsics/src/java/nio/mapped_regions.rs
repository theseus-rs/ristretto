//! Per-VM registry of memory-mapped file regions.
//!
//! Ristretto's `NativeMemory` is a virtualized allocator (a `BTreeMap` of `Vec<u8>` blocks),
//! not a real OS-level memory mapping. To expose a `MappedByteBuffer` that behaves correctly to
//! Java code, `sun/nio/ch/FileChannelImpl.map0` allocates a `NativeMemory` block, copies the
//! file's bytes into it, and registers the (address -> region) entry here. `force0` looks the
//! region up to write the bytes back to the file and `unmap0` removes the registration.

use ahash::AHashMap;
use ristretto_gc::sync::Mutex;

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
    /// Canonicalized path of the underlying file, if known. Used (on Windows) to refuse
    /// `DeleteFile0` for files with active mappings.
    pub path: Option<String>,
}

/// Per-VM registry storing all currently-active memory-mapped file regions, keyed by their base
/// address in `NativeMemory`.
#[derive(Debug, Default)]
pub struct MappedRegions {
    regions: Mutex<AHashMap<i64, MappedRegion>>,
    /// Reference-counted set of canonicalized file paths that currently have one or more
    /// active mappings. Used (on Windows) to refuse `DeleteFile0` for mapped files.
    mapped_paths: Mutex<AHashMap<String, usize>>,
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

    /// Returns all registered (address, region) pairs whose `fd` equals `fd` and whose mode
    /// is writable back to the underlying file.
    pub fn writable_entries_for_fd(&self, fd: i64) -> Vec<(i64, MappedRegion)> {
        self.regions
            .lock()
            .iter()
            .filter(|(_, r)| r.fd == fd && r.mode.is_writable_back_to_file())
            .map(|(addr, region)| (*addr, region.clone()))
            .collect()
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.regions.lock().is_empty()
    }

    /// Records that `path` (a canonicalized file path) currently has an active mapping. Each
    /// call must be balanced by a corresponding `release_path`.
    pub fn track_path(&self, path: String) {
        *self.mapped_paths.lock().entry(path).or_insert(0) += 1;
    }

    /// Releases a previously tracked path. Returns true if the path was tracked.
    pub fn release_path(&self, path: &str) -> bool {
        let mut paths = self.mapped_paths.lock();
        if let Some(count) = paths.get_mut(path) {
            *count -= 1;
            if *count == 0 {
                paths.remove(path);
            }
            true
        } else {
            false
        }
    }

    /// Returns true if `path` currently has any active mapping.
    #[must_use]
    pub fn is_path_mapped(&self, path: &str) -> bool {
        self.mapped_paths.lock().contains_key(path)
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
                path: None,
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
                path: None,
            },
        );
        let (base, _) = regions.find_containing(0x1010, 16).expect("found");
        assert_eq!(base, 0x1000);
        assert!(regions.find_containing(0x2000, 1).is_none());
        assert!(regions.find_containing(0x1050, 100).is_none());
    }

    #[test]
    fn test_track_release_path() {
        let regions = MappedRegions::new();
        assert!(!regions.is_path_mapped("/x"));
        regions.track_path("/x".to_string());
        regions.track_path("/x".to_string());
        assert!(regions.is_path_mapped("/x"));
        assert!(regions.release_path("/x"));
        assert!(regions.is_path_mapped("/x"));
        assert!(regions.release_path("/x"));
        assert!(!regions.is_path_mapped("/x"));
        assert!(!regions.release_path("/x"));
    }
}

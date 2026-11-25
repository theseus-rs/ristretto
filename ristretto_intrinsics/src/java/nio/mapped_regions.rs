//! Per-VM registry of memory-mapped file regions.
//!
//! Ristretto's `NativeMemory` is a virtualized allocator (a `BTreeMap` of `Vec<u8>` blocks),
//! not a real OS-level memory mapping. To expose a `MappedByteBuffer` that behaves correctly to
//! Java code, `sun/nio/ch/FileChannelImpl.map0` allocates a `NativeMemory` block, copies the
//! file's bytes into it, and registers the (address -> region) entry here. `force0` looks the
//! region up to write the bytes back to the file and `unmap0` removes the registration.

use crate::sun::nio::fs::managed_files;
use ahash::AHashMap;
use ristretto_gc::sync::Mutex;
use ristretto_types::handles::{FileHandle, HandleManager};
use ristretto_types::{Result, VM};
use std::io;

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
    pub channel_fd: Option<i64>,
    /// File offset of the start of the mapping (always non-negative).
    pub position: u64,
    /// Size of the mapping in bytes.
    pub length: usize,
    /// Mode the region was mapped with.
    pub mode: MapMode,
    /// Stable host file identity for this mapping, when the platform exposes one. Unix uses
    /// `(st_dev, st_ino)`. This prevents fd-number reuse from attaching an old mapping to a new
    /// file.
    pub file_key: Option<(u64, u64)>,
    /// Canonicalized path of the underlying file, if known. Used (on Windows) to refuse
    /// `DeleteFile0` for files with active mappings.
    pub path: Option<String>,
}

/// Per-VM registry storing all currently-active memory-mapped file regions, keyed by their base
/// address in `NativeMemory`.
#[derive(Debug)]
pub struct MappedRegions {
    regions: Mutex<AHashMap<i64, MappedRegion>>,
    // Mapping addresses are unique for the life of the VM, including after a channel closes.
    pub(crate) backing_files: HandleManager<i64, FileHandle>,
    /// Reference-counted set of canonicalized file paths that currently have one or more
    /// active mappings. Used (on Windows) to refuse `DeleteFile0` for mapped files.
    mapped_paths: Mutex<AHashMap<String, usize>>,
}

impl Default for MappedRegions {
    fn default() -> Self {
        Self::new()
    }
}

impl MappedRegions {
    #[must_use]
    pub fn new() -> Self {
        Self {
            regions: Mutex::default(),
            mapped_paths: Mutex::default(),
            backing_files: HandleManager::new(),
        }
    }

    /// Publish a mapping after retaining its backing file for writeback.
    pub(crate) async fn register<V: VM>(
        &self,
        vm: &V,
        address: i64,
        region: MappedRegion,
    ) -> Result<()> {
        if region.mode.is_writable_back_to_file()
            && let Some(fd) = region.channel_fd
        {
            let file = managed_files::clone_file_handle(vm.file_handles(), fd)
                .await
                .map_err(|error| {
                    vm.native_memory().free(address);
                    ristretto_types::JavaError::IoException(format!("map0: {error}"))
                })?;
            if let Err(error) = self.backing_files.insert(address, file).await {
                vm.native_memory().free(address);
                return Err(error);
            }
        }
        if let Some(path) = &region.path {
            self.track_path(path.clone());
        }
        self.regions.lock().insert(address, region);
        Ok(())
    }

    pub(crate) async fn unmap<V: VM>(&self, vm: &V, address: i64, length: i64) {
        let address = self
            .find_containing(address, usize::try_from(length).unwrap_or(0))
            .map_or(address, |(base, _)| base);
        let removed = self.regions.lock().remove(&address);
        if let Some(region) = removed {
            self.backing_files.remove(&address).await;
            if let Some(path) = region.path {
                self.release_path(&path);
            }
        }
        vm.native_memory().free(address);
    }

    /// Returns a copy of the region metadata for `address`, if any.
    pub fn get(&self, address: i64) -> Option<MappedRegion> {
        self.regions.lock().get(&address).cloned()
    }

    /// Detach a closed descriptor. A live buffer retains its backing file until unmap.
    pub fn detach_fd(&self, fd: i64) {
        for region in self.regions.lock().values_mut() {
            if region.channel_fd == Some(fd) {
                region.channel_fd = None;
            }
        }
    }

    /// Locate a JDK mapping request. The JDK may round its start down to an OS page
    /// boundary, including bytes of an earlier mapping. The end identifies the target.
    pub fn find_containing(&self, address: i64, length: usize) -> Option<(i64, MappedRegion)> {
        let end = address.checked_add(i64::try_from(length).ok()?)?;
        self.regions
            .lock()
            .iter()
            .filter(|(base, region)| {
                **base < end
                    && address
                        < base.saturating_add(i64::try_from(region.length).unwrap_or(i64::MAX))
            })
            .max_by_key(|(base, _)| **base)
            .map(|(&base, region)| (base, region.clone()))
    }

    /// Write the whole range to the file retained at mapping time, including after channel close.
    pub(crate) async fn write_back(
        &self,
        base: i64,
        mut bytes: &[u8],
        mut position: u64,
    ) -> io::Result<()> {
        while !bytes.is_empty() {
            let count = managed_files::write_at(&self.backing_files, base, bytes, position).await?;
            if count == 0 {
                return Err(io::ErrorKind::WriteZero.into());
            }
            bytes = bytes
                .get(count..)
                .ok_or_else(|| io::Error::other("Invalid mapped write count"))?;
            position += u64::try_from(count).map_err(io::Error::other)?;
        }
        Ok(())
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
            .filter(|(_, r)| r.channel_fd == Some(fd) && r.mode.is_writable_back_to_file())
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
        regions.regions.lock().insert(
            0x1000,
            MappedRegion {
                channel_fd: Some(5),
                position: 0,
                length: 64,
                mode: MapMode::ReadWrite,
                file_key: None,
                path: None,
            },
        );
        assert_eq!(regions.len(), 1);
        let r = regions.get(0x1000).expect("present");
        assert_eq!(r.channel_fd, Some(5));
        regions.detach_fd(5);
        let r2 = regions.regions.lock().remove(&0x1000).expect("present");
        assert_eq!(r2.channel_fd, None);
        assert_eq!(regions.len(), 0);
    }

    #[test]
    fn test_find_containing() {
        let regions = MappedRegions::new();
        regions.regions.lock().insert(
            0x1000,
            MappedRegion {
                channel_fd: Some(1),
                position: 0,
                length: 100,
                mode: MapMode::ReadWrite,
                file_key: None,
                path: None,
            },
        );
        let (base, _) = regions.find_containing(0x1010, 16).expect("found");
        assert_eq!(base, 0x1000);
        // Region is [0x1000, 0x1064); queries that touch the region (even partially)
        // succeed under overlap semantics, while non-overlapping queries return None.
        let (base, _) = regions.find_containing(0x1050, 100).expect("overlap");
        assert_eq!(base, 0x1000);
        let (base, _) = regions
            .find_containing(0x0F00, 0x200)
            .expect("overlap-left");
        assert_eq!(base, 0x1000);
        let adjacent = regions.get(0x1000).expect("first region");
        regions.regions.lock().insert(0x10100, adjacent);
        assert_eq!(
            regions
                .find_containing(0x1000, 0xf104)
                .map(|(base, _)| base),
            Some(0x10100)
        );
        assert!(regions.find_containing(0x2000, 1).is_none());
        assert!(regions.find_containing(0x0F00, 0x100).is_none());
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

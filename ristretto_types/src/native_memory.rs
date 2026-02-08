use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicI64, Ordering};

/// Per-VM native memory manager.
#[derive(Debug)]
pub struct NativeMemory {
    next_address: AtomicI64,
    memory: Mutex<HashMap<i64, Vec<u8>>>,
}

impl Default for NativeMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl NativeMemory {
    /// Creates a new `NativeMemory` instance.
    #[must_use]
    pub fn new() -> Self {
        NativeMemory {
            next_address: AtomicI64::new(0x1000_0000),
            memory: Mutex::new(HashMap::new()),
        }
    }

    fn with_memory<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut HashMap<i64, Vec<u8>>) -> R,
    {
        let mut guard = self.memory.lock().expect("native memory lock poisoned");
        f(&mut guard)
    }

    /// Allocates a block of memory and returns the base address.
    pub fn allocate(&self, size: usize) -> i64 {
        let address = self.next_address.fetch_add(
            i64::try_from(size + 4096).unwrap_or(4096),
            Ordering::Relaxed,
        );
        self.with_memory(|map| {
            map.insert(address, vec![0u8; size]);
        });
        address
    }

    /// Frees the memory at the given address.
    pub fn free(&self, address: i64) {
        self.with_memory(|map| {
            map.remove(&address);
        });
    }

    /// Reads `len` bytes starting at `address`.
    pub fn read_bytes(&self, address: i64, len: usize) -> Vec<u8> {
        self.with_memory(|map| {
            for (&base, buf) in map.iter() {
                let offset = address - base;
                if let Ok(offset) = usize::try_from(offset)
                    && offset + len <= buf.len()
                {
                    return buf[offset..offset + len].to_vec();
                }
            }
            vec![0u8; len]
        })
    }

    /// Writes `data` starting at `address`.
    pub fn write_bytes(&self, address: i64, data: &[u8]) {
        self.with_memory(|map| {
            for (&base, buf) in map.iter_mut() {
                let offset = address - base;
                if let Ok(offset) = usize::try_from(offset)
                    && offset + data.len() <= buf.len()
                {
                    buf[offset..offset + data.len()].copy_from_slice(data);
                    return;
                }
            }
        });
    }

    /// Reads a null-terminated C string starting at `address`.
    pub fn read_cstring(&self, address: i64) -> Vec<u8> {
        self.with_memory(|map| {
            for (&base, buf) in map.iter() {
                let offset = address - base;
                if let Ok(offset) = usize::try_from(offset)
                    && offset < buf.len()
                {
                    let end = buf[offset..]
                        .iter()
                        .position(|&b| b == 0)
                        .map_or(buf.len(), |p| offset + p);
                    return buf[offset..end].to_vec();
                }
            }
            Vec::new()
        })
    }

    /// Checks if the given address falls within any managed allocation.
    pub fn contains(&self, address: i64) -> bool {
        self.with_memory(|map| {
            for (&base, buf) in map.iter() {
                let offset = address - base;
                if let Ok(offset) = usize::try_from(offset)
                    && offset < buf.len()
                {
                    return true;
                }
            }
            false
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_and_read_write() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(16);
        mem.write_bytes(addr, &[1, 2, 3, 4]);
        let data = mem.read_bytes(addr, 4);
        assert_eq!(data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_free() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(8);
        mem.write_bytes(addr, &[42]);
        mem.free(addr);
        let data = mem.read_bytes(addr, 1);
        assert_eq!(data, vec![0]);
    }

    #[test]
    fn test_read_cstring() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(16);
        mem.write_bytes(addr, b"hello\0world");
        let s = mem.read_cstring(addr);
        assert_eq!(s, b"hello");
    }

    #[test]
    fn test_contains() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(8);
        assert!(mem.contains(addr));
        assert!(!mem.contains(addr + 100));
    }

    #[test]
    fn test_default() {
        let mem = NativeMemory::default();
        let addr = mem.allocate(4);
        assert!(mem.contains(addr));
    }

    #[test]
    fn test_isolation() {
        let mem1 = NativeMemory::new();
        let mem2 = NativeMemory::new();
        let addr1 = mem1.allocate(8);
        mem1.write_bytes(addr1, &[1, 2, 3]);
        let addr2 = mem2.allocate(8);
        mem2.write_bytes(addr2, &[4, 5, 6]);
        // Each instance is independent
        let data1 = mem1.read_bytes(addr1, 3);
        assert_eq!(data1, vec![1, 2, 3]);
        let data2 = mem2.read_bytes(addr2, 3);
        assert_eq!(data2, vec![4, 5, 6]);
    }
}

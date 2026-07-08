use std::collections::BTreeMap;
use std::sync::atomic::{AtomicI64, Ordering};

use ristretto_gc::sync::RwLock;

/// Per-VM native memory manager.
#[derive(Debug)]
pub struct NativeMemory {
    next_address: AtomicI64,
    memory: RwLock<BTreeMap<i64, RwLock<Vec<u8>>>>,
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
            memory: RwLock::new(BTreeMap::new()),
        }
    }

    /// Allocates a block of memory and returns the base address.
    pub fn allocate(&self, size: usize) -> i64 {
        let address = self.next_address.fetch_add(
            i64::try_from(size.saturating_add(4096)).unwrap_or(4096),
            Ordering::Relaxed,
        );
        let mut bytes = Vec::new();
        if bytes.try_reserve_exact(size).is_ok() {
            bytes.resize(size, 0);
        }
        self.memory.write().insert(address, RwLock::new(bytes));
        address
    }

    /// Frees the memory at the given address.
    pub fn free(&self, address: i64) {
        self.memory.write().remove(&address);
    }

    /// Reads `len` bytes starting at `address`.
    pub fn read_bytes(&self, address: i64, len: usize) -> Vec<u8> {
        self.try_read_bytes(address, len).unwrap_or_else(|| {
            let mut bytes = Vec::new();
            if bytes.try_reserve_exact(len).is_ok() {
                bytes.resize(len, 0);
            }
            bytes
        })
    }

    /// Tries to read `len` bytes starting at `address`.
    ///
    /// # Returns
    ///
    /// `None` if the address is not in any allocated region.
    pub fn try_read_bytes(&self, address: i64, len: usize) -> Option<Vec<u8>> {
        let guard = self.memory.read();
        let (&base, buf_lock) = guard.range(..=address).next_back()?;
        let offset = offset_from_base(address, base);
        let end = offset.checked_add(len)?;
        let buf = buf_lock.read();
        buf.get(offset..end).map(<[u8]>::to_vec)
    }

    /// Reads a value from native memory using a closure, avoiding `Vec` allocation.
    ///
    /// # Returns
    ///
    /// `None` if the address range is not within any allocation.
    pub fn read_with<F, R>(&self, address: i64, len: usize, f: F) -> Option<R>
    where
        F: FnOnce(&[u8]) -> R,
    {
        let guard = self.memory.read();
        let (&base, buf_lock) = guard.range(..=address).next_back()?;
        let offset = offset_from_base(address, base);
        let end = offset.checked_add(len)?;
        let buf = buf_lock.read();
        buf.get(offset..end).map(f)
    }

    /// Reads a single `i8` from `address`.
    pub fn read_i8(&self, address: i64) -> Option<i8> {
        self.read_with(address, 1, |bytes| {
            bytes.first().copied().map(u8::cast_signed)
        })
        .flatten()
    }

    /// Reads a single `i16` (native endian) from `address`.
    pub fn read_i16(&self, address: i64) -> Option<i16> {
        self.read_with(address, 2, |bytes| {
            <[u8; 2]>::try_from(bytes).map(i16::from_ne_bytes).ok()
        })
        .flatten()
    }

    /// Reads a single `i32` (native endian) from `address`.
    pub fn read_i32(&self, address: i64) -> Option<i32> {
        self.read_with(address, 4, |bytes| {
            <[u8; 4]>::try_from(bytes).map(i32::from_ne_bytes).ok()
        })
        .flatten()
    }

    /// Reads a single `i64` (native endian) from `address`.
    pub fn read_i64(&self, address: i64) -> Option<i64> {
        self.read_with(address, 8, |bytes| {
            <[u8; 8]>::try_from(bytes).map(i64::from_ne_bytes).ok()
        })
        .flatten()
    }

    /// Reads a single `f32` (native endian) from `address`.
    pub fn read_f32(&self, address: i64) -> Option<f32> {
        self.read_with(address, 4, |bytes| {
            <[u8; 4]>::try_from(bytes).map(f32::from_ne_bytes).ok()
        })
        .flatten()
    }

    /// Reads a single `f64` (native endian) from `address`.
    pub fn read_f64(&self, address: i64) -> Option<f64> {
        self.read_with(address, 8, |bytes| {
            <[u8; 8]>::try_from(bytes).map(f64::from_ne_bytes).ok()
        })
        .flatten()
    }

    /// Writes `data` starting at `address`.
    pub fn write_bytes(&self, address: i64, data: &[u8]) {
        let guard = self.memory.read();
        if let Some((&base, buf_lock)) = guard.range(..=address).next_back() {
            let offset = offset_from_base(address, base);
            let mut buf = buf_lock.write();
            if let Some(end) = offset.checked_add(data.len())
                && let Some(destination) = buf.get_mut(offset..end)
            {
                destination.copy_from_slice(data);
            }
        }
    }

    /// Writes a single `i8` to `address`.
    pub fn write_i8(&self, address: i64, value: i8) {
        self.write_bytes(address, &[value.cast_unsigned()]);
    }

    /// Writes a single `i16` (native endian) to `address`.
    pub fn write_i16(&self, address: i64, value: i16) {
        self.write_bytes(address, &value.to_ne_bytes());
    }

    /// Writes a single `i32` (native endian) to `address`.
    pub fn write_i32(&self, address: i64, value: i32) {
        self.write_bytes(address, &value.to_ne_bytes());
    }

    /// Writes a single `i64` (native endian) to `address`.
    pub fn write_i64(&self, address: i64, value: i64) {
        self.write_bytes(address, &value.to_ne_bytes());
    }

    /// Writes a single `f32` (native endian) to `address`.
    pub fn write_f32(&self, address: i64, value: f32) {
        self.write_bytes(address, &value.to_ne_bytes());
    }

    /// Writes a single `f64` (native endian) to `address`.
    pub fn write_f64(&self, address: i64, value: f64) {
        self.write_bytes(address, &value.to_ne_bytes());
    }

    /// Reads a null-terminated C string starting at `address`.
    pub fn read_cstring(&self, address: i64) -> Vec<u8> {
        let guard = self.memory.read();
        if let Some((&base, buf_lock)) = guard.range(..=address).next_back() {
            let offset = offset_from_base(address, base);
            let buf = buf_lock.read();
            if let Some(bytes) = buf.get(offset..) {
                let end = bytes
                    .iter()
                    .position(|&b| b == 0)
                    .map_or(bytes.len(), |position| position);
                return bytes.get(..end).map_or_else(Vec::new, <[u8]>::to_vec);
            }
        }
        Vec::new()
    }

    /// Checks if the given address falls within any managed allocation.
    pub fn contains(&self, address: i64) -> bool {
        let guard = self.memory.read();
        if let Some((&base, buf_lock)) = guard.range(..=address).next_back() {
            let offset = offset_from_base(address, base);
            let buf = buf_lock.read();
            return offset < buf.len();
        }
        false
    }
}

fn offset_from_base(address: i64, base: i64) -> usize {
    usize::try_from(address - base).unwrap_or(usize::MAX)
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
    fn test_read_write_at_offset() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(32);
        mem.write_bytes(addr + 8, &[10, 20, 30]);
        let data = mem.read_bytes(addr + 8, 3);
        assert_eq!(data, vec![10, 20, 30]);
        // Ensure beginning is still zeroed
        let zeros = mem.read_bytes(addr, 8);
        assert_eq!(zeros, vec![0; 8]);
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
    fn test_read_cstring_at_offset() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(32);
        mem.write_bytes(addr + 4, b"test\0");
        let s = mem.read_cstring(addr + 4);
        assert_eq!(s, b"test");
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

    #[test]
    fn test_read_write_i8() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(8);
        mem.write_i8(addr, -42);
        assert_eq!(mem.read_i8(addr), Some(-42));
    }

    #[test]
    fn test_read_write_i16() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(8);
        mem.write_i16(addr, -1234);
        assert_eq!(mem.read_i16(addr), Some(-1234));
    }

    #[test]
    fn test_read_write_i32() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(8);
        mem.write_i32(addr, 0x1234_5678);
        assert_eq!(mem.read_i32(addr), Some(0x1234_5678));
    }

    #[test]
    fn test_read_write_i64() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(16);
        mem.write_i64(addr, 0x1234_5678_9ABC_DEF0);
        assert_eq!(mem.read_i64(addr), Some(0x1234_5678_9ABC_DEF0));
    }

    #[test]
    fn test_read_write_f32() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(8);
        mem.write_f32(addr, std::f32::consts::PI);
        let val = mem.read_f32(addr).unwrap();
        assert!((val - std::f32::consts::PI).abs() < f32::EPSILON);
    }

    #[test]
    fn test_read_write_f64() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(16);
        mem.write_f64(addr, std::f64::consts::PI);
        let val = mem.read_f64(addr).unwrap();
        assert!((val - std::f64::consts::PI).abs() < f64::EPSILON);
    }

    #[test]
    fn test_read_with() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(16);
        mem.write_bytes(addr, &[1, 2, 3, 4]);
        let sum = mem.read_with(addr, 4, |bytes| {
            bytes.iter().map(|&b| u32::from(b)).sum::<u32>()
        });
        assert_eq!(sum, Some(10));
    }

    #[test]
    fn test_read_with_out_of_bounds() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(4);
        // Try to read more than allocated
        let result = mem.read_with(addr, 8, |_| 42);
        assert_eq!(result, None);
    }

    #[test]
    fn test_try_read_bytes_out_of_bounds() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(4);
        assert_eq!(mem.try_read_bytes(addr + 2, 4), None);
    }

    #[test]
    fn test_large_length_reads_do_not_panic() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(4);
        assert_eq!(mem.try_read_bytes(addr, usize::MAX), None);
        assert_eq!(mem.read_with(addr, usize::MAX, |_| 42), None);
        assert!(mem.read_bytes(0x9999_0000, usize::MAX).is_empty());
    }

    #[test]
    fn test_write_bytes_out_of_bounds_and_unallocated() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(4);
        mem.write_bytes(addr + 3, &[1, 2]);
        assert_eq!(mem.read_bytes(addr, 4), vec![0; 4]);
        mem.write_bytes(0x9999_0000, &[1, 2, 3]);
        assert_eq!(mem.read_bytes(0x9999_0000, 3), vec![0; 3]);
        mem.write_bytes(addr - 1, &[1]);
        assert_eq!(mem.read_bytes(addr, 4), vec![0; 4]);
    }

    #[test]
    fn test_read_cstring_invalid_addresses() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(4);
        assert_eq!(mem.read_cstring(addr + 4), Vec::<u8>::new());
        assert_eq!(mem.read_cstring(addr - 1), Vec::<u8>::new());
        assert_eq!(mem.read_cstring(0x9999_0000), Vec::<u8>::new());
    }

    #[test]
    fn test_contains_at_allocation_end_and_unallocated() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(4);
        assert!(!mem.contains(addr + 4));
        assert!(!mem.contains(0x9999_0000));
    }

    #[test]
    fn test_typed_read_unallocated() {
        let mem = NativeMemory::new();
        assert_eq!(mem.read_i8(0x9999_0000), None);
        assert_eq!(mem.read_i16(0x9999_0000), None);
        assert_eq!(mem.read_i32(0x9999_0000), None);
        assert_eq!(mem.read_i64(0x9999_0000), None);
        assert_eq!(mem.read_f32(0x9999_0000), None);
        assert_eq!(mem.read_f64(0x9999_0000), None);
    }

    #[test]
    fn test_typed_read_out_of_bounds() {
        let mem = NativeMemory::new();
        let addr = mem.allocate(1);
        assert_eq!(mem.read_i8(addr + 1), None);
        assert_eq!(mem.read_i16(addr), None);
        assert_eq!(mem.read_i32(addr), None);
        assert_eq!(mem.read_i64(addr), None);
        assert_eq!(mem.read_f32(addr), None);
        assert_eq!(mem.read_f64(addr), None);
    }

    #[test]
    fn test_multiple_allocations_lookup() {
        let mem = NativeMemory::new();
        let addr1 = mem.allocate(64);
        let addr2 = mem.allocate(64);
        let addr3 = mem.allocate(64);
        mem.write_i32(addr1, 111);
        mem.write_i32(addr2, 222);
        mem.write_i32(addr3, 333);
        assert_eq!(mem.read_i32(addr1), Some(111));
        assert_eq!(mem.read_i32(addr2), Some(222));
        assert_eq!(mem.read_i32(addr3), Some(333));
    }

    #[test]
    #[cfg(not(target_family = "wasm"))]
    fn test_concurrent_reads() {
        use std::sync::Arc;
        let mem = Arc::new(NativeMemory::new());
        let addr = mem.allocate(1024);
        mem.write_bytes(addr, &[42u8; 1024]);

        let handles: Vec<_> = (0..8)
            .map(|_| {
                let mem = Arc::clone(&mem);
                std::thread::spawn(move || {
                    for _ in 0..1000 {
                        let data = mem.read_bytes(addr, 1024);
                        assert!(data.iter().all(|&b| b == 42));
                        assert!(mem.contains(addr));
                        assert_eq!(mem.read_i32(addr), Some(i32::from_ne_bytes([42; 4])));
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().expect("thread panicked");
        }
    }
}

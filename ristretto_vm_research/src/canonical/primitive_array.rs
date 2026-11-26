//! Reference-free primitive-array backing for the experimental canonical tier.
//! Preparation allocates/initializes before bounded collector polling. Its byte
//! limit covers requested element backing; headers, allocator overhead and caller
//! storage are separate. Published length/type/backing never change.
use ristretto_gc::{GarbageCollector, Trace};
use std::fmt;
use std::sync::atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Kind {
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
}
impl Kind {
    pub(super) const fn width(self) -> usize {
        match self {
            Self::Boolean | Self::Byte => size_of::<AtomicU8>(),
            Self::Char | Self::Short => size_of::<AtomicU16>(),
            Self::Int | Self::Float => size_of::<AtomicU32>(),
            Self::Long | Self::Double => size_of::<AtomicU64>(),
        }
    }
}
/// Floating-point values retain their raw bits, including signed zero/NaN payloads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Element {
    Int(i32),
    Long(i64),
    Float(u32),
    Double(u64),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Error {
    NegativeLength,
    SizeOverflow,
    ByteLimit,
    Allocation,
    Index,
    Type,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::NegativeLength => "negative array length",
            Self::SizeOverflow => "array size overflow",
            Self::ByteLimit => "array backing byte limit",
            Self::Allocation => "array backing allocation",
            Self::Index => "array index outside bounds",
            Self::Type => "primitive array value type mismatch",
        })
    }
}
impl std::error::Error for Error {}

enum Storage {
    U8(Box<[AtomicU8]>),
    U16(Box<[AtomicU16]>),
    U32(Box<[AtomicU32]>),
    U64(Box<[AtomicU64]>),
}
pub(super) struct PrimitiveArray {
    kind: Kind,
    length: i32,
    bytes: usize,
    storage: Storage,
}
impl fmt::Debug for PrimitiveArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrimitiveArray")
            .field("kind", &self.kind)
            .field("length", &self.length)
            .field("backing_bytes", &self.bytes)
            .finish_non_exhaustive()
    }
}
fn zeroed<T>(length: usize, make: impl FnMut() -> T) -> Result<Box<[T]>, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| Error::Allocation)?;
    values.resize_with(length, make);
    Ok(values.into_boxed_slice())
}
impl PrimitiveArray {
    pub(super) fn required_bytes(kind: Kind, length: i32) -> Result<usize, Error> {
        usize::try_from(length)
            .map_err(|_| Error::NegativeLength)?
            .checked_mul(kind.width())
            .ok_or(Error::SizeOverflow)
    }
    pub(super) fn new(kind: Kind, length: i32, byte_limit: usize) -> Result<Self, Error> {
        let bytes = Self::required_bytes(kind, length)?;
        if bytes > byte_limit {
            return Err(Error::ByteLimit);
        }
        let count = usize::try_from(length).map_err(|_| Error::NegativeLength)?;
        let storage = match kind {
            Kind::Boolean | Kind::Byte => Storage::U8(zeroed(count, || AtomicU8::new(0))?),
            Kind::Char | Kind::Short => Storage::U16(zeroed(count, || AtomicU16::new(0))?),
            Kind::Int | Kind::Float => Storage::U32(zeroed(count, || AtomicU32::new(0))?),
            Kind::Long | Kind::Double => Storage::U64(zeroed(count, || AtomicU64::new(0))?),
        };
        Ok(Self {
            kind,
            length,
            bytes,
            storage,
        })
    }
    pub(super) fn kind(&self) -> Kind {
        self.kind
    }
    pub(super) fn len(&self) -> i32 {
        self.length
    }
    pub(super) fn backing_bytes(&self) -> usize {
        self.bytes
    }
    fn index(&self, index: i32) -> Result<usize, Error> {
        if index >= self.length {
            return Err(Error::Index);
        }
        usize::try_from(index).map_err(|_| Error::Index)
    }
    pub(super) fn load(&self, index: i32) -> Result<Element, Error> {
        let index = self.index(index)?;
        Ok(match (&self.storage, self.kind) {
            (Storage::U8(a), Kind::Boolean) => Element::Int(i32::from(
                a.get(index).ok_or(Error::Index)?.load(Ordering::Relaxed),
            )),
            (Storage::U8(a), Kind::Byte) => Element::Int(i32::from(i8::from_ne_bytes([a
                .get(index)
                .ok_or(Error::Index)?
                .load(Ordering::Relaxed)]))),
            (Storage::U16(a), Kind::Char) => Element::Int(i32::from(
                a.get(index).ok_or(Error::Index)?.load(Ordering::Relaxed),
            )),
            (Storage::U16(a), Kind::Short) => Element::Int(i32::from(i16::from_ne_bytes(
                a.get(index)
                    .ok_or(Error::Index)?
                    .load(Ordering::Relaxed)
                    .to_ne_bytes(),
            ))),
            (Storage::U32(a), Kind::Int) => Element::Int(
                a.get(index)
                    .ok_or(Error::Index)?
                    .load(Ordering::Relaxed)
                    .cast_signed(),
            ),
            (Storage::U32(a), Kind::Float) => {
                Element::Float(a.get(index).ok_or(Error::Index)?.load(Ordering::Relaxed))
            }
            (Storage::U64(a), Kind::Long) => Element::Long(
                a.get(index)
                    .ok_or(Error::Index)?
                    .load(Ordering::Relaxed)
                    .cast_signed(),
            ),
            (Storage::U64(a), Kind::Double) => {
                Element::Double(a.get(index).ok_or(Error::Index)?.load(Ordering::Relaxed))
            }
            _ => return Err(Error::Type),
        })
    }
    pub(super) fn store(&self, index: i32, value: Element) -> Result<(), Error> {
        let index = self.index(index)?;
        match (&self.storage, self.kind, value) {
            (Storage::U8(a), Kind::Boolean, Element::Int(v)) => {
                a.get(index)
                    .ok_or(Error::Index)?
                    .store(u8::from(v & 1 != 0), Ordering::Relaxed);
            }
            (Storage::U8(a), Kind::Byte, Element::Int(v)) => {
                a.get(index)
                    .ok_or(Error::Index)?
                    .store(v.to_le_bytes()[0], Ordering::Relaxed);
            }
            (Storage::U16(a), Kind::Char | Kind::Short, Element::Int(v)) => {
                let bytes = v.to_le_bytes();
                a.get(index)
                    .ok_or(Error::Index)?
                    .store(u16::from_le_bytes([bytes[0], bytes[1]]), Ordering::Relaxed);
            }
            (Storage::U32(a), Kind::Int, Element::Int(v)) => {
                a.get(index)
                    .ok_or(Error::Index)?
                    .store(v.cast_unsigned(), Ordering::Relaxed);
            }
            (Storage::U32(a), Kind::Float, Element::Float(v)) => {
                a.get(index)
                    .ok_or(Error::Index)?
                    .store(v, Ordering::Relaxed);
            }
            (Storage::U64(a), Kind::Long, Element::Long(v)) => {
                a.get(index)
                    .ok_or(Error::Index)?
                    .store(v.cast_unsigned(), Ordering::Relaxed);
            }
            (Storage::U64(a), Kind::Double, Element::Double(v)) => {
                a.get(index)
                    .ok_or(Error::Index)?
                    .store(v, Ordering::Relaxed);
            }
            _ => return Err(Error::Type),
        }
        Ok(())
    }
}
// SAFETY: only fixed-size atomics and exclusively owned atomic slices are stored.
// No managed references, user callbacks, resize or raw mutable pointer escapes.
#[expect(unsafe_code, reason = "audited reference-free primitive array backing")]
unsafe impl Trace for PrimitiveArray {
    fn canonical_payload() -> Option<ristretto_gc::CanonicalPayload<Self>> {
        // SAFETY: every field is primitive storage. Managed edges live in the catalog.
        Some(unsafe { ristretto_gc::CanonicalPayload::new() })
    }

    fn may_contain_gc_references() -> bool {
        false
    }
    fn needs_write_barrier(&self) -> bool {
        false
    }
    fn external_size(&self) -> usize {
        self.bytes
    }
    fn trace(&self, _: &GarbageCollector) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_backing_preserves_jvm_element_semantics() -> Result<(), Box<dyn std::error::Error>>
    {
        for (kind, written, read) in [
            (Kind::Boolean, Element::Int(-2), Element::Int(0)),
            (Kind::Byte, Element::Int(255), Element::Int(-1)),
            (Kind::Char, Element::Int(-1), Element::Int(65535)),
            (Kind::Short, Element::Int(0x8001), Element::Int(-32767)),
            (Kind::Int, Element::Int(i32::MIN), Element::Int(i32::MIN)),
            (Kind::Long, Element::Long(i64::MIN), Element::Long(i64::MIN)),
            (
                Kind::Float,
                Element::Float(0x7fc0_0042),
                Element::Float(0x7fc0_0042),
            ),
            (
                Kind::Double,
                Element::Double(1 << 63),
                Element::Double(1 << 63),
            ),
        ] {
            let array = PrimitiveArray::new(kind, 2, 2 * kind.width())?;
            assert_eq!(array.len(), 2);
            assert_eq!(array.kind(), kind);
            assert_eq!(array.backing_bytes(), 2 * kind.width());
            array.store(1, written)?;
            assert_eq!(read, array.load(1)?);
            assert_eq!(Err(Error::Index), array.store(-1, written));
            assert_eq!(Err(Error::Index), array.load(2));
        }
        assert!(matches!(
            PrimitiveArray::new(Kind::Double, 2, 15),
            Err(Error::ByteLimit)
        ));
        assert!(matches!(
            PrimitiveArray::new(Kind::Int, -1, 64),
            Err(Error::NegativeLength)
        ));
        let bytes = PrimitiveArray::new(Kind::Byte, 1, 1)?;
        assert_eq!(Err(Error::Type), bytes.store(0, Element::Long(255)));
        assert_eq!(Element::Int(0), bytes.load(0)?);
        Ok(())
    }
}

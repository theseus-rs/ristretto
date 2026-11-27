use crate::Result;
use ristretto_classfile::{JAVA_8, JAVA_17, Version};
use ristretto_classloader::{Class, Object, Reference, Value};
use ristretto_gc::{GarbageCollector, HeapBuffer};
use std::sync::Arc;
use zerocopy::IntoBytes;

type Utf8Constructor = fn(Arc<Class>, &Arc<GarbageCollector>, &str) -> Result<Value>;
type Utf16Constructor = fn(Arc<Class>, &Arc<GarbageCollector>, Vec<u16>) -> Result<Value>;

/// String constructors selected once for a VM's immutable runtime version.
///
/// Both VM and intrinsic conversions share these constructors. The synchronous
/// constructors use function pointers; asynchronous constructors use a cached
/// layout. Neither reads the VM version for each string. Content-dependent
/// Latin-1/UTF-16 selection remains per string.
#[derive(Clone, Debug)]
pub struct StringFactory {
    utf8: Utf8Constructor,
    utf16: Utf16Constructor,
    layout: Layout,
}

#[derive(Clone, Copy, Debug)]
enum Layout {
    Legacy,
    Compact { hash_is_zero: bool },
}

impl StringFactory {
    /// Selects the constructors for the configured runtime during VM creation.
    #[must_use]
    pub fn new(version: &Version) -> Self {
        if version <= &JAVA_8 {
            Self {
                utf8: legacy_utf8,
                utf16: legacy_utf16,
                layout: Layout::Legacy,
            }
        } else if version >= &JAVA_17 {
            Self {
                utf8: compact_utf8::<true>,
                utf16: compact_utf16::<true>,
                layout: Layout::Compact { hash_is_zero: true },
            }
        } else {
            Self {
                utf8: compact_utf8::<false>,
                utf16: compact_utf16::<false>,
                layout: Layout::Compact {
                    hash_is_zero: false,
                },
            }
        }
    }

    /// Creates a string from Rust UTF-8 using the selected layout.
    ///
    /// # Errors
    /// Returns an error if the string class cannot be instantiated or its fields cannot be set.
    pub fn create_utf8(
        &self,
        class: Arc<Class>,
        collector: &Arc<GarbageCollector>,
        string: &str,
    ) -> Result<Value> {
        (self.utf8)(class, collector, string)
    }

    /// Creates a string from UTF-16 code units, preserving lone surrogates.
    ///
    /// # Errors
    /// Returns an error if the string class cannot be instantiated or its fields cannot be set.
    pub fn create_utf16(
        &self,
        class: Arc<Class>,
        collector: &Arc<GarbageCollector>,
        units: Vec<u16>,
    ) -> Result<Value> {
        (self.utf16)(class, collector, units)
    }

    /// Creates a string without inline full-cycle recovery or a staging payload.
    /// Encoding selection and payload initialization yield between 64 KiB batches.
    ///
    /// # Errors
    /// Returns an allocation, class-layout, or field-access error.
    pub async fn create_utf8_async(
        &self,
        class: Arc<Class>,
        collector: &Arc<GarbageCollector>,
        string: &str,
    ) -> Result<Value> {
        check_guards()?;
        let encoding = utf8_encoding(string).await;
        let (coder, reference) = match self.layout {
            Layout::Legacy => {
                let units = string.encode_utf16();
                (
                    None,
                    Reference::CharArray(payload(collector, encoding.units, units).await?),
                )
            }
            Layout::Compact { .. } if encoding.characters == string.len() => (
                Some(0),
                Reference::ByteArray(
                    payload(collector, string.len(), string.bytes().map(u8::cast_signed)).await?,
                ),
            ),
            Layout::Compact { .. } if encoding.latin1 => (
                Some(0),
                Reference::ByteArray(
                    payload(
                        collector,
                        encoding.characters,
                        string.chars().map(|c| c as i8),
                    )
                    .await?,
                ),
            ),
            Layout::Compact { .. } => {
                let units = string.encode_utf16();
                let length = encoding
                    .units
                    .checked_mul(2)
                    .ok_or(ristretto_gc::Error::OutOfMemory)?;
                let bytes = units.flat_map(|unit| unit.to_ne_bytes().map(u8::cast_signed));
                (
                    Some(1),
                    Reference::ByteArray(payload(collector, length, bytes).await?),
                )
            }
        };
        self.publish_async(class, collector, coder, reference).await
    }

    /// Creates a string from UTF-16, retaining lone surrogates across suspension.
    /// Layout selection and initialization yield between 64 KiB input/output
    /// batches. Cancellation releases any partially constructed payload.
    ///
    /// # Errors
    /// Returns an allocation, class-layout, or field-access error.
    pub async fn create_utf16_async(
        &self,
        class: Arc<Class>,
        collector: &Arc<GarbageCollector>,
        units: Vec<u16>,
    ) -> Result<Value> {
        check_guards()?;
        let latin1 = matches!(self.layout, Layout::Compact { .. }) && utf16_is_latin1(&units).await;
        let (coder, reference) = match self.layout {
            Layout::Legacy => (
                None,
                Reference::CharArray(payload(collector, units.len(), units.into_iter()).await?),
            ),
            Layout::Compact { .. } if latin1 => {
                #[expect(
                    clippy::cast_possible_truncation,
                    reason = "Latin-1 fits in a Java byte"
                )]
                let bytes = units.into_iter().map(|unit| unit as i8);
                (
                    Some(0),
                    Reference::ByteArray(payload(collector, bytes.len(), bytes).await?),
                )
            }
            Layout::Compact { .. } => {
                let length = units
                    .len()
                    .checked_mul(2)
                    .ok_or(ristretto_gc::Error::OutOfMemory)?;
                let bytes = units
                    .into_iter()
                    .flat_map(|unit| unit.to_ne_bytes().map(u8::cast_signed));
                (
                    Some(1),
                    Reference::ByteArray(payload(collector, length, bytes).await?),
                )
            }
        };
        self.publish_async(class, collector, coder, reference).await
    }

    async fn publish_async(
        &self,
        class: Arc<Class>,
        collector: &Arc<GarbageCollector>,
        coder: Option<i32>,
        reference: Reference,
    ) -> Result<Value> {
        // Keep the payload rooted while field allocation or final publication waits.
        let array = Value::try_new_object_async(collector, reference).await?;
        let mut object = Object::new_in_async(collector, class).await?;
        if let Some(coder) = coder {
            object.set_value("coder", Value::Int(coder))?;
        }
        if matches!(self.layout, Layout::Compact { hash_is_zero: true }) {
            object.set_value("hashIsZero", Value::Int(0))?;
        }
        object.set_value("value", array)?;
        object.set_value("hash", Value::Int(0))?;
        Ok(Value::try_new_object_async(collector, Reference::Object(object)).await?)
    }
}

fn check_guards() -> Result<()> {
    if ristretto_gc::cell::has_guards() {
        return Err(ristretto_gc::Error::CollectionPhaseError(
            "asynchronous string construction requires released managed guards".into(),
        )
        .into());
    }
    Ok(())
}

struct Encoding {
    characters: usize,
    units: usize,
    latin1: bool,
}

// Valid UTF-8 contributes one UTF-16 unit for each leading byte, plus one for
// four-byte sequences. Counting bytes allows chunks to split a scalar without
// losing it or allocating an intermediate UTF-16 representation.
async fn utf8_encoding(string: &str) -> Encoding {
    let mut encoding = Encoding {
        characters: 0,
        units: 0,
        latin1: true,
    };
    let mut chunks = string.as_bytes().chunks(64 * 1024).peekable();
    while let Some(chunk) = chunks.next() {
        if chunk.is_ascii() {
            encoding.characters += chunk.len();
            encoding.units += chunk.len();
        } else {
            for &byte in chunk {
                let leading = usize::from(byte & 0xC0 != 0x80);
                encoding.characters += leading;
                encoding.units += leading + usize::from(byte >= 0xF0);
                encoding.latin1 &= byte < 0xC4;
            }
        }
        if chunks.peek().is_some() {
            yield_once().await;
        }
    }
    encoding
}

async fn utf16_is_latin1(units: &[u16]) -> bool {
    let mut chunks = units.chunks(32 * 1024).peekable();
    while let Some(chunk) = chunks.next() {
        if chunk.iter().any(|&unit| unit > 0xFF) {
            return false;
        }
        if chunks.peek().is_some() {
            yield_once().await;
        }
    }
    true
}

async fn yield_once() {
    let mut yielded = false;
    std::future::poll_fn(|cx| {
        if std::mem::replace(&mut yielded, true) {
            std::task::Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        }
    })
    .await;
}

async fn payload<T>(
    collector: &GarbageCollector,
    length: usize,
    mut values: impl Iterator<Item = T>,
) -> Result<HeapBuffer<T>> {
    Ok(HeapBuffer::try_with_fn_in_async(collector, length, |_| {
        values.next().ok_or(ristretto_gc::Error::OutOfMemory)
    })
    .await?)
}

fn legacy_utf8(
    class: Arc<Class>,
    collector: &Arc<GarbageCollector>,
    string: &str,
) -> Result<Value> {
    legacy_utf16(class, collector, string.encode_utf16().collect())
}

fn legacy_utf16(
    class: Arc<Class>,
    collector: &Arc<GarbageCollector>,
    units: Vec<u16>,
) -> Result<Value> {
    let mut object = Object::new(class)?;
    let array = Value::try_new_object(collector, Reference::CharArray(units.into()))?;
    object.set_value("value", array)?;
    object.set_value("hash", Value::Int(0))?;
    Ok(Value::try_from_object(collector, object)?)
}

fn compact_utf8<const HASH_IS_ZERO: bool>(
    class: Arc<Class>,
    collector: &Arc<GarbageCollector>,
    string: &str,
) -> Result<Value> {
    let (coder, bytes) = if string.is_ascii() {
        let bytes: &[i8] = zerocopy::transmute_ref!(string.as_bytes());
        (0, bytes.to_vec())
    } else if string.as_bytes().iter().all(|&byte| byte < 0xC4) {
        // Latin-1's non-ASCII characters have UTF-8 lead bytes C2/C3.
        (0, string.chars().map(|c| c as i8).collect())
    } else {
        // UTF-8's byte length covers non-ASCII UTF-16 text without repeatedly
        // growing the buffer. Mixed ASCII text may grow it.
        let mut bytes = Vec::with_capacity(string.len());
        for unit in string.encode_utf16() {
            bytes.extend_from_slice(&unit.to_ne_bytes().map(u8::cast_signed));
        }
        (1, bytes)
    };
    compact::<HASH_IS_ZERO>(class, collector, coder, bytes)
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "shared constructor signature lets Java 8 reuse the UTF-16 allocation"
)]
fn compact_utf16<const HASH_IS_ZERO: bool>(
    class: Arc<Class>,
    collector: &Arc<GarbageCollector>,
    units: Vec<u16>,
) -> Result<Value> {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Latin-1 fits in a Java byte"
    )]
    let (coder, bytes) = if units.iter().all(|&unit| unit <= 0xFF) {
        (0, units.iter().map(|&unit| unit as i8).collect())
    } else {
        let bytes: &[i8] = zerocopy::transmute_ref!(units.as_bytes());
        (1, bytes.to_vec())
    };
    compact::<HASH_IS_ZERO>(class, collector, coder, bytes)
}

fn compact<const HASH_IS_ZERO: bool>(
    class: Arc<Class>,
    collector: &Arc<GarbageCollector>,
    coder: i32,
    bytes: Vec<i8>,
) -> Result<Value> {
    let mut object = Object::new(class)?;
    // This condition is a compile-time constant in each selected constructor.
    if HASH_IS_ZERO {
        object.set_value("hashIsZero", Value::Int(0))?;
    }
    object.set_value("coder", Value::Int(coder))?;
    let array = Value::try_new_object(collector, Reference::from(bytes))?;
    object.set_value("value", array)?;
    object.set_value("hash", Value::Int(0))?;
    Ok(Value::try_from_object(collector, object)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Error, JavaError, test_utils};
    use ristretto_classfile::{JAVA_9, JAVA_11, JAVA_16, JAVA_25};
    use std::future::Future;
    use std::task::{Context, Poll, Waker};

    fn string_class(version: Version) -> Result<Arc<Class>> {
        let fields: &[(&str, &str)] = if version <= JAVA_8 {
            &[("value", "[C"), ("hash", "I")]
        } else if version >= JAVA_17 {
            &[
                ("value", "[B"),
                ("hash", "I"),
                ("coder", "B"),
                ("hashIsZero", "Z"),
            ]
        } else {
            &[("value", "[B"), ("hash", "I"), ("coder", "B")]
        };
        test_utils::class("java/lang/String", fields)
    }

    fn collector(compact: bool, limit: usize) -> Arc<GarbageCollector> {
        GarbageCollector::with_config(
            ristretto_gc::Configuration::builder()
                .compact_heap(compact)
                .diagnostics(true)
                .region_layout(128 * 1024, 4096)
                .max_heap_bytes(limit)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        )
    }

    fn assert_released(gc: &GarbageCollector) -> Result<()> {
        gc.collect_full()?;
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(gc.diagnostics().allocation_waiters, 0);
        if let Some(heap) = gc.region_heap() {
            let stats = heap.statistics();
            assert_eq!(stats.occupied_bytes, 0);
            assert_eq!(stats.bytes_staged, 0);
        }
        Ok(())
    }

    #[tokio::test]
    async fn encoding_counts_across_chunk_boundaries() {
        for offset in 0..4 {
            let input = "a".repeat(64 * 1024 - offset) + "\u{1f600}\u{e9}\u{100}\0";
            let encoding = utf8_encoding(&input).await;
            assert_eq!(encoding.characters, input.chars().count());
            assert_eq!(encoding.units, input.encode_utf16().count());
            assert!(!encoding.latin1);
        }
        let input = "\u{e9}".repeat(64 * 1024) + "\0";
        let encoding = utf8_encoding(&input).await;
        assert!(encoding.latin1);
        assert_eq!(encoding.characters, input.chars().count());
        assert_eq!(encoding.units, input.encode_utf16().count());
    }

    #[tokio::test]
    async fn strings_recover_while_other_tasks_progress() -> Result<()> {
        for compact in [false, true] {
            for version in [JAVA_8, JAVA_9, JAVA_11, JAVA_16, JAVA_17, JAVA_25] {
                recover_strings(compact, version).await?;
            }
        }
        Ok(())
    }

    async fn recover_strings(compact: bool, version: Version) -> Result<()> {
        use std::cell::Cell;
        let gc = collector(compact, 512 * 1024);
        let factory = StringFactory::new(&version);
        let class = string_class(version)?;
        let input = "a\u{e9}\u{1f600}".repeat(16 * 1024);
        let done = Cell::new(false);
        let progress = Cell::new(0);
        let allocations = async {
            let result: Result<()> = async {
                for iteration in 0..10 {
                    let value = if iteration % 2 == 0 {
                        factory
                            .create_utf8_async(class.clone(), &gc, &input)
                            .await?
                    } else {
                        factory
                            .create_utf16_async(class.clone(), &gc, input.encode_utf16().collect())
                            .await?
                    };
                    assert_eq!(value.as_string()?, input);
                }
                Ok(())
            }
            .await;
            done.set(true);
            result
        };
        let companion = async {
            while !done.get() {
                progress.set(progress.get() + 1);
                tokio::task::yield_now().await;
            }
        };
        let (result, ()) = tokio::join!(allocations, companion);
        result?;
        assert!(progress.get() >= 10);
        let diagnostics = gc.diagnostics();
        assert!(diagnostics.async_allocation_recovery.count > 0);
        assert_eq!(diagnostics.allocation_recovery.count, 0);
        assert_eq!(diagnostics.full_collection_calls, 0);
        assert_released(&gc)?;
        Ok(())
    }

    #[tokio::test]
    async fn utf16_pressure_preserves_live_source() -> Result<()> {
        for compact in [false, true] {
            for version in [JAVA_8, JAVA_11, JAVA_17] {
                let gc = collector(compact, 256 * 1024);
                let bytes = HeapBuffer::try_with_fn_in(&gc, 100 * 1024, |_| Ok(7_i8))?;
                let source = Value::try_new_object(&gc, Reference::ByteArray(bytes))?;
                let factory = StringFactory::new(&version);
                let result = factory
                    .create_utf16_async(string_class(version)?, &gc, vec![0xD800; 100 * 1024])
                    .await;
                assert!(matches!(
                    result,
                    Err(Error::JavaError(JavaError::OutOfMemoryError(_)))
                ));
                assert!(source.as_byte_vec_ref()?.iter().all(|&byte| byte == 7));
                assert!(gc.diagnostics().async_allocation_recovery.count > 0);
                assert_eq!(gc.diagnostics().allocation_recovery.count, 0);
                assert_eq!(gc.diagnostics().allocation_waiters, 0);
                drop(source);
                assert_released(&gc)?;
            }
        }
        Ok(())
    }

    #[test]
    fn a_managed_guard_is_rejected_before_scanning_can_suspend() -> Result<()> {
        let gc = collector(false, 1024 * 1024);
        let factory = StringFactory::new(&JAVA_17);
        let class = string_class(JAVA_17)?;
        let source = Value::try_new_object(&gc, Reference::from(vec![7_i8]))?;
        let guard = source.as_byte_vec_ref()?;
        let input = "a".repeat(128 * 1024);
        let mut cx = Context::from_waker(Waker::noop());
        let mut utf8 = std::pin::pin!(factory.create_utf8_async(class.clone(), &gc, &input));
        assert!(matches!(
            utf8.as_mut().poll(&mut cx),
            Poll::Ready(Err(Error::GcError(
                ristretto_gc::Error::CollectionPhaseError(_)
            )))
        ));
        let mut utf16 =
            std::pin::pin!(factory.create_utf16_async(class, &gc, vec![0_u16; 128 * 1024]));
        assert!(matches!(
            utf16.as_mut().poll(&mut cx),
            Poll::Ready(Err(Error::GcError(
                ristretto_gc::Error::CollectionPhaseError(_)
            )))
        ));
        assert_eq!(&*guard, &[7]);
        Ok(())
    }

    #[test]
    fn cancellation_releases_scanning_and_initialization_state() -> Result<()> {
        let input = "a".repeat(128 * 1024);
        for compact in [false, true] {
            for version in [JAVA_8, JAVA_11, JAVA_17] {
                for utf16 in [false, true] {
                    let gc = collector(compact, 1024 * 1024);
                    let factory = StringFactory::new(&version);
                    let class = string_class(version)?;
                    let mut completed = false;
                    // Cancel at every suspension up to and including publication.
                    for polls in 0..16 {
                        let mut future = Box::pin(async {
                            if utf16 {
                                factory
                                    .create_utf16_async(
                                        class.clone(),
                                        &gc,
                                        input.encode_utf16().collect(),
                                    )
                                    .await
                            } else {
                                factory.create_utf8_async(class.clone(), &gc, &input).await
                            }
                        });
                        let mut cx = Context::from_waker(Waker::noop());
                        for _ in 0..polls {
                            if let Poll::Ready(value) = future.as_mut().poll(&mut cx) {
                                assert_eq!(value?.as_string()?, input);
                                completed = true;
                                break;
                            }
                        }
                        drop(future);
                        assert_released(&gc)?;
                        if completed {
                            break;
                        }
                    }
                    assert!(completed, "fixture did not reach publication");
                }
            }
        }
        Ok(())
    }
}

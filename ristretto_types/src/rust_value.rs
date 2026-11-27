use ristretto_classloader::{Reference, Value};
use ristretto_gc::GarbageCollector;
use std::fmt::Debug;
use std::future::{Future, Ready};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

/// Object-safe argument conversion. Scalar and legacy custom implementations
/// complete without allocating a boxed future; built-in vectors may suspend.
#[must_use = "conversion futures must be awaited or polled"]
pub enum ValueConversion<'a> {
    Ready(Ready<crate::Result<Value>>),
    #[cfg(not(target_family = "wasm"))]
    Async(Pin<Box<dyn Future<Output = crate::Result<Value>> + Send + 'a>>),
    #[cfg(target_family = "wasm")]
    Async(Pin<Box<dyn Future<Output = crate::Result<Value>> + 'a>>),
}

impl Debug for ValueConversion<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ready(future) => formatter.debug_tuple("Ready").field(future).finish(),
            Self::Async(_) => formatter.write_str("Async(..)"),
        }
    }
}

impl Future for ValueConversion<'_> {
    type Output = crate::Result<Value>;
    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        match self.get_mut() {
            Self::Ready(future) => Pin::new(future).poll(context),
            Self::Async(future) => future.as_mut().poll(context),
        }
    }
}

#[expect(
    clippy::indexing_slicing,
    reason = "initializer indices are bounded by the input length"
)]
fn array_conversion<'a, T: Copy + Sync, U: Send + Sync + 'static>(
    values: &'a [T],
    collector: &'a Arc<GarbageCollector>,
    convert: fn(T) -> U,
    reference: fn(ristretto_gc::HeapBuffer<U>) -> Reference,
) -> ValueConversion<'a> {
    ValueConversion::Async(Box::pin(async move {
        let buffer =
            ristretto_gc::HeapBuffer::try_with_fn_in_async(collector, values.len(), |index| {
                Ok(convert(values[index]))
            })
            .await?;
        Ok(Value::try_new_object_async(collector, reference(buffer)).await?)
    }))
}

/// Trait for converting Rust values to `Value`.
pub trait RustValue: Debug {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value;

    /// Converts arguments while propagating managed allocation failure.
    /// # Errors
    /// Returns an allocation error when conversion needs a Java object.
    fn try_to_value(&self, collector: &Arc<GarbageCollector>) -> crate::Result<Value> {
        Ok(self.to_value(collector))
    }

    /// Converts a VM argument with asynchronous recovery when supported.
    /// Built-in vectors initialize directly in bounded batches. Custom embedding
    /// implementations keep their synchronous behavior unless they override this hook.
    fn try_to_value_async<'a>(
        &'a self,
        collector: &'a Arc<GarbageCollector>,
    ) -> ValueConversion<'a> {
        ValueConversion::Ready(std::future::ready(self.try_to_value(collector)))
    }

    /// Return the underlying Rust string when this value needs conversion to a Java string.
    ///
    /// Most values return `None`, allowing the VM's normal invocation path to avoid inspecting
    /// object arguments just to distinguish Rust strings from Java objects.
    fn as_rust_string(&self) -> Option<&str> {
        None
    }
}

impl RustValue for bool {
    fn to_value(&self, _collector: &Arc<GarbageCollector>) -> Value {
        Value::from(*self)
    }
}

impl RustValue for char {
    fn to_value(&self, _collector: &Arc<GarbageCollector>) -> Value {
        Value::from(*self)
    }
}

impl RustValue for i8 {
    fn to_value(&self, _collector: &Arc<GarbageCollector>) -> Value {
        Value::from(*self)
    }
}

impl RustValue for u8 {
    fn to_value(&self, _collector: &Arc<GarbageCollector>) -> Value {
        Value::from(*self)
    }
}

impl RustValue for i16 {
    fn to_value(&self, _collector: &Arc<GarbageCollector>) -> Value {
        Value::from(*self)
    }
}

impl RustValue for u16 {
    fn to_value(&self, _collector: &Arc<GarbageCollector>) -> Value {
        Value::from(*self)
    }
}

impl RustValue for i32 {
    fn to_value(&self, _collector: &Arc<GarbageCollector>) -> Value {
        Value::from(*self)
    }
}

impl RustValue for u32 {
    fn to_value(&self, _collector: &Arc<GarbageCollector>) -> Value {
        Value::from(*self)
    }
}

impl RustValue for i64 {
    fn to_value(&self, _collector: &Arc<GarbageCollector>) -> Value {
        Value::from(*self)
    }
}

impl RustValue for u64 {
    fn to_value(&self, _collector: &Arc<GarbageCollector>) -> Value {
        Value::from(*self)
    }
}

impl RustValue for isize {
    fn to_value(&self, _collector: &Arc<GarbageCollector>) -> Value {
        Value::from(*self)
    }
}

impl RustValue for usize {
    fn to_value(&self, _collector: &Arc<GarbageCollector>) -> Value {
        Value::from(*self)
    }
}

impl RustValue for f32 {
    fn to_value(&self, _collector: &Arc<GarbageCollector>) -> Value {
        Value::from(*self)
    }
}

impl RustValue for f64 {
    fn to_value(&self, _collector: &Arc<GarbageCollector>) -> Value {
        Value::from(*self)
    }
}

impl RustValue for &str {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        use ristretto_classfile::{ClassFile, ConstantPool};
        use ristretto_classloader::{Class, Object};

        const STRING_PREFIX: &str = "str:";
        let class_name = [STRING_PREFIX, self].concat();
        let mut constant_pool = ConstantPool::new();
        let class_index = constant_pool.add_class(class_name).unwrap_or_default();
        let class_file = ClassFile {
            constant_pool,
            this_class: class_index,
            ..Default::default()
        };
        Class::from(None, class_file)
            .and_then(Object::new)
            .map_or(Value::Object(None), |object| {
                Value::from_object(collector, object)
            })
    }

    fn as_rust_string(&self) -> Option<&str> {
        Some(self)
    }
}

impl RustValue for String {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        self.as_str().to_value(collector)
    }

    fn as_rust_string(&self) -> Option<&str> {
        Some(self)
    }
}

impl RustValue for Value {
    fn to_value(&self, _collector: &Arc<GarbageCollector>) -> Value {
        self.clone()
    }
}

impl RustValue for Vec<bool> {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }

    fn try_to_value(&self, collector: &Arc<GarbageCollector>) -> crate::Result<Value> {
        Ok(Value::try_new_object(
            collector,
            Reference::from(self.clone()),
        )?)
    }
    fn try_to_value_async<'a>(
        &'a self,
        collector: &'a Arc<GarbageCollector>,
    ) -> ValueConversion<'a> {
        array_conversion(self, collector, i8::from, Reference::BooleanArray)
    }
}

impl RustValue for Vec<char> {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }

    fn try_to_value(&self, collector: &Arc<GarbageCollector>) -> crate::Result<Value> {
        Ok(Value::try_new_object(
            collector,
            Reference::from(self.clone()),
        )?)
    }
    fn try_to_value_async<'a>(
        &'a self,
        collector: &'a Arc<GarbageCollector>,
    ) -> ValueConversion<'a> {
        array_conversion(self, collector, |value| value as u16, Reference::CharArray)
    }
}

impl RustValue for Vec<i8> {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }

    fn try_to_value(&self, collector: &Arc<GarbageCollector>) -> crate::Result<Value> {
        Ok(Value::try_new_object(
            collector,
            Reference::from(self.clone()),
        )?)
    }
    fn try_to_value_async<'a>(
        &'a self,
        collector: &'a Arc<GarbageCollector>,
    ) -> ValueConversion<'a> {
        array_conversion(self, collector, |value| value, Reference::ByteArray)
    }
}

impl RustValue for Vec<u8> {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        let values: &[i8] = zerocopy::transmute_ref!(self.as_slice());
        Value::new_object(collector, Reference::from(values.to_vec()))
    }

    fn try_to_value(&self, collector: &Arc<GarbageCollector>) -> crate::Result<Value> {
        let values: &[i8] = zerocopy::transmute_ref!(self.as_slice());
        Ok(Value::try_new_object(
            collector,
            Reference::from(values.to_vec()),
        )?)
    }
    fn try_to_value_async<'a>(
        &'a self,
        collector: &'a Arc<GarbageCollector>,
    ) -> ValueConversion<'a> {
        array_conversion(self, collector, u8::cast_signed, Reference::ByteArray)
    }
}

impl RustValue for Vec<i16> {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }

    fn try_to_value(&self, collector: &Arc<GarbageCollector>) -> crate::Result<Value> {
        Ok(Value::try_new_object(
            collector,
            Reference::from(self.clone()),
        )?)
    }
    fn try_to_value_async<'a>(
        &'a self,
        collector: &'a Arc<GarbageCollector>,
    ) -> ValueConversion<'a> {
        array_conversion(self, collector, |value| value, Reference::ShortArray)
    }
}

impl RustValue for Vec<u16> {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        let values: Vec<i64> = self.iter().map(|&x| i64::from(x)).collect();

        Value::new_object(collector, Reference::from(values))
    }

    fn try_to_value(&self, collector: &Arc<GarbageCollector>) -> crate::Result<Value> {
        let values: Vec<i64> = self.iter().map(|&x| i64::from(x)).collect();

        Ok(Value::try_new_object(collector, Reference::from(values))?)
    }
    fn try_to_value_async<'a>(
        &'a self,
        collector: &'a Arc<GarbageCollector>,
    ) -> ValueConversion<'a> {
        array_conversion(self, collector, i64::from, Reference::LongArray)
    }
}

impl RustValue for Vec<i32> {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }

    fn try_to_value(&self, collector: &Arc<GarbageCollector>) -> crate::Result<Value> {
        Ok(Value::try_new_object(
            collector,
            Reference::from(self.clone()),
        )?)
    }
    fn try_to_value_async<'a>(
        &'a self,
        collector: &'a Arc<GarbageCollector>,
    ) -> ValueConversion<'a> {
        array_conversion(self, collector, |value| value, Reference::IntArray)
    }
}

impl RustValue for Vec<u32> {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        let values: Vec<i64> = self.iter().map(|&x| i64::from(x)).collect();
        Value::new_object(collector, Reference::from(values))
    }

    fn try_to_value(&self, collector: &Arc<GarbageCollector>) -> crate::Result<Value> {
        let values: Vec<i64> = self.iter().map(|&x| i64::from(x)).collect();
        Ok(Value::try_new_object(collector, Reference::from(values))?)
    }
    fn try_to_value_async<'a>(
        &'a self,
        collector: &'a Arc<GarbageCollector>,
    ) -> ValueConversion<'a> {
        array_conversion(self, collector, i64::from, Reference::LongArray)
    }
}

impl RustValue for Vec<i64> {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }

    fn try_to_value(&self, collector: &Arc<GarbageCollector>) -> crate::Result<Value> {
        Ok(Value::try_new_object(
            collector,
            Reference::from(self.clone()),
        )?)
    }
    fn try_to_value_async<'a>(
        &'a self,
        collector: &'a Arc<GarbageCollector>,
    ) -> ValueConversion<'a> {
        array_conversion(self, collector, |value| value, Reference::LongArray)
    }
}

impl RustValue for Vec<u64> {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        let values: &[i64] = zerocopy::transmute_ref!(self.as_slice());
        Value::new_object(collector, Reference::from(values.to_vec()))
    }

    fn try_to_value(&self, collector: &Arc<GarbageCollector>) -> crate::Result<Value> {
        let values: &[i64] = zerocopy::transmute_ref!(self.as_slice());
        Ok(Value::try_new_object(
            collector,
            Reference::from(values.to_vec()),
        )?)
    }
    fn try_to_value_async<'a>(
        &'a self,
        collector: &'a Arc<GarbageCollector>,
    ) -> ValueConversion<'a> {
        array_conversion(self, collector, u64::cast_signed, Reference::LongArray)
    }
}

impl RustValue for Vec<isize> {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        let values: Vec<i64> = self.iter().map(|&x| x as i64).collect();
        Value::new_object(collector, Reference::from(values))
    }

    fn try_to_value(&self, collector: &Arc<GarbageCollector>) -> crate::Result<Value> {
        let values: Vec<i64> = self.iter().map(|&x| x as i64).collect();
        Ok(Value::try_new_object(collector, Reference::from(values))?)
    }
    fn try_to_value_async<'a>(
        &'a self,
        collector: &'a Arc<GarbageCollector>,
    ) -> ValueConversion<'a> {
        array_conversion(self, collector, |value| value as i64, Reference::LongArray)
    }
}

impl RustValue for Vec<usize> {
    #[expect(clippy::cast_possible_wrap)]
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        let values: Vec<i64> = self.iter().map(|&x| x as i64).collect();
        Value::new_object(collector, Reference::from(values))
    }

    #[expect(clippy::cast_possible_wrap)]
    fn try_to_value(&self, collector: &Arc<GarbageCollector>) -> crate::Result<Value> {
        let values: Vec<i64> = self.iter().map(|&x| x as i64).collect();
        Ok(Value::try_new_object(collector, Reference::from(values))?)
    }
    #[expect(
        clippy::cast_possible_wrap,
        reason = "preserve the existing Rust argument conversion"
    )]
    fn try_to_value_async<'a>(
        &'a self,
        collector: &'a Arc<GarbageCollector>,
    ) -> ValueConversion<'a> {
        array_conversion(self, collector, |value| value as i64, Reference::LongArray)
    }
}

impl RustValue for Vec<f32> {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }

    fn try_to_value(&self, collector: &Arc<GarbageCollector>) -> crate::Result<Value> {
        Ok(Value::try_new_object(
            collector,
            Reference::from(self.clone()),
        )?)
    }
    fn try_to_value_async<'a>(
        &'a self,
        collector: &'a Arc<GarbageCollector>,
    ) -> ValueConversion<'a> {
        array_conversion(self, collector, |value| value, Reference::FloatArray)
    }
}

impl RustValue for Vec<f64> {
    fn to_value(&self, collector: &Arc<GarbageCollector>) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }

    fn try_to_value(&self, collector: &Arc<GarbageCollector>) -> crate::Result<Value> {
        Ok(Value::try_new_object(
            collector,
            Reference::from(self.clone()),
        )?)
    }
    fn try_to_value_async<'a>(
        &'a self,
        collector: &'a Arc<GarbageCollector>,
    ) -> ValueConversion<'a> {
        array_conversion(self, collector, |value| value, Reference::DoubleArray)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;

    #[tokio::test]
    async fn asynchronous_vectors_preserve_embedding_conversion_semantics() -> crate::Result<()> {
        let inputs: Vec<Box<dyn RustValue>> = vec![
            Box::new(vec![false, true]),
            Box::new(vec!['a', '\u{1f600}']),
            Box::new(vec![i8::MIN, i8::MAX]),
            Box::new(vec![0_u8, u8::MAX]),
            Box::new(vec![i16::MIN, i16::MAX]),
            Box::new(vec![0_u16, u16::MAX]),
            Box::new(vec![i32::MIN, i32::MAX]),
            Box::new(vec![0_u32, u32::MAX]),
            Box::new(vec![i64::MIN, i64::MAX]),
            Box::new(vec![0_u64, u64::MAX]),
            Box::new(vec![isize::MIN, isize::MAX]),
            Box::new(vec![0_usize, usize::MAX]),
            Box::new(vec![-0.0_f32, f32::from_bits(0x7f80_0001)]),
            Box::new(vec![-0.0_f64, f64::from_bits(0x7ff0_0000_0000_0001)]),
        ];
        for compact in [false, true] {
            let gc = GarbageCollector::with_config(
                ristretto_gc::Configuration::builder()
                    .compact_heap(compact)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            for input in &inputs {
                let staged = gc
                    .region_heap()
                    .map_or(0, |heap| heap.statistics().bytes_staged);
                let value = input.try_to_value_async(&gc).await?;
                assert_eq!(
                    gc.region_heap()
                        .map_or(0, |heap| heap.statistics().bytes_staged),
                    staged
                );
                let legacy = input.try_to_value(&gc)?;
                assert_eq!(value, legacy, "conversion changed for {input:?}");
            }
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
        Ok(())
    }

    #[tokio::test]
    async fn scalar_and_custom_conversions_remain_object_safe_and_ready() -> crate::Result<()> {
        #[derive(Debug)]
        struct Custom(std::rc::Rc<i32>);
        impl RustValue for Custom {
            fn to_value(&self, _: &Arc<GarbageCollector>) -> Value {
                Value::Int(*self.0)
            }
        }
        let gc = GarbageCollector::new();
        let custom = Custom(std::rc::Rc::new(42));
        let inputs: [&dyn RustValue; 2] = [&7_i32, &custom];
        for (input, expected) in inputs.into_iter().zip([7, 42]) {
            let future = input.try_to_value_async(&gc);
            assert!(matches!(&future, ValueConversion::Ready(_)));
            assert_eq!(future.await?, Value::Int(expected));
        }
        assert_eq!(gc.managed_bytes(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn vector_arguments_recover_cooperatively_and_cancel_unpublished_storage()
    -> crate::Result<()> {
        use ristretto_gc::Gc;
        use std::cell::Cell;
        for compact in [false, true] {
            let gc = GarbageCollector::with_config(
                ristretto_gc::Configuration::builder()
                    .compact_heap(compact)
                    .diagnostics(true)
                    .max_heap_bytes(512 * 1024)
                    .allocation_threshold(usize::MAX)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .build(),
            );
            let live = Gc::try_new(&gc, vec![1_u8; 400 * 1024])?;
            let input = vec![9_u8; 200 * 1024];
            let done = Cell::new(false);
            let ticks = Cell::new(0);
            let allocation = async {
                let result = input.try_to_value_async(&gc).await;
                done.set(true);
                result
            };
            let companion = async {
                while !done.get() {
                    ticks.set(ticks.get() + 1);
                    tokio::task::yield_now().await;
                }
            };
            let (result, ()) = tokio::join!(allocation, companion);
            assert!(matches!(
                result,
                Err(crate::Error::JavaError(crate::JavaError::OutOfMemoryError(
                    _
                )))
            ));
            assert!(ticks.get() > 0);
            assert_eq!(gc.diagnostics().allocation_recovery.count, 0);
            assert!(gc.diagnostics().async_allocation_recovery.count > 0);
            assert_eq!(gc.diagnostics().allocation_waiters, 0);
            drop(live);
            gc.collect_full_async().await?;
            gc.collect_full_async().await?;
            // The first pending poll owns initialized but unpublished storage.
            let mut future = Box::pin(input.try_to_value_async(&gc));
            let waker = std::task::Waker::noop();
            let mut context = Context::from_waker(waker);
            assert!(future.as_mut().poll(&mut context).is_pending());
            assert_eq!(gc.managed_bytes(), 0);
            drop(future);
            assert!(input.iter().all(|&byte| byte == 9));
            if let Some(heap) = gc.region_heap() {
                assert_eq!(heap.statistics().occupied_bytes, 0);
            }
            gc.stop()?;
        }
        Ok(())
    }

    #[test]
    fn test_bool() {
        let collector = GarbageCollector::new();
        assert_eq!(true.to_value(&collector), Value::from(true));
        assert_eq!(false.to_value(&collector), Value::from(false));
    }

    #[test]
    fn test_char() {
        let collector = GarbageCollector::new();
        assert_eq!('a'.to_value(&collector), Value::from('a'));
    }

    #[test]
    fn test_i8() {
        let collector = GarbageCollector::new();
        assert_eq!(42i8.to_value(&collector), Value::from(42i8));
    }

    #[test]
    fn test_u8() {
        let collector = GarbageCollector::new();
        assert_eq!(42u8.to_value(&collector), Value::from(42u8));
    }

    #[test]
    fn test_i16() {
        let collector = GarbageCollector::new();
        assert_eq!(42i16.to_value(&collector), Value::from(42i16));
    }

    #[test]
    fn test_u16() {
        let collector = GarbageCollector::new();
        assert_eq!(42u16.to_value(&collector), Value::from(42u16));
    }

    #[test]
    fn test_i32() {
        let collector = GarbageCollector::new();
        assert_eq!(42i32.to_value(&collector), Value::from(42i32));
    }

    #[test]
    fn test_u32() {
        let collector = GarbageCollector::new();
        assert_eq!(42u32.to_value(&collector), Value::from(42u32));
    }

    #[test]
    fn test_i64() {
        let collector = GarbageCollector::new();
        assert_eq!(42i64.to_value(&collector), Value::from(42i64));
    }

    #[test]
    fn test_u64() {
        let collector = GarbageCollector::new();
        assert_eq!(42u64.to_value(&collector), Value::from(42u64));
    }

    #[test]
    fn test_isize() {
        let collector = GarbageCollector::new();
        assert_eq!(42isize.to_value(&collector), Value::from(42isize));
    }

    #[test]
    fn test_usize() {
        let collector = GarbageCollector::new();
        assert_eq!(42usize.to_value(&collector), Value::from(42usize));
    }

    #[test]
    fn test_f32() {
        let collector = GarbageCollector::new();
        assert_eq!(42.1f32.to_value(&collector), Value::from(42.1f32));
    }

    #[test]
    fn test_f64() {
        let collector = GarbageCollector::new();
        assert_eq!(42.1f64.to_value(&collector), Value::from(42.1f64));
    }

    #[test]
    fn test_str() {
        let collector = GarbageCollector::new();
        assert_eq!(Some("foo"), "foo".as_rust_string());
        let value = "foo".to_value(&collector);
        let object = value.as_object_ref().expect("object ref");
        let class_name = object.class().name();
        assert_eq!("str:foo", class_name);
    }

    #[test]
    fn test_string() {
        let collector = GarbageCollector::new();
        let string = "foo".to_string();
        assert_eq!(Some("foo"), string.as_rust_string());
        let value = string.to_value(&collector);
        let object = value.as_object_ref().expect("object ref");
        let class_name = object.class().name();
        assert_eq!("str:foo", class_name);
    }

    #[test]
    fn test_value() {
        let collector = GarbageCollector::new();
        let value = Value::from(42);
        assert_eq!(None, value.as_rust_string());
        assert_eq!(value.to_value(&collector), value);
    }

    #[test]
    fn test_vec_bool() {
        let collector = GarbageCollector::new();
        let value = vec![true, false].to_value(&collector);
        let bytes = value.as_bytes().expect("bytes");
        assert_eq!(bytes.len(), 2);
    }

    #[test]
    fn test_vec_char() {
        let collector = GarbageCollector::new();
        let value = vec!['a', 'b'].to_value(&collector);
        let chars = value.as_char_vec_ref().expect("chars");
        assert_eq!(chars.len(), 2);
    }

    #[test]
    fn test_vec_i8() {
        let collector = GarbageCollector::new();
        let value = vec![42i8, 43i8].to_value(&collector);
        let bytes = value.as_bytes().expect("bytes");
        assert_eq!(bytes.len(), 2);
    }

    #[test]
    fn test_vec_u8() {
        let collector = GarbageCollector::new();
        let value = vec![42u8, 43u8].to_value(&collector);
        let bytes = value.as_bytes().expect("bytes");
        assert_eq!(bytes.len(), 2);
    }

    #[test]
    fn test_vec_i16() {
        let collector = GarbageCollector::new();
        let value = vec![42i16, 43i16].to_value(&collector);
        let reference = value.as_reference().expect("reference");
        assert!(matches!(&*reference, Reference::ShortArray(_)));
    }

    #[test]
    fn test_vec_u16() {
        let collector = GarbageCollector::new();
        let value = vec![42u16, 43u16].to_value(&collector);
        // mapped to long array
        let reference = value.as_reference().expect("reference");
        assert!(matches!(&*reference, Reference::LongArray(_)));
    }

    #[test]
    fn test_vec_i32() {
        let collector = GarbageCollector::new();
        let value = vec![42i32, 43i32].to_value(&collector);
        let ints = value.as_int_vec_ref().expect("ints");
        assert_eq!(ints.len(), 2);
    }

    #[test]
    fn test_vec_u32() {
        let collector = GarbageCollector::new();
        let value = vec![42u32, 43u32].to_value(&collector);
        // mapped to long array
        let longs = value.as_long_vec_ref().expect("longs");
        assert_eq!(longs.len(), 2);
    }

    #[test]
    fn test_vec_i64() {
        let collector = GarbageCollector::new();
        let value = vec![42i64, 43i64].to_value(&collector);
        let longs = value.as_long_vec_ref().expect("longs");
        assert_eq!(longs.len(), 2);
    }

    #[test]
    fn test_vec_u64() {
        let collector = GarbageCollector::new();
        let value = vec![42u64, 43u64].to_value(&collector);
        let longs = value.as_long_vec_ref().expect("longs");
        assert_eq!(longs.len(), 2);
    }

    #[test]
    fn test_vec_isize() {
        let collector = GarbageCollector::new();
        let value = vec![42isize, 43isize].to_value(&collector);
        let longs = value.as_long_vec_ref().expect("longs");
        assert_eq!(longs.len(), 2);
    }

    #[test]
    fn test_vec_usize() {
        let collector = GarbageCollector::new();
        let value = vec![42usize, 43usize].to_value(&collector);
        let longs = value.as_long_vec_ref().expect("longs");
        assert_eq!(longs.len(), 2);
    }

    #[test]
    fn test_vec_f32() {
        let collector = GarbageCollector::new();
        let value = vec![42.1f32, 43.1f32].to_value(&collector);
        let floats = value.as_float_vec_ref().expect("floats");
        assert_eq!(floats.len(), 2);
    }

    #[test]
    fn test_vec_f64() {
        let collector = GarbageCollector::new();
        let value = vec![42.1f64, 43.1f64].to_value(&collector);
        let doubles = value.as_double_vec_ref().expect("doubles");
        assert_eq!(doubles.len(), 2);
    }
}

//! Array guards never span runtime waits or throwable construction.
use super::{
    ArrayIndexOutOfBoundsException, Gc, InternalError, NullPointerException, Ordering, Reference,
    Result, RuntimeContext, RwLock, Value, gc_ref_from_ptr, run_async, set_top_frame_pc,
    store_pending_error, store_pending_error_async,
};
use ristretto_jit::{ArrayAccessKind, ArrayAccessRequest};

impl RuntimeContext {
    pub(crate) async fn invoke_array_access(
        &self,
        request: ArrayAccessRequest,
        arguments: &[i64],
    ) -> Result<i64> {
        let bci = i32::try_from(request.bci)?;
        self.current_bci.store(bci, Ordering::Relaxed);
        set_top_frame_pc(self, bci);
        let result = match Operation::new(self, request.kind, arguments) {
            Ok(operation) => operation.complete().await,
            Err(error) => Err(error),
        };
        match result {
            Ok(value) => Ok(result_bits(self, value)),
            Err(error) => {
                store_pending_error_async(self, error).await;
                Ok(0)
            }
        }
    }
}

/// Synchronous embedding preserves its completion contract. Uncontended successful
/// accesses do not enter the executor bridge; contention and errors do so without guards.
pub(super) fn synchronous(ctx: &RuntimeContext, kind: ArrayAccessKind, arguments: &[i64]) -> i64 {
    let result = Operation::new(ctx, kind, arguments).and_then(|operation| {
        operation
            .try_complete()
            .unwrap_or_else(|| run_async(operation.complete()))
    });
    match result {
        Ok(value) => result_bits(ctx, value),
        Err(error) => {
            store_pending_error(ctx, error);
            0
        }
    }
}

fn result_bits(ctx: &RuntimeContext, value: Value) -> i64 {
    match value {
        Value::Int(value) => i64::from(value),
        Value::Long(value) => value,
        Value::Float(value) => i64::from(value.to_bits()),
        Value::Double(value) => i64::from_ne_bytes(value.to_bits().to_ne_bytes()),
        Value::Object(Some(owner)) => {
            ctx.retain_helper_result(&owner);
            owner.as_handle_i64()
        }
        _ => 0,
    }
}

struct Operation {
    kind: ArrayAccessKind,
    array: Gc<RwLock<Reference>>,
    index: i32,
    value: i64,
}

impl Operation {
    fn new(ctx: &RuntimeContext, kind: ArrayAccessKind, arguments: &[i64]) -> Result<Self> {
        if arguments.len() != kind.argument_count() {
            return Err(InternalError(
                "invalid array-access argument count".to_owned(),
            ));
        }
        let array = arguments
            .first()
            .copied()
            .ok_or_else(|| InternalError("missing array".to_owned()))?;
        if array == 0 {
            return Err(NullPointerException(None).into());
        }
        Ok(Self {
            kind,
            array: gc_ref_from_ptr(ctx, array)?,
            index: arguments
                .get(1)
                .copied()
                .map(i32::try_from)
                .transpose()?
                .unwrap_or_default(),
            value: arguments.get(2).copied().unwrap_or_default(),
        })
    }

    async fn complete(&self) -> Result<Value> {
        loop {
            if let Some(result) = self.try_complete() {
                return result;
            }
            // A held array blocks only this operation. No borrowed guard or native
            // generated frame survives the yield; cancellation releases this owner.
            tokio::task::yield_now().await;
        }
    }

    fn try_complete(&self) -> Option<Result<Value>> {
        if self.kind.argument_count() == 3 {
            let mut guard = self.array.try_write()?;
            Some(self.store(&mut guard))
        } else {
            let guard = self.array.try_read()?;
            Some(self.load(&guard))
        }
    }

    fn load(&self, array: &Reference) -> Result<Value> {
        use ArrayAccessKind as A;
        Ok(match (self.kind, array) {
            (A::Length, array) => {
                let length = match array {
                    Reference::BooleanArray(a) | Reference::ByteArray(a) => a.len(),
                    Reference::CharArray(a) => a.len(),
                    Reference::ShortArray(a) => a.len(),
                    Reference::IntArray(a) => a.len(),
                    Reference::LongArray(a) => a.len(),
                    Reference::FloatArray(a) => a.len(),
                    Reference::DoubleArray(a) => a.len(),
                    Reference::Array(a) => a.elements.len(),
                    Reference::Object(_) => return Err(self.wrong_type(array)),
                };
                Value::Int(i32::try_from(length)?)
            }
            (A::ByteLoad, Reference::ByteArray(a) | Reference::BooleanArray(a)) => {
                Value::Int(i32::from(*element(a, self.index)?))
            }
            (A::CharLoad, Reference::CharArray(a)) => {
                Value::Int(i32::from(*element(a, self.index)?))
            }
            (A::ShortLoad, Reference::ShortArray(a)) => {
                Value::Int(i32::from(*element(a, self.index)?))
            }
            (A::IntLoad, Reference::IntArray(a)) => Value::Int(*element(a, self.index)?),
            (A::LongLoad, Reference::LongArray(a)) => Value::Long(*element(a, self.index)?),
            (A::FloatLoad, Reference::FloatArray(a)) => Value::Float(*element(a, self.index)?),
            (A::DoubleLoad, Reference::DoubleArray(a)) => Value::Double(*element(a, self.index)?),
            (A::ReferenceLoad, Reference::Array(a)) => {
                let value = element(&a.elements, self.index)?;
                if !matches!(value, Value::Object(_)) {
                    return Err(InternalError(
                        "aaload: element is not an object reference".to_owned(),
                    ));
                }
                // Create the owning result before releasing the array guard. Its
                // helper registration is installed afterward, outside that guard.
                value.clone()
            }
            _ => return Err(self.wrong_type(array)),
        })
    }

    fn store(&self, array: &mut Reference) -> Result<Value> {
        use ArrayAccessKind as A;
        match (self.kind, &mut *array) {
            (A::ByteStore, Reference::BooleanArray(a)) => {
                *element_mut(a, self.index)? = (self.value & 1) as i8;
            }
            (A::ByteStore, Reference::ByteArray(a)) => {
                *element_mut(a, self.index)? = self.value as i8;
            }
            (A::CharStore, Reference::CharArray(a)) => {
                *element_mut(a, self.index)? = self.value as u16;
            }
            (A::ShortStore, Reference::ShortArray(a)) => {
                *element_mut(a, self.index)? = self.value as i16;
            }
            (A::IntStore, Reference::IntArray(a)) => {
                *element_mut(a, self.index)? = self.value as i32;
            }
            (A::LongStore, Reference::LongArray(a)) => *element_mut(a, self.index)? = self.value,
            (A::FloatStore, Reference::FloatArray(a)) => {
                *element_mut(a, self.index)? = f32::from_bits(self.value as u32);
            }
            (A::DoubleStore, Reference::DoubleArray(a)) => {
                *element_mut(a, self.index)? = f64::from_bits(self.value as u64);
            }
            _ => return Err(self.wrong_type(array)),
        }
        Ok(Value::Int(0))
    }

    fn wrong_type(&self, array: &Reference) -> crate::Error {
        InternalError(format!(
            "{:?}: invalid array variant {:?}",
            self.kind,
            std::mem::discriminant(array)
        ))
    }
}

fn index(index: i32, length: usize) -> Result<usize> {
    usize::try_from(index)
        .ok()
        .filter(|index| *index < length)
        .ok_or_else(|| ArrayIndexOutOfBoundsException { index, length }.into())
}
fn element<T>(values: &[T], offset: i32) -> Result<&T> {
    let offset = index(offset, values.len())?;
    values
        .get(offset)
        .ok_or_else(|| InternalError("validated array index missing".to_owned()))
}
fn element_mut<T>(values: &mut [T], offset: i32) -> Result<&mut T> {
    let offset = index(offset, values.len())?;
    values
        .get_mut(offset)
        .ok_or_else(|| InternalError("validated array index missing".to_owned()))
}

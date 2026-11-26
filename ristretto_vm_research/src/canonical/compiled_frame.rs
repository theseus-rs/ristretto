//! Typed canonical frame ownership for explicit native reference activations.
use super::frame_table::{Snapshot, Table, intern_reference};
use super::{Error, Value};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Boxed for a stable address. Raw context pointers exist only during a native resume.
pub(super) struct Context {
    table: Rc<RefCell<Table>>,
    frame: RefCell<Snapshot>,
    limit: usize,
    failed: Cell<bool>,
    error: Cell<Option<Error>>,
}

impl Context {
    pub(super) fn new(table: Rc<RefCell<Table>>) -> Result<Box<Self>, Error> {
        let (frame, limit) = {
            let table = table
                .try_borrow()
                .map_err(|_| Error::Boundary("reentrant canonical table"))?;
            (table.enter()?, table.publication_limit())
        };
        Ok(Box::new(Self {
            table,
            frame: RefCell::new(frame),
            limit,
            failed: Cell::new(false),
            error: Cell::new(None),
        }))
    }
    pub(super) fn pointer(&self) -> *const u8 {
        std::ptr::from_ref(self).cast()
    }
    pub(super) async fn export(
        &self,
        value: &Value,
        lease_attempts: usize,
    ) -> Result<ristretto_jit::Value, Error> {
        Ok(match value {
            Value::Int(value) => ristretto_jit::Value::I32(*value),
            Value::Long(value) => ristretto_jit::Value::I64(*value),
            Value::Float(value) => ristretto_jit::Value::F32(*value),
            Value::Double(value) => ristretto_jit::Value::F64(*value),
            Value::Reference(None) => ristretto_jit::Value::Ptr(0),
            Value::Reference(Some(root)) => ristretto_jit::Value::Ptr(
                intern_reference(&self.table, &self.frame, root, lease_attempts).await?,
            ),
            Value::Unused => return Err(Error::Invalid("unused compiled value".to_owned())),
        })
    }
    pub(super) fn reference(&self, token: i64) -> Result<Value, Error> {
        Ok(Value::Reference(
            self.table
                .try_borrow()
                .map_err(|_| Error::Boundary("reentrant canonical table"))?
                .resolve(token)?,
        ))
    }
    pub(super) fn take_error(&self) -> Option<Error> {
        self.error.take()
    }
    fn fail(&self, error: Error) {
        self.error.set(Some(error));
        self.failed.set(true);
    }
}

// SAFETY: these private symbols are linked only into admitted request-only functions.
// Their boxed Context lives throughout the native resume and generated stack root slices
// have the declared i64 layout. No pointer is retained or interpreted as a legacy context.
#[expect(
    unsafe_code,
    clippy::cast_ptr_alignment,
    reason = "private typed canonical native callback boundary"
)]
pub(super) unsafe extern "C" fn publish_roots(
    context: *const u8,
    roots: *const i64,
    count: i32,
) -> i32 {
    let context = unsafe { &*context.cast::<Context>() };
    if context.failed.get() {
        return 0;
    }
    let result = (|| {
        let count = usize::try_from(count).map_err(|_| Error::Boundary("negative root count"))?;
        if count > context.limit || (count != 0 && (roots.is_null() || !roots.is_aligned())) {
            return Err(Error::Boundary("canonical publication slice"));
        }
        let roots = if count == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(roots, count) }
        };
        let table = context
            .table
            .try_borrow()
            .map_err(|_| Error::Boundary("reentrant canonical table"))?;
        let mut frame = context
            .frame
            .try_borrow_mut()
            .map_err(|_| Error::Boundary("reentrant canonical frame"))?;
        table.publish(&mut frame, roots)
    })();
    match result {
        Ok(()) => 1,
        Err(error) => {
            context.fail(error);
            0
        }
    }
}

#[expect(
    unsafe_code,
    clippy::cast_ptr_alignment,
    reason = "private typed canonical native callback boundary"
)]
pub(super) unsafe extern "C" fn pending_error(context: *const u8) -> i64 {
    let context = unsafe { &*context.cast::<Context>() };
    i64::from(context.failed.get())
}

#[cfg(test)]
#[path = "compiled_frame_tests.rs"]
mod tests;

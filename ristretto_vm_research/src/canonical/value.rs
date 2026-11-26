use super::Error;

/// JVM computational value of a restricted primitive entry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScalarValue {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
}

impl ScalarValue {
    fn wide(self) -> bool {
        matches!(self, Self::Double(_) | Self::Long(_))
    }
    pub(super) fn int(self) -> Result<i32, Error> {
        if let Self::Int(n) = self {
            Ok(n)
        } else {
            Err(Error::Invalid("expected int".into()))
        }
    }
    pub(super) fn long(self) -> Result<i64, Error> {
        if let Self::Long(n) = self {
            Ok(n)
        } else {
            Err(Error::Invalid("expected long".into()))
        }
    }
    pub(super) fn float(self) -> Result<f32, Error> {
        if let Self::Float(n) = self {
            Ok(n)
        } else {
            Err(Error::Invalid("expected float".into()))
        }
    }
    pub(super) fn double(self) -> Result<f64, Error> {
        if let Self::Double(n) = self {
            Ok(n)
        } else {
            Err(Error::Invalid("expected double".into()))
        }
    }
}

pub(super) fn pop(stack: &mut Vec<ScalarValue>) -> Result<ScalarValue, Error> {
    stack
        .pop()
        .ok_or_else(|| Error::Invalid("stack underflow".into()))
}

pub(super) fn load(
    locals: &[Option<ScalarValue>],
    stack: &mut Vec<ScalarValue>,
    index: usize,
) -> Result<(), Error> {
    let value = locals
        .get(index)
        .copied()
        .flatten()
        .ok_or_else(|| Error::Invalid("uninitialized local".into()))?;
    stack.push(value);
    Ok(())
}

pub(super) fn store(
    locals: &mut [Option<ScalarValue>],
    stack: &mut Vec<ScalarValue>,
    index: usize,
) -> Result<(), Error> {
    let value = pop(stack)?;
    let invalid = || Error::Invalid("local index".into());
    if index > 0
        && locals
            .get(index - 1)
            .copied()
            .flatten()
            .is_some_and(ScalarValue::wide)
    {
        *locals.get_mut(index - 1).ok_or_else(invalid)? = None;
    }
    if value.wide() {
        *locals.get_mut(index + 1).ok_or_else(invalid)? = None;
    }
    *locals.get_mut(index).ok_or_else(invalid)? = Some(value);
    Ok(())
}

/// Scalar bytecodes operate on the primitive part of either kind of Java frame.
pub(super) trait PrimitiveFrameValue: From<ScalarValue> {
    type Slot;
    fn primitive(self) -> Result<ScalarValue, Error>;
    fn pop_scalar(stack: &mut Vec<Self>) -> Result<ScalarValue, Error> {
        stack
            .pop()
            .ok_or_else(|| Error::Invalid("stack underflow".into()))?
            .primitive()
    }
    fn load(locals: &[Self::Slot], stack: &mut Vec<Self>, index: usize) -> Result<(), Error>;
    fn store(locals: &mut [Self::Slot], stack: &mut Vec<Self>, index: usize) -> Result<(), Error>;
}

impl PrimitiveFrameValue for ScalarValue {
    type Slot = Option<Self>;
    fn primitive(self) -> Result<ScalarValue, Error> {
        Ok(self)
    }
    fn load(locals: &[Self::Slot], stack: &mut Vec<Self>, index: usize) -> Result<(), Error> {
        load(locals, stack, index)
    }
    fn store(locals: &mut [Self::Slot], stack: &mut Vec<Self>, index: usize) -> Result<(), Error> {
        store(locals, stack, index)
    }
}

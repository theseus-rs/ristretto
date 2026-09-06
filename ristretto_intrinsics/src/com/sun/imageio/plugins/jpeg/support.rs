//! Shared `ImageIO` state, Java callbacks and table marshalling.
use super::codec::{Codec, Tables};
use parking_lot::Mutex;
use portable_atomic::{AtomicI32, AtomicI64};
use ristretto_classloader::{Reference, Value};
use ristretto_gc::sync::RwLock;
use ristretto_gc::{Gc, GcRootGuard};
use ristretto_types::{Error, JavaError, JavaObject, Result, Thread, VM as _};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub(super) const READER: &str = "com/sun/imageio/plugins/jpeg/JPEGImageReader";
pub(super) const WRITER: &str = "com/sun/imageio/plugins/jpeg/JPEGImageWriter";

#[derive(Debug)]
pub(super) struct Context {
    pub codec: Mutex<Codec>,
    pub writer: bool,
    pub aborted: AtomicBool,
    pub color: AtomicI32,
    pub header: Mutex<Vec<u8>>,
    pub info: Mutex<[i32; 6]>,
}

impl Context {
    pub fn aborted(&self) -> bool {
        self.aborted.load(Ordering::Acquire)
    }

    pub fn abort(&self, flag: bool) {
        self.aborted.store(flag, Ordering::Release);
    }

    pub fn reset(&self) {
        self.codec.lock().reset();
        self.header.lock().clear();
        self.abort(false);
    }
}

#[derive(Debug)]
pub(super) struct State {
    pub contexts: RwLock<HashMap<i64, Arc<Context>>>,
    next: AtomicI64,
}

pub(super) fn state<T: Thread>(thread: &T) -> Result<Arc<State>> {
    thread.vm()?.resource_manager().get_or_init(|| State {
        contexts: RwLock::new(HashMap::new()),
        next: AtomicI64::new(1),
    })
}

pub(super) async fn context<T: Thread>(thread: &T, id: i64, writer: bool) -> Result<Arc<Context>> {
    let context = state(thread)?
        .contexts
        .read()
        .get(&id)
        .filter(|c| c.writer == writer)
        .cloned();
    if let Some(context) = context {
        return Ok(context);
    }
    let message = format!(
        "Attempting to use {} after dispose()",
        if writer { "writer" } else { "reader" }
    );
    let message = message.to_object(thread).await?;
    Err(Error::Throwable(
        thread
            .object(
                "java/lang/IllegalStateException",
                "Ljava/lang/String;",
                &[message],
            )
            .await?,
    ))
}

pub(super) fn create<T: Thread>(thread: &T, writer: bool) -> Result<i64> {
    let codec = Codec::new(writer).map_err(JavaError::OutOfMemoryError)?;
    let state = state(thread)?;
    let id = state.next.fetch_add(1, Ordering::Relaxed);
    if id <= 0 {
        return Err(JavaError::OutOfMemoryError("JPEG handle space exhausted".to_string()).into());
    }
    state.contexts.write().insert(
        id,
        Arc::new(Context {
            codec: Mutex::new(codec),
            writer,
            aborted: AtomicBool::new(false),
            color: AtomicI32::new(0),
            header: Mutex::new(Vec::new()),
            info: Mutex::new([0; 6]),
        }),
    );
    Ok(id)
}

pub(super) async fn io_error<T: Thread>(thread: &T, message: impl AsRef<str>) -> Error {
    let result = async {
        let message = message.as_ref().to_object(thread).await?;
        thread
            .object(
                "javax/imageio/IIOException",
                "Ljava/lang/String;",
                &[message],
            )
            .await
    }
    .await;
    match result {
        Ok(error) => Error::Throwable(error),
        Err(error) => error,
    }
}

pub(super) async fn checked<T: Thread, V>(
    thread: &T,
    result: std::result::Result<V, String>,
) -> Result<V> {
    match result {
        Ok(value) => Ok(value),
        Err(message) => Err(io_error(thread, message).await),
    }
}

pub(super) fn nonnull(value: &Value) -> Result<()> {
    if matches!(value, Value::Object(None)) {
        Err(JavaError::NullPointerException(Some("null JPEG argument".to_string())).into())
    } else {
        Ok(())
    }
}

pub(super) fn bytes<T: Thread>(thread: &T, data: &[u8]) -> Result<Value> {
    let vm = thread.vm()?;
    Ok(Value::new_object(
        vm.garbage_collector(),
        Reference::from(
            data.iter()
                .map(|v| i8::from_ne_bytes([*v]))
                .collect::<Vec<_>>(),
        ),
    ))
}

pub(super) async fn warning<T: Thread>(thread: &T, receiver: &Value, ctx: &Context) -> Result<()> {
    let message = ctx.codec.lock().warning();
    if let Some(message) = message {
        let message = message.to_object(thread).await?;
        thread
            .invoke(
                if ctx.writer { WRITER } else { READER },
                "warningWithMessage(Ljava/lang/String;)V",
                &[receiver.clone(), message],
            )
            .await?;
    }
    Ok(())
}

pub(super) async fn output<T: Thread>(thread: &T, receiver: &Value, ctx: &Context) -> Result<()> {
    let data = ctx.codec.lock().output();
    for chunk in data.chunks(4096) {
        let array = bytes(thread, chunk)?;
        thread
            .invoke(
                WRITER,
                "writeOutputData([BII)V",
                &[
                    receiver.clone(),
                    array,
                    Value::Int(0),
                    Value::Int(i32::try_from(chunk.len())?),
                ],
            )
            .await?;
    }
    warning(thread, receiver, ctx).await
}

/// Extract all four possible table slots, validating before codec operations.
pub(super) fn tables(q: &Value, dc: &Value, ac: &Value) -> Result<Tables> {
    let mut tables = Tables::default();
    if !matches!(q, Value::Object(None)) {
        let (_, values) = q.as_class_vec_ref()?;
        if values.len() > 4 {
            return Err(JavaError::IllegalArgumentException(
                "Too many JPEG quantization tables".to_string(),
            )
            .into());
        }
        tables.nq = i32::try_from(values.len())?;
        for (value, dest) in values.iter().zip(tables.q.iter_mut()) {
            nonnull(value)?;
            let array = value.as_object_ref()?.value("qTable")?;
            let values = array.as_int_vec_ref()?;
            if values.len() != 64 || values.iter().any(|v| !(0..=65535).contains(v)) {
                return Err(JavaError::IllegalArgumentException(
                    "Invalid JPEG quantization table".to_string(),
                )
                .into());
            }
            for (dest, value) in dest.iter_mut().zip(values.iter()) {
                *dest = u32::try_from(*value)?;
            }
        }
    }
    for (array, count, bits, symbols) in [
        (
            dc,
            &mut tables.ndc,
            &mut tables.dc_bits,
            &mut tables.dc_values,
        ),
        (
            ac,
            &mut tables.nac,
            &mut tables.ac_bits,
            &mut tables.ac_values,
        ),
    ] {
        if matches!(array, Value::Object(None)) {
            continue;
        }
        let (_, values) = array.as_class_vec_ref()?;
        if values.len() > 4 {
            return Err(JavaError::IllegalArgumentException(
                "Too many JPEG Huffman tables".to_string(),
            )
            .into());
        }
        *count = i32::try_from(values.len())?;
        for ((value, bits), symbols) in values.iter().zip(bits.iter_mut()).zip(symbols.iter_mut()) {
            nonnull(value)?;
            let lengths = value.as_object_ref()?.value("lengths")?;
            let lengths = lengths.as_short_vec_ref()?;
            let codes = value.as_object_ref()?.value("values")?;
            let codes = codes.as_short_vec_ref()?;
            if lengths.len() > 16
                || codes.len() > 256
                || lengths.iter().any(|v| !(0..=255).contains(v))
                || codes.iter().any(|v| !(0..=255).contains(v))
                || lengths.iter().map(|v| i32::from(*v)).sum::<i32>() != i32::try_from(codes.len())?
            {
                return Err(JavaError::IllegalArgumentException(
                    "Invalid JPEG Huffman table".to_string(),
                )
                .into());
            }
            for (dest, value) in bits.iter_mut().skip(1).zip(lengths.iter()) {
                *dest = u8::try_from(*value)?;
            }
            for (dest, value) in symbols.iter_mut().zip(codes.iter()) {
                *dest = u8::try_from(*value)?;
            }
        }
    }
    Ok(tables)
}

/// Pull bytes through the reader's callback and push back any read-ahead. Header
/// queries stop at SOS and therefore also work on streams containing only a header.
pub(super) struct Input<'a, T: Thread> {
    thread: &'a T,
    receiver: &'a Value,
    buffer: Value,
    _buffer_root: GcRootGuard<RwLock<Reference>>,
    pending: Vec<u8>,
    position: usize,
}

impl<'a, T: Thread> Input<'a, T> {
    pub fn new(thread: &'a T, receiver: &'a Value) -> Result<Self> {
        let root = Gc::new(
            thread.vm()?.garbage_collector(),
            RwLock::new(Reference::from(vec![0_i8; 4096])),
        );
        Ok(Self {
            thread,
            receiver,
            buffer: Value::Object(Some(root.clone_gc())),
            _buffer_root: root,
            pending: Vec::new(),
            position: 0,
        })
    }

    pub async fn byte(&mut self) -> Result<Option<u8>> {
        if self.position == self.pending.len() {
            let count = self
                .thread
                .try_invoke(
                    READER,
                    "readInputData([BII)I",
                    &[
                        self.receiver.clone(),
                        self.buffer.clone(),
                        Value::Int(0),
                        Value::Int(4096),
                    ],
                )
                .await?
                .as_i32()?;
            if count < 0 {
                return Ok(None);
            }
            if !(1..=4096).contains(&count) {
                return Err(io_error(
                    self.thread,
                    "ImageInputStream returned an invalid byte count",
                )
                .await);
            }
            let count = usize::try_from(count)?;
            self.pending = self
                .buffer
                .as_byte_vec_ref()?
                .iter()
                .take(count)
                .map(|v| v.to_ne_bytes()[0])
                .collect();
            self.position = 0;
        }
        let byte = self.pending.get(self.position).copied();
        self.position += 1;
        Ok(byte)
    }

    async fn required(&mut self) -> Result<u8> {
        match self.byte().await? {
            Some(byte) => Ok(byte),
            None => Err(io_error(self.thread, "Premature end of JPEG file").await),
        }
    }

    pub async fn finish(&mut self) -> Result<()> {
        let remaining = self.pending.len().saturating_sub(self.position);
        if remaining != 0 {
            self.thread
                .invoke(
                    READER,
                    "pushBack(I)V",
                    &[self.receiver.clone(), Value::Int(i32::try_from(remaining)?)],
                )
                .await?;
            self.position = self.pending.len();
        }
        Ok(())
    }

    pub async fn header(&mut self) -> Result<Vec<u8>> {
        let mut data = vec![self.required().await?, self.required().await?];
        if data != [0xff, 0xd8] {
            return Err(io_error(self.thread, "Not a JPEG file: missing SOI marker").await);
        }
        loop {
            let prefix = self.required().await?;
            data.push(prefix);
            if prefix != 0xff {
                // Preserve extraneous bytes for the codec marker parser.
                continue;
            }
            let mut marker = self.required().await?;
            data.push(marker);
            while marker == 0xff {
                marker = self.required().await?;
                data.push(marker);
            }
            if marker == 0xd9 || marker == 0xd8 {
                break;
            }
            if marker == 0 || marker == 0x01 || (0xd0..=0xd7).contains(&marker) {
                continue;
            }
            let high = self.required().await?;
            let low = self.required().await?;
            data.extend_from_slice(&[high, low]);
            let length = usize::from(u16::from_be_bytes([high, low]));
            if length < 2 {
                return Err(io_error(self.thread, "Invalid JPEG marker length").await);
            }
            for _ in 2..length {
                data.push(self.required().await?);
            }
            if marker == 0xda {
                break;
            }
        }
        Ok(data)
    }

    pub async fn image_tail(&mut self, data: &mut Vec<u8>) -> Result<()> {
        loop {
            let Some(byte) = self.byte().await? else {
                self.thread
                    .invoke(
                        READER,
                        "warningOccurred(I)V",
                        &[self.receiver.clone(), Value::Int(0)],
                    )
                    .await?;
                data.extend_from_slice(&[0xff, 0xd9]);
                return Ok(());
            };
            data.push(byte);
            if byte != 0xff {
                continue;
            }
            let mut marker = self.required().await?;
            data.push(marker);
            while marker == 0xff {
                marker = self.required().await?;
                data.push(marker);
            }
            if marker == 0xd9 {
                return Ok(());
            }
            if marker == 0 || marker == 0x01 || (0xd0..=0xd7).contains(&marker) {
                continue;
            }
            let high = self.required().await?;
            let low = self.required().await?;
            data.extend_from_slice(&[high, low]);
            let length = usize::from(u16::from_be_bytes([high, low]));
            if length < 2 {
                return Err(io_error(self.thread, "Invalid JPEG marker length").await);
            }
            for _ in 2..length {
                data.push(self.required().await?);
            }
        }
    }
}

/// Assemble APP2 ICC chunks in sequence order, rejecting incomplete/duplicate sets.
pub(super) fn icc_profile(header: &[u8]) -> std::result::Result<Option<Vec<u8>>, String> {
    let mut chunks = HashMap::new();
    let mut expected = None;
    let mut pos = 2;
    while let Some(&prefix) = header.get(pos) {
        if prefix != 0xff {
            break;
        }
        pos += 1;
        while header.get(pos) == Some(&0xff) {
            pos += 1;
        }
        let Some(&marker) = header.get(pos) else {
            break;
        };
        pos += 1;
        if marker == 0xd9 || marker == 0xda {
            break;
        }
        if marker == 0x01 || (0xd0..=0xd7).contains(&marker) {
            continue;
        }
        let Some(length) = header.get(pos..pos + 2) else {
            break;
        };
        let [high, low] = length else {
            break;
        };
        let length = usize::from(u16::from_be_bytes([*high, *low]));
        let Some(data) = header.get(pos + 2..pos + length) else {
            break;
        };
        if marker == 0xe2 && data.starts_with(b"ICC_PROFILE\0") {
            let Some(&[index, count, ref profile @ ..]) = data.get(12..) else {
                return Err("Invalid ICC profile data".to_string());
            };
            if index == 0
                || count == 0
                || index > count
                || expected.is_some_and(|old| old != count)
                || chunks.insert(index, profile).is_some()
            {
                return Err("Invalid ICC profile data".to_string());
            }
            expected = Some(count);
        }
        pos += length;
    }
    let Some(count) = expected else {
        return Ok(None);
    };
    if chunks.len() != usize::from(count) {
        return Err("Invalid ICC profile data".to_string());
    }
    let mut profile = Vec::new();
    for index in 1..=count {
        if let Some(chunk) = chunks.get(&index) {
            profile.extend_from_slice(chunk);
        }
    }
    Ok(Some(profile))
}

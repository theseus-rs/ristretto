use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_gc::sync::RwLock;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM as _};
use std::collections::HashMap;
use std::io::{self, Read};
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use zip::ZipArchive;

/// Pre-parsed information about a single ZIP entry.
pub(crate) struct ZipEntryInfo {
    /// Entry name as raw bytes (UTF-8)
    pub(crate) name_bytes: Vec<u8>,
    /// Extra field bytes (may be empty)
    pub(crate) extra_bytes: Vec<u8>,
    /// Comment bytes (may be empty)
    pub(crate) comment_bytes: Vec<u8>,
    /// Compressed size
    pub(crate) compressed_size: u64,
    /// Uncompressed size
    pub(crate) uncompressed_size: u64,
    /// CRC-32
    pub(crate) crc32: u32,
    /// Compression method (0 = stored, 8 = deflated)
    pub(crate) method: u16,
    /// General purpose bit flag
    pub(crate) flag: u16,
    /// Last modification time in DOS format packed as a long.
    /// High 16 bits = date, low 16 bits = time.
    pub(crate) last_modified_time: i64,
    /// Raw (decompressed) data of the entry
    pub(crate) data: Vec<u8>,
}

/// Context for an open ZIP file.
pub(crate) struct ZipFileContext {
    /// Pre-parsed entries
    pub(crate) entries: Vec<ZipEntryInfo>,
    /// ZIP-level comment bytes
    pub(crate) comment: Vec<u8>,
    /// Whether the file starts with a local file header (LOC signature)
    pub(crate) starts_with_loc: bool,
}

/// Per-VM storage for `ZipFile` native handles.
pub(crate) struct ZipFileState {
    /// Map from zip handle to `ZipFileContext`
    pub(crate) zip_handles: RwLock<HashMap<i64, ZipFileContext>>,
    pub(crate) next_zip_id: AtomicI64,
    /// Map from entry handle to (`zip_handle`, `entry_index`)
    pub(crate) entry_handles: RwLock<HashMap<i64, (i64, usize)>>,
    /// Next entry handle ID
    pub(crate) next_entry_id: AtomicI64,
}

impl ZipFileState {
    fn new() -> Self {
        Self {
            zip_handles: RwLock::new(HashMap::new()),
            next_zip_id: AtomicI64::new(1),
            entry_handles: RwLock::new(HashMap::new()),
            next_entry_id: AtomicI64::new(1),
        }
    }
}

pub(crate) fn get_zip_file_state<T: Thread + 'static>(
    thread: &Arc<T>,
) -> Result<Arc<ZipFileState>> {
    let vm = thread.vm()?;
    vm.resource_manager().get_or_init(ZipFileState::new)
}

/// Convert DOS date/time to a packed i64.
/// The `zip` crate provides `DateTime` which stores DOS date and time.
/// Java's `ZipFile.getEntryTime` returns a DOS timestamp:
///   bits[0..15] = time, bits[16..31] = date
fn dos_datetime_to_long(dt: zip::DateTime) -> i64 {
    let date = i64::from(dt.datepart());
    let time = i64::from(dt.timepart());
    (date << 16) | time
}

/// Implementation of `ZipFile` methods for Java 8 and earlier
#[intrinsic_method("java/util/zip/ZipFile.close(J)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn close<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;

    let state = get_zip_file_state(&thread)?;

    // Remove all entry handles associated with this zip handle
    {
        let mut entry_guard = state.entry_handles.write();
        entry_guard.retain(|_, (zh, _)| *zh != handle);
    }

    // Remove the zip handle
    {
        let mut zip_guard = state.zip_handles.write();
        zip_guard.remove(&handle);
    }

    Ok(None)
}

#[intrinsic_method("java/util/zip/ZipFile.freeEntry(JJ)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn free_entry<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let entry_handle = parameters.pop_long()?;
    let _zip_handle = parameters.pop_long()?;

    let state = get_zip_file_state(&thread)?;
    let mut entry_guard = state.entry_handles.write();
    entry_guard.remove(&entry_handle);

    Ok(None)
}

#[intrinsic_method("java/util/zip/ZipFile.getCommentBytes(J)[B", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_comment_bytes<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;

    let state = get_zip_file_state(&thread)?;
    let zip_guard = state.zip_handles.read();
    let Some(context) = zip_guard.get(&handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "ZipFile has been closed".to_string(),
        )
        .into());
    };

    if context.comment.is_empty() {
        return Ok(Some(Value::Object(None)));
    }

    let comment_bytes: Vec<i8> = context
        .comment
        .iter()
        .map(|&b| {
            #[expect(clippy::cast_possible_wrap)]
            let v = b as i8;
            v
        })
        .collect();
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();
    let reference = Reference::from(comment_bytes);
    Ok(Some(Value::new_object(gc, reference)))
}

// Note: `getEntry(J[BZ)J` is not registered as an intrinsic because it is not present as a native
// method in the supported JDK distributions; the public entry point is `getEntry(J[B)J` which
// dispatches to this helper.
#[async_method]
pub async fn get_entry<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let add_slash = parameters.pop_int()? != 0;
    let name_ref = parameters.pop_reference()?;
    let handle = parameters.pop_long()?;

    let Some(name_ref) = name_ref else {
        return Ok(Some(Value::Long(0)));
    };

    // Read the name bytes from the byte array reference
    let name_bytes: Vec<u8> = {
        let guard = name_ref.read();
        let bytes = guard.as_byte_vec_ref()?;
        bytes
            .iter()
            .map(|&b| {
                #[expect(clippy::cast_sign_loss)]
                let v = b as u8;
                v
            })
            .collect()
    };
    let name = String::from_utf8_lossy(&name_bytes);

    let state = get_zip_file_state(&thread)?;
    let zip_guard = state.zip_handles.read();
    let Some(context) = zip_guard.get(&handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "ZipFile has been closed".to_string(),
        )
        .into());
    };

    // Search for the entry by name
    let mut found_index = None;
    for (i, entry) in context.entries.iter().enumerate() {
        let entry_name = String::from_utf8_lossy(&entry.name_bytes);
        if entry_name == name {
            found_index = Some(i);
            break;
        }
    }

    // If not found and add_slash is true, try appending '/'
    if found_index.is_none() && add_slash {
        let name_with_slash = format!("{name}/");
        for (i, entry) in context.entries.iter().enumerate() {
            let entry_name = String::from_utf8_lossy(&entry.name_bytes);
            if entry_name == name_with_slash {
                found_index = Some(i);
                break;
            }
        }
    }

    let Some(index) = found_index else {
        return Ok(Some(Value::Long(0)));
    };

    // Create an entry handle
    let entry_handle = state.next_entry_id.fetch_add(1, Ordering::SeqCst);
    drop(zip_guard);
    let mut entry_guard = state.entry_handles.write();
    entry_guard.insert(entry_handle, (handle, index));

    Ok(Some(Value::Long(entry_handle)))
}

#[intrinsic_method("java/util/zip/ZipFile.getEntry(J[B)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry_no_add_slash<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    parameters.push_int(0);
    get_entry(thread, parameters).await
}

#[intrinsic_method("java/util/zip/ZipFile.getEntryBytes(JI)[B", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry_bytes<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let entry_type = parameters.pop_int()?;
    let entry_handle = parameters.pop_long()?;

    let state = get_zip_file_state(&thread)?;

    // Resolve entry handle to (zipHandle, index)
    let (zip_handle, index) = {
        let entry_guard = state.entry_handles.read();
        let Some(&(zh, idx)) = entry_guard.get(&entry_handle) else {
            return Ok(Some(Value::Object(None)));
        };
        (zh, idx)
    };

    let zip_guard = state.zip_handles.read();
    let Some(context) = zip_guard.get(&zip_handle) else {
        return Ok(Some(Value::Object(None)));
    };
    let Some(entry) = context.entries.get(index) else {
        return Ok(Some(Value::Object(None)));
    };

    // type: 0 = name, 1 = extra, 2 = comment
    let bytes: &[u8] = match entry_type {
        0 => &entry.name_bytes,
        1 => &entry.extra_bytes,
        2 => &entry.comment_bytes,
        _ => return Ok(Some(Value::Object(None))),
    };

    if bytes.is_empty() {
        return Ok(Some(Value::Object(None)));
    }

    let result_bytes: Vec<i8> = bytes
        .iter()
        .map(|&b| {
            #[expect(clippy::cast_possible_wrap)]
            let v = b as i8;
            v
        })
        .collect();
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();
    let reference = Reference::from(result_bytes);
    Ok(Some(Value::new_object(gc, reference)))
}

#[intrinsic_method("java/util/zip/ZipFile.getEntryCSize(J)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry_c_size<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let entry_handle = parameters.pop_long()?;

    let state = get_zip_file_state(&thread)?;
    let (zip_handle, index) = {
        let entry_guard = state.entry_handles.read();
        let Some(&(zh, idx)) = entry_guard.get(&entry_handle) else {
            return Err(ristretto_types::JavaError::RuntimeException(
                "Invalid entry handle".to_string(),
            )
            .into());
        };
        (zh, idx)
    };

    let zip_guard = state.zip_handles.read();
    let Some(context) = zip_guard.get(&zip_handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "ZipFile has been closed".to_string(),
        )
        .into());
    };
    let Some(entry) = context.entries.get(index) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Invalid entry index".to_string(),
        )
        .into());
    };

    #[expect(clippy::cast_possible_wrap)]
    Ok(Some(Value::Long(entry.compressed_size as i64)))
}

#[intrinsic_method("java/util/zip/ZipFile.getEntryCrc(J)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry_crc<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let entry_handle = parameters.pop_long()?;

    let state = get_zip_file_state(&thread)?;
    let (zip_handle, index) = {
        let entry_guard = state.entry_handles.read();
        let Some(&(zh, idx)) = entry_guard.get(&entry_handle) else {
            return Err(ristretto_types::JavaError::RuntimeException(
                "Invalid entry handle".to_string(),
            )
            .into());
        };
        (zh, idx)
    };

    let zip_guard = state.zip_handles.read();
    let Some(context) = zip_guard.get(&zip_handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "ZipFile has been closed".to_string(),
        )
        .into());
    };
    let Some(entry) = context.entries.get(index) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Invalid entry index".to_string(),
        )
        .into());
    };

    Ok(Some(Value::Long(i64::from(entry.crc32))))
}

#[intrinsic_method("java/util/zip/ZipFile.getEntryFlag(J)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry_flag<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let entry_handle = parameters.pop_long()?;

    let state = get_zip_file_state(&thread)?;
    let (zip_handle, index) = {
        let entry_guard = state.entry_handles.read();
        let Some(&(zh, idx)) = entry_guard.get(&entry_handle) else {
            return Err(ristretto_types::JavaError::RuntimeException(
                "Invalid entry handle".to_string(),
            )
            .into());
        };
        (zh, idx)
    };

    let zip_guard = state.zip_handles.read();
    let Some(context) = zip_guard.get(&zip_handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "ZipFile has been closed".to_string(),
        )
        .into());
    };
    let Some(entry) = context.entries.get(index) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Invalid entry index".to_string(),
        )
        .into());
    };

    Ok(Some(Value::Int(i32::from(entry.flag))))
}

#[intrinsic_method("java/util/zip/ZipFile.getEntryMethod(J)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry_method<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let entry_handle = parameters.pop_long()?;

    let state = get_zip_file_state(&thread)?;
    let (zip_handle, index) = {
        let entry_guard = state.entry_handles.read();
        let Some(&(zh, idx)) = entry_guard.get(&entry_handle) else {
            return Err(ristretto_types::JavaError::RuntimeException(
                "Invalid entry handle".to_string(),
            )
            .into());
        };
        (zh, idx)
    };

    let zip_guard = state.zip_handles.read();
    let Some(context) = zip_guard.get(&zip_handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "ZipFile has been closed".to_string(),
        )
        .into());
    };
    let Some(entry) = context.entries.get(index) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Invalid entry index".to_string(),
        )
        .into());
    };

    Ok(Some(Value::Int(i32::from(entry.method))))
}

#[intrinsic_method("java/util/zip/ZipFile.getEntrySize(J)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry_size<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let entry_handle = parameters.pop_long()?;

    let state = get_zip_file_state(&thread)?;
    let (zip_handle, index) = {
        let entry_guard = state.entry_handles.read();
        let Some(&(zh, idx)) = entry_guard.get(&entry_handle) else {
            return Err(ristretto_types::JavaError::RuntimeException(
                "Invalid entry handle".to_string(),
            )
            .into());
        };
        (zh, idx)
    };

    let zip_guard = state.zip_handles.read();
    let Some(context) = zip_guard.get(&zip_handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "ZipFile has been closed".to_string(),
        )
        .into());
    };
    let Some(entry) = context.entries.get(index) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Invalid entry index".to_string(),
        )
        .into());
    };

    #[expect(clippy::cast_possible_wrap)]
    Ok(Some(Value::Long(entry.uncompressed_size as i64)))
}

#[intrinsic_method("java/util/zip/ZipFile.getEntryTime(J)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry_time<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let entry_handle = parameters.pop_long()?;

    let state = get_zip_file_state(&thread)?;
    let (zip_handle, index) = {
        let entry_guard = state.entry_handles.read();
        let Some(&(zh, idx)) = entry_guard.get(&entry_handle) else {
            return Err(ristretto_types::JavaError::RuntimeException(
                "Invalid entry handle".to_string(),
            )
            .into());
        };
        (zh, idx)
    };

    let zip_guard = state.zip_handles.read();
    let Some(context) = zip_guard.get(&zip_handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "ZipFile has been closed".to_string(),
        )
        .into());
    };
    let Some(entry) = context.entries.get(index) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Invalid entry index".to_string(),
        )
        .into());
    };

    Ok(Some(Value::Long(entry.last_modified_time)))
}

#[intrinsic_method("java/util/zip/ZipFile.getManifestNum(J)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_manifest_num<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;

    let state = get_zip_file_state(&thread)?;
    let zip_guard = state.zip_handles.read();
    let Some(context) = zip_guard.get(&handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "ZipFile has been closed".to_string(),
        )
        .into());
    };

    let count = context
        .entries
        .iter()
        .filter(|e| {
            let name = String::from_utf8_lossy(&e.name_bytes);
            name.eq_ignore_ascii_case("META-INF/MANIFEST.MF")
        })
        .count();

    #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    Ok(Some(Value::Int(count as i32)))
}

#[intrinsic_method("java/util/zip/ZipFile.getNextEntry(JI)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_next_entry<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let handle = parameters.pop_long()?;

    let state = get_zip_file_state(&thread)?;
    let zip_guard = state.zip_handles.read();
    let Some(context) = zip_guard.get(&handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "ZipFile has been closed".to_string(),
        )
        .into());
    };

    #[expect(clippy::cast_sign_loss)]
    let index = index as usize;
    if index >= context.entries.len() {
        return Ok(Some(Value::Long(0)));
    }

    // Create an entry handle for this index
    let entry_handle = state.next_entry_id.fetch_add(1, Ordering::SeqCst);
    drop(zip_guard);
    let mut entry_guard = state.entry_handles.write();
    entry_guard.insert(entry_handle, (handle, index));

    Ok(Some(Value::Long(entry_handle)))
}

#[intrinsic_method("java/util/zip/ZipFile.getTotal(J)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_total<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;

    let state = get_zip_file_state(&thread)?;
    let zip_guard = state.zip_handles.read();
    let Some(context) = zip_guard.get(&handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "ZipFile has been closed".to_string(),
        )
        .into());
    };

    #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    Ok(Some(Value::Int(context.entries.len() as i32)))
}

#[intrinsic_method(
    "java/util/zip/ZipFile.getZipMessage(J)Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_zip_message<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    // No error message to report
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method("java/util/zip/ZipFile.initIDs()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/util/zip/ZipFile.open(Ljava/lang/String;IJZ)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn open<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _use_mmap = parameters.pop_int()?;
    let _last_modified = parameters.pop_long()?;
    let _mode = parameters.pop_int()?;
    let name_ref = parameters.pop_reference()?;

    let Some(name_ref) = name_ref else {
        return Err(ristretto_types::JavaError::NullPointerException(Some(
            "file name is null".to_string(),
        ))
        .into());
    };

    // Extract the file path string from the Java String reference
    let path = {
        let guard = name_ref.read();
        guard.as_string()?
    };

    if path.is_empty() {
        return Err(
            ristretto_types::JavaError::IoException("file path is empty".to_string()).into(),
        );
    }

    // Read the file bytes
    let file_bytes = std::fs::read(&path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            ristretto_types::JavaError::FileNotFoundException(format!("File not found: {path}"))
        } else {
            ristretto_types::JavaError::IoException(format!("Error reading file '{path}': {e}"))
        }
    })?;

    // Check if the file starts with a local file header signature (PK\x03\x04)
    let starts_with_loc = file_bytes.len() >= 4 && file_bytes[0..4] == [0x50, 0x4B, 0x03, 0x04];

    // Parse the ZIP archive
    let cursor = io::Cursor::new(file_bytes);
    let mut archive = ZipArchive::new(cursor).map_err(|e| {
        ristretto_types::JavaError::IoException(format!("Error opening zip file '{path}': {e}"))
    })?;

    // Extract comment
    let comment = archive.comment().to_vec();

    // Pre-parse all entries
    let mut entries = Vec::with_capacity(archive.len());
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| {
            ristretto_types::JavaError::IoException(format!(
                "Error reading zip entry {i} from '{path}': {e}"
            ))
        })?;

        let name_bytes = file.name_raw().to_vec();
        let extra_bytes = file.extra_data().map_or_else(Vec::new, <[u8]>::to_vec);
        let comment_bytes = file.comment().as_bytes().to_vec();
        let compressed_size = file.compressed_size();
        let uncompressed_size = file.size();
        let crc32 = file.crc32();
        let method = match file.compression() {
            zip::CompressionMethod::Deflated => 8,
            _ => 0,
        };
        // Check for UTF-8 name flag (bit 11 of the general purpose bit flag).
        // The zip crate doesn't expose the raw flag, so we check if name_raw is valid UTF-8.
        let flag: u16 = if std::str::from_utf8(file.name_raw()).is_ok() {
            1 << 11
        } else {
            0
        };
        let last_modified_time = file.last_modified().map_or(0, dos_datetime_to_long);

        // Read the entry data (decompressed)
        #[expect(clippy::cast_possible_truncation)]
        let mut data = Vec::with_capacity(uncompressed_size as usize);
        file.read_to_end(&mut data).map_err(|e| {
            ristretto_types::JavaError::IoException(format!(
                "Error reading zip entry data from '{path}': {e}"
            ))
        })?;

        entries.push(ZipEntryInfo {
            name_bytes,
            extra_bytes,
            comment_bytes,
            compressed_size,
            uncompressed_size,
            crc32,
            method,
            flag,
            last_modified_time,
            data,
        });
    }

    let context = ZipFileContext {
        entries,
        comment,
        starts_with_loc,
    };

    let state = get_zip_file_state(&thread)?;
    let handle = state.next_zip_id.fetch_add(1, Ordering::SeqCst);
    let mut zip_guard = state.zip_handles.write();
    zip_guard.insert(handle, context);

    Ok(Some(Value::Long(handle)))
}

#[intrinsic_method("java/util/zip/ZipFile.read(JJJ[BII)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn read<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let len = parameters.pop_int()?;
    let off = parameters.pop_int()?;
    let output_ref = parameters.pop_reference()?;
    let pos = parameters.pop_long()?;
    let entry_handle = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;

    let Some(output_ref) = output_ref else {
        return Err(ristretto_types::JavaError::NullPointerException(Some(
            "output array is null".to_string(),
        ))
        .into());
    };

    if len <= 0 {
        return Ok(Some(Value::Int(0)));
    }

    let state = get_zip_file_state(&thread)?;

    // Resolve entry handle
    let (zip_handle, index) = {
        let entry_guard = state.entry_handles.read();
        let Some(&(zh, idx)) = entry_guard.get(&entry_handle) else {
            return Err(ristretto_types::JavaError::RuntimeException(
                "Invalid entry handle".to_string(),
            )
            .into());
        };
        (zh, idx)
    };

    let zip_guard = state.zip_handles.read();
    let Some(context) = zip_guard.get(&zip_handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "ZipFile has been closed".to_string(),
        )
        .into());
    };
    let Some(entry) = context.entries.get(index) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "Invalid entry index".to_string(),
        )
        .into());
    };

    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let pos = pos as usize;
    #[expect(clippy::cast_sign_loss)]
    let off = off as usize;
    #[expect(clippy::cast_sign_loss)]
    let len = len as usize;

    // Check if we're past the end of the entry data
    if pos >= entry.data.len() {
        return Ok(Some(Value::Int(-1)));
    }

    // Calculate how many bytes we can read
    let available = entry.data.len() - pos;
    let to_read = len.min(available);

    // Copy data into the output array
    {
        let mut guard = output_ref.write();
        let output_bytes = guard.as_byte_vec_mut()?;
        if off + to_read > output_bytes.len() {
            return Err(ristretto_types::JavaError::ArrayIndexOutOfBoundsException {
                #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
                index: (off + to_read) as i32,
                length: output_bytes.len(),
            }
            .into());
        }
        for (i, &byte) in entry.data[pos..pos + to_read].iter().enumerate() {
            #[expect(clippy::cast_possible_wrap)]
            {
                output_bytes[off + i] = byte as i8;
            }
        }
    }

    #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    Ok(Some(Value::Int(to_read as i32)))
}

#[intrinsic_method("java/util/zip/ZipFile.startsWithLOC(J)Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn starts_with_loc<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;

    let state = get_zip_file_state(&thread)?;
    let zip_guard = state.zip_handles.read();
    let Some(context) = zip_guard.get(&handle) else {
        return Err(ristretto_types::JavaError::RuntimeException(
            "ZipFile has been closed".to_string(),
        )
        .into());
    };

    Ok(Some(Value::from(context.starts_with_loc)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;
    use ristretto_types::JavaObject;
    use std::io::Write;

    fn new_named_temp_file() -> tempfile::NamedTempFile {
        ristretto_test_util::init_wasi_tempdir();
        tempfile::NamedTempFile::new().expect("failed to create temp file")
    }

    /// Helper to create a test zip file with known content and return its path.
    fn create_test_zip() -> (tempfile::NamedTempFile, String) {
        let temp = new_named_temp_file();
        let path = temp.path().to_string_lossy().to_string();

        {
            let file = std::fs::File::create(&path).expect("failed to create file");
            let mut zip_writer = zip::ZipWriter::new(file);

            // Add a stored file
            let options = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored);
            zip_writer
                .start_file("hello.txt", options)
                .expect("failed to start file");
            zip_writer
                .write_all(b"Hello, World!")
                .expect("failed to write data");

            // Add a deflated file
            let options = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated);
            zip_writer
                .start_file("compressed.txt", options)
                .expect("failed to start file");
            zip_writer
                .write_all(b"This is compressed data for testing purposes.")
                .expect("failed to write data");

            // Add a directory entry
            let options = zip::write::SimpleFileOptions::default();
            zip_writer
                .add_directory("testdir/", options)
                .expect("failed to add directory");

            // Add a manifest
            let options = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored);
            zip_writer
                .start_file("META-INF/MANIFEST.MF", options)
                .expect("failed to start file");
            zip_writer
                .write_all(b"Manifest-Version: 1.0\r\n")
                .expect("failed to write data");

            zip_writer.finish().expect("failed to finish zip");
        }

        (temp, path)
    }

    /// Helper: create a Java String-like reference for a file path.
    async fn create_string_ref<T: Thread + 'static>(
        thread: &Arc<T>,
        s: &str,
    ) -> Result<Option<ristretto_gc::Gc<RwLock<Reference>>>> {
        let value = s.to_object(thread.as_ref()).await?;
        let Value::Object(gc_ref) = value else {
            return Err(ristretto_types::Error::InternalError(
                "Expected object value from to_object".to_string(),
            ));
        };
        Ok(gc_ref)
    }

    /// Helper: create a byte array reference (for entry name lookups).
    fn create_byte_array_ref(
        gc: &ristretto_gc::GarbageCollector,
        bytes: &[u8],
    ) -> ristretto_gc::Gc<RwLock<Reference>> {
        let signed_bytes: Vec<i8> = bytes.iter().map(|&b| b.cast_signed()).collect();
        let reference = Reference::from(signed_bytes);
        ristretto_gc::Gc::new(gc, RwLock::new(reference)).clone_gc()
    }

    /// Helper: open a zip file and return the handle.
    async fn open_zip<T: Thread + 'static>(thread: &Arc<T>, path: &str) -> Result<i64> {
        let string_ref = create_string_ref(thread, path).await?;
        let mut parameters = Parameters::default();
        parameters.push_reference(string_ref);
        parameters.push_int(1); // mode
        parameters.push_long(0); // last_modified
        parameters.push_int(0); // use_mmap

        let result = open(thread.clone(), parameters).await?;
        let handle = result.expect("expected handle").as_i64()?;
        assert!(handle > 0);
        Ok(handle)
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_open_and_close() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();

        let handle = open_zip(&thread, &path).await?;

        // Close
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        let result = close(thread, parameters).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_open_null_name() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_reference(None);
        parameters.push_int(1);
        parameters.push_long(0);
        parameters.push_int(0);

        let result = open(thread, parameters).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_open_nonexistent_file() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;

        let string_ref = create_string_ref(&thread, "/nonexistent/path/to/file.zip").await?;
        let mut parameters = Parameters::default();
        parameters.push_reference(string_ref);
        parameters.push_int(1);
        parameters.push_long(0);
        parameters.push_int(0);

        let result = open(thread, parameters).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_total() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        let result = get_total(thread.clone(), parameters).await?;
        // We created 4 entries: hello.txt, compressed.txt, testdir/, META-INF/MANIFEST.MF
        assert_eq!(Some(Value::Int(4)), result);

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_entry_and_metadata() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let vm = thread.vm()?;
        let gc = vm.garbage_collector();

        // Look up "hello.txt"
        let name_ref = Some(create_byte_array_ref(gc, b"hello.txt"));
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(name_ref);
        parameters.push_int(0); // add_slash = false

        let result = get_entry(thread.clone(), parameters).await?;
        let entry_handle = result.expect("expected entry handle").as_i64()?;
        assert!(entry_handle > 0, "entry handle should be > 0");

        // Test getEntrySize - "Hello, World!" = 13 bytes
        let mut parameters = Parameters::default();
        parameters.push_long(entry_handle);
        let result = get_entry_size(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Long(13)), result);

        // Test getEntryCSize stored, so compressed size == uncompressed size
        let mut parameters = Parameters::default();
        parameters.push_long(entry_handle);
        let result = get_entry_c_size(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Long(13)), result);

        // Test getEntryCrc
        let mut parameters = Parameters::default();
        parameters.push_long(entry_handle);
        let result = get_entry_crc(thread.clone(), parameters).await?;
        let crc_value = result.expect("expected CRC").as_i64()?;
        assert!(crc_value != 0, "CRC should not be 0 for non-empty content");

        // Test getEntryMethod; should be 0 (stored)
        let mut parameters = Parameters::default();
        parameters.push_long(entry_handle);
        let result = get_entry_method(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Int(0)), result);

        // Test getEntryFlag
        let mut parameters = Parameters::default();
        parameters.push_long(entry_handle);
        let result = get_entry_flag(thread.clone(), parameters).await?;
        let _flag = result.expect("expected flag").as_i32()?;

        // Test getEntryTime
        let mut parameters = Parameters::default();
        parameters.push_long(entry_handle);
        let result = get_entry_time(thread.clone(), parameters).await?;
        let _time = result.expect("expected time").as_i64()?;

        // Test getEntryBytes; type 0 = name
        let mut parameters = Parameters::default();
        parameters.push_long(entry_handle);
        parameters.push_int(0); // name
        let result = get_entry_bytes(thread.clone(), parameters).await?;
        let name_value = result.expect("expected name bytes");
        let name_bytes = name_value.as_byte_vec_ref()?;
        let name_u8: Vec<u8> = name_bytes.iter().map(|&b| b.cast_unsigned()).collect();
        let name = String::from_utf8_lossy(&name_u8);
        assert_eq!("hello.txt", name);

        // Free entry
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_long(entry_handle);
        free_entry(thread.clone(), parameters).await?;

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_entry_not_found() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let vm = thread.vm()?;
        let gc = vm.garbage_collector();

        let name_ref = Some(create_byte_array_ref(gc, b"nonexistent.txt"));
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(name_ref);
        parameters.push_int(0);

        let result = get_entry(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Long(0)), result);

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_entry_null_name() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(None);
        parameters.push_int(0);

        let result = get_entry(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Long(0)), result);

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_entry_with_add_slash() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let vm = thread.vm()?;
        let gc = vm.garbage_collector();

        // Look up "testdir" (without slash), with add_slash=true
        let name_ref = Some(create_byte_array_ref(gc, b"testdir"));
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(name_ref);
        parameters.push_int(1); // add_slash = true

        let result = get_entry(thread.clone(), parameters).await?;
        let entry_handle = result.expect("expected entry handle").as_i64()?;
        assert!(entry_handle > 0, "should find testdir/ with add_slash");

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_compressed_entry_metadata() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let vm = thread.vm()?;
        let gc = vm.garbage_collector();

        // Look up "compressed.txt"
        let name_ref = Some(create_byte_array_ref(gc, b"compressed.txt"));
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(name_ref);
        parameters.push_int(0);

        let result = get_entry(thread.clone(), parameters).await?;
        let entry_handle = result.expect("expected entry handle").as_i64()?;
        assert!(entry_handle > 0);

        // Check uncompressed size
        let mut parameters = Parameters::default();
        parameters.push_long(entry_handle);
        let result = get_entry_size(thread.clone(), parameters).await?;
        let size = result.expect("expected size").as_i64()?;
        assert_eq!(size, 45, "uncompressed size should be 45 bytes");

        // Check method is deflated (8)
        let mut parameters = Parameters::default();
        parameters.push_long(entry_handle);
        let result = get_entry_method(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Int(8)), result);

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_next_entry() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        // Get first entry (index 0)
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_int(0);
        let result = get_next_entry(thread.clone(), parameters).await?;
        let entry_handle = result.expect("expected entry handle").as_i64()?;
        assert!(entry_handle > 0);

        // Get entry at out-of-bounds index
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_int(100);
        let result = get_next_entry(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Long(0)), result);

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_manifest_num() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        let result = get_manifest_num(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Int(1)), result);

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_comment_bytes_empty() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        let result = get_comment_bytes(thread.clone(), parameters).await?;
        // Default zip has no comment
        assert_eq!(Some(Value::Object(None)), result);

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_zip_message() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1);
        let result = get_zip_message(thread, parameters).await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_read_entry_data() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let vm = thread.vm()?;
        let gc = vm.garbage_collector();

        // Look up "hello.txt"
        let name_ref = Some(create_byte_array_ref(gc, b"hello.txt"));
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(name_ref);
        parameters.push_int(0);

        let result = get_entry(thread.clone(), parameters).await?;
        let entry_handle = result.expect("expected entry handle").as_i64()?;

        // Create output buffer
        let output_bytes: Vec<i8> = vec![0i8; 20];
        let output_ref = Reference::from(output_bytes);
        let output_value = Value::new_object(gc, output_ref);
        let Value::Object(wrapped_output) = output_value else {
            panic!("expected object");
        };

        // Read from position 0
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_long(entry_handle);
        parameters.push_long(0); // pos
        parameters.push_reference(wrapped_output.clone());
        parameters.push_int(0); // off
        parameters.push_int(20); // len

        let result = read(thread.clone(), parameters).await?;
        let bytes_read = result.expect("expected bytes read").as_i32()?;
        assert_eq!(13, bytes_read); // "Hello, World!" is 13 bytes

        // Verify the data
        {
            let guard = wrapped_output.as_ref().expect("output ref").read();
            let bytes = guard.as_byte_vec_ref()?;
            let data: Vec<u8> = bytes[..13].iter().map(|&b| b.cast_unsigned()).collect();
            assert_eq!(b"Hello, World!", data.as_slice());
        }

        // Read at EOF
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_long(entry_handle);
        parameters.push_long(13); // pos = at end
        parameters.push_reference(wrapped_output);
        parameters.push_int(0);
        parameters.push_int(10);

        let result = read(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Int(-1)), result); // EOF

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_read_partial() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let vm = thread.vm()?;
        let gc = vm.garbage_collector();

        // Look up "hello.txt"
        let name_ref = Some(create_byte_array_ref(gc, b"hello.txt"));
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(name_ref);
        parameters.push_int(0);

        let result = get_entry(thread.clone(), parameters).await?;
        let entry_handle = result.expect("expected entry handle").as_i64()?;

        // Create output buffer large enough
        let output_bytes: Vec<i8> = vec![0i8; 5];
        let output_ref = Reference::from(output_bytes);
        let output_value = Value::new_object(gc, output_ref);
        let Value::Object(wrapped_output) = output_value else {
            panic!("expected object");
        };

        // Read 5 bytes starting from position 7 ("World")
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_long(entry_handle);
        parameters.push_long(7); // pos
        parameters.push_reference(wrapped_output.clone());
        parameters.push_int(0); // off
        parameters.push_int(5); // len

        let result = read(thread.clone(), parameters).await?;
        let bytes_read = result.expect("expected bytes read").as_i32()?;
        assert_eq!(5, bytes_read);

        {
            let guard = wrapped_output.as_ref().expect("output ref").read();
            let bytes = guard.as_byte_vec_ref()?;
            let data: Vec<u8> = bytes[..5].iter().map(|&b| b.cast_unsigned()).collect();
            assert_eq!(b"World", data.as_slice());
        }

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_read_null_output() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // handle
        parameters.push_long(1); // entry_handle
        parameters.push_long(0); // pos
        parameters.push_reference(None); // null output
        parameters.push_int(0); // off
        parameters.push_int(10); // len

        let result = read(thread, parameters).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_read_zero_length() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let vm = thread.vm()?;
        let gc = vm.garbage_collector();

        let name_ref = Some(create_byte_array_ref(gc, b"hello.txt"));
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(name_ref);
        parameters.push_int(0);

        let result = get_entry(thread.clone(), parameters).await?;
        let entry_handle = result.expect("expected entry handle").as_i64()?;

        let output_bytes: Vec<i8> = vec![0i8; 5];
        let output_ref = Reference::from(output_bytes);
        let output_value = Value::new_object(gc, output_ref);
        let Value::Object(wrapped_output) = output_value else {
            panic!("expected object");
        };

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_long(entry_handle);
        parameters.push_long(0);
        parameters.push_reference(wrapped_output);
        parameters.push_int(0);
        parameters.push_int(0); // len = 0

        let result = read(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Int(0)), result);

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_starts_with_loc() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        let result = starts_with_loc(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::from(true)), result);

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_free_entry() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let vm = thread.vm()?;
        let gc = vm.garbage_collector();

        // Get an entry
        let name_ref = Some(create_byte_array_ref(gc, b"hello.txt"));
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(name_ref);
        parameters.push_int(0);

        let result = get_entry(thread.clone(), parameters).await?;
        let entry_handle = result.expect("expected entry handle").as_i64()?;

        // Free it
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_long(entry_handle);
        let result = free_entry(thread.clone(), parameters).await?;
        assert!(result.is_none());

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_close_removes_entry_handles() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        // Create some entry handles
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_int(0);
        let result = get_next_entry(thread.clone(), parameters).await?;
        let entry_handle = result.expect("expected entry handle").as_i64()?;
        assert!(entry_handle > 0);

        // Close should also remove entry handles
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread.clone(), parameters).await?;

        // Verify the entry handle is gone
        let state = get_zip_file_state(&thread)?;
        let entry_guard = state.entry_handles.read();
        assert!(!entry_guard.contains_key(&entry_handle));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_entry_bytes_all_types() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let vm = thread.vm()?;
        let gc = vm.garbage_collector();

        // Look up an entry
        let name_ref = Some(create_byte_array_ref(gc, b"hello.txt"));
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(name_ref);
        parameters.push_int(0);

        let result = get_entry(thread.clone(), parameters).await?;
        let entry_handle = result.expect("expected entry handle").as_i64()?;

        // Type 0 = name
        let mut parameters = Parameters::default();
        parameters.push_long(entry_handle);
        parameters.push_int(0);
        let result = get_entry_bytes(thread.clone(), parameters).await?;
        assert!(result.expect("expected name").is_object());

        // Type 1 = extra (may be empty/null)
        let mut parameters = Parameters::default();
        parameters.push_long(entry_handle);
        parameters.push_int(1);
        let _result = get_entry_bytes(thread.clone(), parameters).await?;

        // Type 2 = comment (empty for our entries)
        let mut parameters = Parameters::default();
        parameters.push_long(entry_handle);
        parameters.push_int(2);
        let result = get_entry_bytes(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Object(None)), result);

        // Invalid type
        let mut parameters = Parameters::default();
        parameters.push_long(entry_handle);
        parameters.push_int(99);
        let result = get_entry_bytes(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Object(None)), result);

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_total_closed() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        // Close first
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread.clone(), parameters).await?;

        // Now try to get total; should error
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        let result = get_total(thread, parameters).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_read_compressed_entry() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let vm = thread.vm()?;
        let gc = vm.garbage_collector();

        // Look up "compressed.txt"
        let name_ref = Some(create_byte_array_ref(gc, b"compressed.txt"));
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_reference(name_ref);
        parameters.push_int(0);

        let result = get_entry(thread.clone(), parameters).await?;
        let entry_handle = result.expect("expected entry handle").as_i64()?;

        // Create output buffer large enough
        let output_bytes: Vec<i8> = vec![0i8; 100];
        let output_ref = Reference::from(output_bytes);
        let output_value = Value::new_object(gc, output_ref);
        let Value::Object(wrapped_output) = output_value else {
            panic!("expected object");
        };

        // Read from position 0
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        parameters.push_long(entry_handle);
        parameters.push_long(0);
        parameters.push_reference(wrapped_output.clone());
        parameters.push_int(0);
        parameters.push_int(100);

        let result = read(thread.clone(), parameters).await?;
        let bytes_read = result.expect("expected bytes read").as_i32()?;
        assert_eq!(45, bytes_read);

        {
            let guard = wrapped_output.as_ref().expect("output ref").read();
            let bytes = guard.as_byte_vec_ref()?;
            let data: Vec<u8> = bytes[..45].iter().map(|&b| b.cast_unsigned()).collect();
            assert_eq!(
                b"This is compressed data for testing purposes.",
                data.as_slice()
            );
        }

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_open_classes_jar() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;

        let cargo_manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let jar_path = cargo_manifest
            .join("..")
            .join("classes")
            .join("classes.jar");
        let jar_path = jar_path.to_string_lossy().to_string();

        let handle = open_zip(&thread, &jar_path).await?;

        // Verify we can get the total number of entries
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        let result = get_total(thread.clone(), parameters).await?;
        let total = result.expect("expected total").as_i32()?;
        assert!(total > 0, "classes.jar should have entries");

        // Cleanup
        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_comment_bytes_closed() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (_temp, path) = create_test_zip();
        let handle = open_zip(&thread, &path).await?;

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread.clone(), parameters).await?;

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        let result = get_comment_bytes(thread, parameters).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_manifest_num_no_manifest() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;

        // Create a zip without a manifest
        let temp = new_named_temp_file();
        let path = temp.path().to_string_lossy().to_string();
        {
            let file = std::fs::File::create(&path).expect("failed to create file");
            let mut zip_writer = zip::ZipWriter::new(file);
            let options = zip::write::SimpleFileOptions::default();
            zip_writer
                .start_file("test.txt", options)
                .expect("failed to start file");
            zip_writer.write_all(b"test").expect("failed to write data");
            zip_writer.finish().expect("failed to finish zip");
        }

        let handle = open_zip(&thread, &path).await?;

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        let result = get_manifest_num(thread.clone(), parameters).await?;
        assert_eq!(Some(Value::Int(0)), result);

        let mut parameters = Parameters::default();
        parameters.push_long(handle);
        close(thread, parameters).await?;
        Ok(())
    }
}

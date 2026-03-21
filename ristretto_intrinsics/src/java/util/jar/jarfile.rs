use crate::java::util::zip::zipfile::get_zip_file_state;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM as _};
use std::sync::Arc;

/// Implementation of `JarFile.getMetaInfEntryNames` for Java 8 and earlier.
///
/// Returns a `String[]` of all entry names in the ZIP file whose names start with
/// "META-INF/" (case-insensitive match on the prefix), or `null` if there are no
/// such entries.
#[intrinsic_method(
    "java/util/jar/JarFile.getMetaInfEntryNames()[Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_meta_inf_entry_names<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // Pop 'this' (JarFile instance) from parameters
    let this = parameters.pop()?;

    // Read the 'jzfile' field from the ZipFile/JarFile object.
    // Extract the value and drop the guard before any .await points.
    let jzfile = {
        let this_obj = this.as_object_ref()?;
        this_obj.value("jzfile")?.as_i64()?
    };

    if jzfile == 0 {
        return Ok(Some(Value::Object(None)));
    }

    // Access the zip state
    let state = get_zip_file_state(&thread)?;

    // Collect META-INF/ entry names while holding the read lock
    let meta_inf_names: Vec<String> = {
        let zip_guard = state.zip_handles.read();
        let Some(context) = zip_guard.get(&jzfile) else {
            return Ok(Some(Value::Object(None)));
        };

        context
            .entries
            .iter()
            .filter_map(|entry| {
                let name = String::from_utf8_lossy(&entry.name_bytes);
                if name.len() >= 9 && name[..9].eq_ignore_ascii_case("META-INF/") {
                    Some(name.into_owned())
                } else {
                    None
                }
            })
            .collect()
    };

    if meta_inf_names.is_empty() {
        return Ok(Some(Value::Object(None)));
    }

    // Create String objects for each entry name
    let mut string_values = Vec::with_capacity(meta_inf_names.len());
    for name in &meta_inf_names {
        let string_value = thread.intern_string(name).await?;
        string_values.push(string_value);
    }

    // Load the String array class and create the array
    let string_array_class = thread.class("[Ljava/lang/String;").await?;
    let reference = Reference::try_from((string_array_class, string_values))?;
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();
    Ok(Some(Value::new_object(gc, reference)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java::util::zip::zipfile::{ZipEntryInfo, ZipFileContext, get_zip_file_state};
    use crate::test::java8_thread;
    use ristretto_classloader::Object;
    use std::sync::atomic::Ordering;

    /// Create a minimal `ZipEntryInfo` with just a name.
    fn entry(name: &str) -> ZipEntryInfo {
        ZipEntryInfo {
            name_bytes: name.as_bytes().to_vec(),
            extra_bytes: Vec::new(),
            comment_bytes: Vec::new(),
            compressed_size: 0,
            uncompressed_size: 0,
            crc32: 0,
            method: 0,
            flag: 0,
            last_modified_time: 0,
            data: Vec::new(),
        }
    }

    /// Register a zip context in the shared state and return the handle.
    fn register_zip_context<T: Thread + 'static>(
        thread: &Arc<T>,
        context: ZipFileContext,
    ) -> Result<i64> {
        let state = get_zip_file_state(thread)?;
        let handle = state.next_zip_id.fetch_add(1, Ordering::SeqCst);
        state.zip_handles.write().insert(handle, context);
        Ok(handle)
    }

    /// Create a `ZipFile` object with the `jzfile` field set to `handle`.
    async fn create_jar_file_object<T: Thread + 'static>(
        thread: &Arc<T>,
        handle: i64,
    ) -> Result<Value> {
        let vm = thread.vm()?;
        let gc = vm.garbage_collector();
        let zip_class = thread.class("java/util/zip/ZipFile").await?;
        let mut object = Object::new(zip_class)?;
        object.set_value("jzfile", Value::Long(handle))?;
        let reference = Reference::from(object);
        Ok(Value::new_object(gc, reference))
    }

    #[tokio::test]
    async fn test_get_meta_inf_entry_names_empty_parameters() {
        let (_vm, thread) = java8_thread().await.expect("thread");
        let result = get_meta_inf_entry_names(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_meta_inf_entry_names_jzfile_zero() -> Result<()> {
        let (_vm, thread) = java8_thread().await?;
        let jar_value = create_jar_file_object(&thread, 0).await?;
        let parameters = Parameters::new(vec![jar_value]);
        let result = get_meta_inf_entry_names(thread, parameters).await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_meta_inf_entry_names_invalid_handle() -> Result<()> {
        let (_vm, thread) = java8_thread().await?;
        let jar_value = create_jar_file_object(&thread, 99999).await?;
        let parameters = Parameters::new(vec![jar_value]);
        let result = get_meta_inf_entry_names(thread, parameters).await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_meta_inf_entry_names_no_meta_inf() -> Result<()> {
        let (_vm, thread) = java8_thread().await?;

        let context = ZipFileContext {
            entries: vec![entry("hello.txt"), entry("world.txt")],
            comment: Vec::new(),
            starts_with_loc: true,
        };
        let handle = register_zip_context(&thread, context)?;
        let jar_value = create_jar_file_object(&thread, handle).await?;

        let parameters = Parameters::new(vec![jar_value]);
        let result = get_meta_inf_entry_names(thread, parameters).await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_meta_inf_entry_names_with_meta_inf() -> Result<()> {
        let (_vm, thread) = java8_thread().await?;

        let context = ZipFileContext {
            entries: vec![
                entry("hello.txt"),
                entry("META-INF/MANIFEST.MF"),
                entry("META-INF/services/com.example.Service"),
                entry("com/example/Main.class"),
            ],
            comment: Vec::new(),
            starts_with_loc: true,
        };
        let handle = register_zip_context(&thread, context)?;
        let jar_value = create_jar_file_object(&thread, handle).await?;

        let parameters = Parameters::new(vec![jar_value]);
        let result = get_meta_inf_entry_names(thread, parameters).await?;

        // Should return a String array with 2 entries
        let result = result.expect("expected a value");
        let (class, elements) = result.as_class_vec_ref()?;
        assert_eq!("[Ljava/lang/String;", class.name());
        assert_eq!(2, elements.len());

        let name0 = elements[0].as_string()?;
        let name1 = elements[1].as_string()?;
        assert_eq!("META-INF/MANIFEST.MF", name0);
        assert_eq!("META-INF/services/com.example.Service", name1);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_meta_inf_entry_names_case_insensitive() -> Result<()> {
        let (_vm, thread) = java8_thread().await?;

        let context = ZipFileContext {
            entries: vec![
                entry("meta-inf/MANIFEST.MF"),
                entry("Meta-Inf/signatures/sig.sf"),
            ],
            comment: Vec::new(),
            starts_with_loc: true,
        };
        let handle = register_zip_context(&thread, context)?;
        let jar_value = create_jar_file_object(&thread, handle).await?;

        let parameters = Parameters::new(vec![jar_value]);
        let result = get_meta_inf_entry_names(thread, parameters).await?;

        let result = result.expect("expected a value");
        let (_class, elements) = result.as_class_vec_ref()?;
        assert_eq!(2, elements.len());

        let name0 = elements[0].as_string()?;
        let name1 = elements[1].as_string()?;
        assert_eq!("meta-inf/MANIFEST.MF", name0);
        assert_eq!("Meta-Inf/signatures/sig.sf", name1);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_meta_inf_entry_names_only_prefix_match() -> Result<()> {
        let (_vm, thread) = java8_thread().await?;

        // "META-INF" without trailing '/' should NOT match
        let context = ZipFileContext {
            entries: vec![entry("META-INF"), entry("META-INFORMATION/data.txt")],
            comment: Vec::new(),
            starts_with_loc: true,
        };
        let handle = register_zip_context(&thread, context)?;
        let jar_value = create_jar_file_object(&thread, handle).await?;

        let parameters = Parameters::new(vec![jar_value]);
        let result = get_meta_inf_entry_names(thread, parameters).await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_meta_inf_entry_names_null_this() {
        let (_vm, thread) = java8_thread().await.expect("thread");
        let parameters = Parameters::new(vec![Value::Object(None)]);
        let result = get_meta_inf_entry_names(thread, parameters).await;
        assert!(result.is_err());
    }
}

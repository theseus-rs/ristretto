use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaObject;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/fs/UTIFileTypeDetector.probe0(Ljava/lang/String;)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn probe_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path_value = parameters.pop()?;
    let path_str = path_value.as_string()?;

    let mime_type = mime_type_for_extension(&path_str);
    match mime_type {
        Some(mime) => {
            let result = mime.to_object(&*thread).await?;
            Ok(Some(result))
        }
        None => Ok(Some(Value::Object(None))),
    }
}

fn mime_type_for_extension(path: &str) -> Option<&'static str> {
    let ext = match path.rfind('.') {
        Some(pos) => &path[pos + 1..],
        None => return None,
    };
    if ext.is_empty() {
        return None;
    }
    let ext_lower = ext.to_ascii_lowercase();
    match ext_lower.as_str() {
        "txt" | "text" => Some("text/plain"),
        "html" | "htm" => Some("text/html"),
        "css" => Some("text/css"),
        "js" => Some("text/javascript"),
        "json" => Some("application/json"),
        "xml" => Some("text/xml"),
        "csv" => Some("text/csv"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "png" => Some("image/png"),
        "gif" => Some("image/gif"),
        "bmp" => Some("image/bmp"),
        "svg" => Some("image/svg+xml"),
        "ico" => Some("image/x-icon"),
        "webp" => Some("image/webp"),
        "tiff" | "tif" => Some("image/tiff"),
        "pdf" => Some("application/pdf"),
        "zip" => Some("application/zip"),
        "gz" | "gzip" => Some("application/gzip"),
        "tar" => Some("application/x-tar"),
        "jar" | "war" => Some("application/java-archive"),
        "class" => Some("application/java-vm"),
        "java" => Some("text/x-java-source"),
        "properties" => Some("text/x-java-properties"),
        "mp3" => Some("audio/mpeg"),
        "wav" => Some("audio/wav"),
        "mp4" => Some("video/mp4"),
        "avi" => Some("video/x-msvideo"),
        "mov" => Some("video/quicktime"),
        "doc" => Some("application/msword"),
        "xls" => Some("application/vnd.ms-excel"),
        "ppt" => Some("application/vnd.ms-powerpoint"),
        "docx" => Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
        "xlsx" => Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
        "pptx" => Some("application/vnd.openxmlformats-officedocument.presentationml.presentation"),
        "wasm" => Some("application/wasm"),
        "rs" => Some("text/x-rust"),
        "c" | "h" => Some("text/x-c"),
        "cpp" | "cc" | "cxx" => Some("text/x-c++src"),
        "py" => Some("text/x-python"),
        "rb" => Some("text/x-ruby"),
        "sh" => Some("application/x-sh"),
        "yaml" | "yml" => Some("text/yaml"),
        "toml" => Some("application/toml"),
        "md" => Some("text/markdown"),
        "rtf" => Some("application/rtf"),
        "woff" => Some("font/woff"),
        "woff2" => Some("font/woff2"),
        "ttf" => Some("font/ttf"),
        "otf" => Some("font/otf"),
        "eot" => Some("application/vnd.ms-fontobject"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::JavaObject;

    #[test]
    fn test_mime_type_for_extension() {
        assert_eq!(mime_type_for_extension("test.txt"), Some("text/plain"));
        assert_eq!(mime_type_for_extension("test.html"), Some("text/html"));
        assert_eq!(
            mime_type_for_extension("test.json"),
            Some("application/json")
        );
        assert_eq!(mime_type_for_extension("test.jpg"), Some("image/jpeg"));
        assert_eq!(mime_type_for_extension("test.png"), Some("image/png"));
        assert_eq!(
            mime_type_for_extension("test.jar"),
            Some("application/java-archive")
        );
        assert_eq!(
            mime_type_for_extension("test.class"),
            Some("application/java-vm")
        );
        assert_eq!(
            mime_type_for_extension("test.java"),
            Some("text/x-java-source")
        );
        assert_eq!(mime_type_for_extension("test.unknown_ext"), None);
        assert_eq!(mime_type_for_extension("noext"), None);
    }

    #[test]
    fn test_mime_type_for_extension_case_insensitive() {
        assert_eq!(mime_type_for_extension("test.TXT"), Some("text/plain"));
        assert_eq!(mime_type_for_extension("test.HTML"), Some("text/html"));
        assert_eq!(mime_type_for_extension("test.Jpg"), Some("image/jpeg"));
    }

    #[test]
    fn test_mime_type_for_extension_additional() {
        assert_eq!(mime_type_for_extension("file.css"), Some("text/css"));
        assert_eq!(mime_type_for_extension("file.js"), Some("text/javascript"));
        assert_eq!(mime_type_for_extension("file.pdf"), Some("application/pdf"));
        assert_eq!(mime_type_for_extension("file.zip"), Some("application/zip"));
        assert_eq!(mime_type_for_extension("file.mp4"), Some("video/mp4"));
        assert_eq!(mime_type_for_extension("file.rs"), Some("text/x-rust"));
        assert_eq!(mime_type_for_extension("file.md"), Some("text/markdown"));
        assert_eq!(
            mime_type_for_extension("file.wasm"),
            Some("application/wasm")
        );
    }

    #[test]
    fn test_mime_type_for_extension_no_extension() {
        assert_eq!(mime_type_for_extension(""), None);
        assert_eq!(mime_type_for_extension("noext"), None);
        assert_eq!(mime_type_for_extension("."), None);
    }

    #[tokio::test]
    async fn test_probe_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = probe_0(thread, Parameters::default()).await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::ParametersUnderflow)
        ));
    }

    #[tokio::test]
    async fn test_probe_0_known_extension() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let file_name = "test.txt".to_object(&*thread).await.expect("to_object");
        let mut params = Parameters::default();
        params.push(file_name);
        let result = probe_0(thread, params).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_some());
    }

    #[tokio::test]
    async fn test_probe_0_unknown_extension() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let file_name = "test.zzzzz".to_object(&*thread).await.expect("to_object");
        let mut params = Parameters::default();
        params.push(file_name);
        let result = probe_0(thread, params).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        // Unknown extension returns null
        assert_eq!(value, Some(Value::Object(None)));
    }
}

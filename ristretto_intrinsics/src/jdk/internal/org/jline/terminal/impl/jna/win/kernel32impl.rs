use ristretto_classfile::VersionSpecification::Between;
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.FillConsoleOutputAttribute(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;SILjdk/internal/org/jline/terminal/impl/jna/win/Kernel32$COORD;Ljdk/internal/org/jline/terminal/impl/jna/win/IntByReference;)V",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn fill_console_output_attribute<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out_lp_number_of_attrs_written = parameters.pop_reference()?;
    let _in_dw_write_coord = parameters.pop_reference()?;
    let _in_n_length = parameters.pop_int()?;
    let _in_w_attribute = parameters.pop_int()?;
    let _in_h_console_output = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.FillConsoleOutputAttribute(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;SILjdk/internal/org/jline/terminal/impl/jna/win/Kernel32$COORD;Ljdk/internal/org/jline/terminal/impl/jna/win/IntByReference;)V".to_string()).into())
}
#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.FillConsoleOutputCharacter(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;CILjdk/internal/org/jline/terminal/impl/jna/win/Kernel32$COORD;Ljdk/internal/org/jline/terminal/impl/jna/win/IntByReference;)V",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn fill_console_output_character<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out_lp_number_of_chars_written = parameters.pop_reference()?;
    let _in_dw_write_coord = parameters.pop_reference()?;
    let _in_n_length = parameters.pop_int()?;
    let _in_c_character = parameters.pop_int()?;
    let _in_h_console_output = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.FillConsoleOutputCharacter(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;CILjdk/internal/org/jline/terminal/impl/jna/win/Kernel32$COORD;Ljdk/internal/org/jline/terminal/impl/jna/win/IntByReference;)V".to_string()).into())
}
#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.GetConsoleMode(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;Ljdk/internal/org/jline/terminal/impl/jna/win/IntByReference;)V",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn get_console_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out_lp_mode = parameters.pop_reference()?;
    let _in_h_console_output = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.GetConsoleMode(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;Ljdk/internal/org/jline/terminal/impl/jna/win/IntByReference;)V".to_string()).into())
}
#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.GetConsoleScreenBufferInfo(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$CONSOLE_SCREEN_BUFFER_INFO;)V",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn get_console_screen_buffer_info<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out_lp_console_screen_buffer_info = parameters.pop_reference()?;
    let _in_h_console_output = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.GetConsoleScreenBufferInfo(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$CONSOLE_SCREEN_BUFFER_INFO;)V".to_string()).into())
}
#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.GetStdHandle(I)Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn get_std_handle<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _n_std_handle = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.GetStdHandle(I)Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;".to_string()).into())
}
#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.ReadConsoleInput(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;[Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$INPUT_RECORD;ILjdk/internal/org/jline/terminal/impl/jna/win/IntByReference;)V",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn read_console_input<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _out_lp_number_of_events_read = parameters.pop_reference()?;
    let _in_n_length = parameters.pop_int()?;
    let _out_lp_buffer = parameters.pop_reference()?;
    let _in_h_console_output = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.ReadConsoleInput(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;[Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$INPUT_RECORD;ILjdk/internal/org/jline/terminal/impl/jna/win/IntByReference;)V".to_string()).into())
}
#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.ScrollConsoleScreenBuffer(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$SMALL_RECT;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$SMALL_RECT;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$COORD;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$CHAR_INFO;)V",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn scroll_console_screen_buffer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _in_lp_fill = parameters.pop_reference()?;
    let _in_dw_destination_origin = parameters.pop_reference()?;
    let _in_lp_clip_rectangle = parameters.pop_reference()?;
    let _in_lp_scroll_rectangle = parameters.pop_reference()?;
    let _in_h_console_output = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.ScrollConsoleScreenBuffer(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$SMALL_RECT;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$SMALL_RECT;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$COORD;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$CHAR_INFO;)V".to_string()).into())
}
#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.SetConsoleCursorPosition(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$COORD;)V",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn set_console_cursor_position<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _in_dw_cursor_position = parameters.pop_reference()?;
    let _in_h_console_output = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.SetConsoleCursorPosition(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$COORD;)V".to_string()).into())
}
#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.SetConsoleMode(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;I)V",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn set_console_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _in_dw_mode = parameters.pop_int()?;
    let _in_h_console_output = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.SetConsoleMode(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;I)V".to_string()).into())
}
#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.SetConsoleTextAttribute(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;S)V",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn set_console_text_attribute<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _in_w_attributes = parameters.pop_int()?;
    let _in_h_console_output = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.SetConsoleTextAttribute(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;S)V".to_string()).into())
}
#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.SetConsoleTitle(Ljava/lang/String;)V",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn set_console_title<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _in_lp_console_title = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.SetConsoleTitle(Ljava/lang/String;)V".to_string()).into())
}
#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.WaitForSingleObject(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;I)I",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn wait_for_single_object<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _in_dw_milliseconds = parameters.pop_int()?;
    let _in_h_handle = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.WaitForSingleObject(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;I)I".to_string()).into())
}
#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.WriteConsoleW(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;[CILjdk/internal/org/jline/terminal/impl/jna/win/IntByReference;Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;)V",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn write_console_w<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg4 = parameters.pop_reference()?;
    let _arg3 = parameters.pop_reference()?;
    let _arg2 = parameters.pop_int()?;
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.WriteConsoleW(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;[CILjdk/internal/org/jline/terminal/impl/jna/win/IntByReference;Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;)V".to_string()).into())
}
#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.initIDs()V",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.initIDs()V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_fill_console_output_attribute() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fill_console_output_attribute(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.FillConsoleOutputAttribute(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;SILjdk/internal/org/jline/terminal/impl/jna/win/Kernel32$COORD;Ljdk/internal/org/jline/terminal/impl/jna/win/IntByReference;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_fill_console_output_character() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fill_console_output_character(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.FillConsoleOutputCharacter(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;CILjdk/internal/org/jline/terminal/impl/jna/win/Kernel32$COORD;Ljdk/internal/org/jline/terminal/impl/jna/win/IntByReference;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_console_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_console_mode(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.GetConsoleMode(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;Ljdk/internal/org/jline/terminal/impl/jna/win/IntByReference;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_console_screen_buffer_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_console_screen_buffer_info(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.GetConsoleScreenBufferInfo(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$CONSOLE_SCREEN_BUFFER_INFO;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_std_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_std_handle(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.GetStdHandle(I)Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_read_console_input() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_console_input(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.ReadConsoleInput(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;[Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$INPUT_RECORD;ILjdk/internal/org/jline/terminal/impl/jna/win/IntByReference;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_scroll_console_screen_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = scroll_console_screen_buffer(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.ScrollConsoleScreenBuffer(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$SMALL_RECT;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$SMALL_RECT;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$COORD;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$CHAR_INFO;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_console_cursor_position() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_console_cursor_position(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.SetConsoleCursorPosition(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;Ljdk/internal/org/jline/terminal/impl/jna/win/Kernel32$COORD;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_console_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_console_mode(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.SetConsoleMode(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_console_text_attribute() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_console_text_attribute(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.SetConsoleTextAttribute(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;S)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_console_title() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_console_title(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.SetConsoleTitle(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_wait_for_single_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = wait_for_single_object(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.WaitForSingleObject(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_write_console_w() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write_console_w(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.WriteConsoleW(Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;[CILjdk/internal/org/jline/terminal/impl/jna/win/IntByReference;Ljdk/internal/org/jline/terminal/impl/jna/win/Pointer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "jdk/internal/org/jline/terminal/impl/jna/win/Kernel32Impl.initIDs()V",
            result.unwrap_err().to_string()
        );
    }
}

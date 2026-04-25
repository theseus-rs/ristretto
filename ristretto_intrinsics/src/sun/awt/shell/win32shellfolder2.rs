use ristretto_classfile::VersionSpecification::{Any, Equal, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.bindToObject(JJ)J", Any)]
#[async_method]
pub async fn bind_to_object<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _relative_pidl = parameters.pop_long()?;
    let _parent_i_shell_folder = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.bindToObject(JJ)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.combinePIDLs(JJ)J", Any)]
#[async_method]
pub async fn combine_pidls<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jp_idl = parameters.pop_long()?;
    let _jpp_idl = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.combinePIDLs(JJ)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.compareIDs(JJJ)I", Any)]
#[async_method]
pub async fn compare_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_idl2 = parameters.pop_long()?;
    let _p_idl1 = parameters.pop_long()?;
    let _jp_parent_i_shell_folder = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.compareIDs(JJJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.compareIDsByColumn(JJJI)I", Any)]
#[async_method]
pub async fn compare_ids_by_column<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _column_idx = parameters.pop_int()?;
    let _p_idl2 = parameters.pop_long()?;
    let _p_idl1 = parameters.pop_long()?;
    let _jp_parent_i_shell_folder = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.compareIDsByColumn(JJJI)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.copyFirstPIDLEntry(J)J", Any)]
#[async_method]
pub async fn copy_first_pidlentry<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jp_idl = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.copyFirstPIDLEntry(J)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.disposeIcon(J)V", Any)]
#[async_method]
pub async fn dispose_icon<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _hicon = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.disposeIcon(J)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/shell/Win32ShellFolder2.doGetColumnInfo(J)[Lsun/awt/shell/ShellFolderColumnInfo;",
    Any
)]
#[async_method]
pub async fn do_get_column_info<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _i_shell_folder = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.doGetColumnInfo(J)[Lsun/awt/shell/ShellFolderColumnInfo;"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/shell/Win32ShellFolder2.doGetColumnValue(JJI)Ljava/lang/Object;",
    Any
)]
#[async_method]
pub async fn do_get_column_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _column_idx = parameters.pop_int()?;
    let _jpidl = parameters.pop_long()?;
    let _i_shell_folder = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.doGetColumnValue(JJI)Ljava/lang/Object;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/shell/Win32ShellFolder2.extractIcon(JJIZ)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn extract_icon<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _get_default_icon = parameters.pop_bool()?;
    let _size = parameters.pop_int()?;
    let _relative_pidl = parameters.pop_long()?;
    let _p_i_shell_folder_l = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.extractIcon(JJIZ)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.extractIcon(JJZ)J", Equal(JAVA_8))]
#[async_method]
pub async fn extract_icon_windows_v8<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_bool()?;
    let _arg1 = parameters.pop_long()?;
    let _arg0 = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.extractIcon(JJZ)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.extractIcon(JJZZ)J", Equal(JAVA_11))]
#[async_method]
pub async fn extract_icon_windows_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _get_default_icon = parameters.pop_bool()?;
    let _size = parameters.pop_bool()?;
    let _relative_pidl = parameters.pop_long()?;
    let _p_i_shell_folder_l = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.extractIcon(JJZZ)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.getAttributes0(JJI)I", Any)]
#[async_method]
pub async fn get_attributes0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _attrs_mask = parameters.pop_int()?;
    let _jp_idl = parameters.pop_long()?;
    let _jp_parent_i_shell_folder = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getAttributes0(JJI)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/shell/Win32ShellFolder2.getDisplayNameOf(JJI)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_display_name_of<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _attrs = parameters.pop_int()?;
    let _relative_pidl = parameters.pop_long()?;
    let _parent_i_shell_folder = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getDisplayNameOf(JJI)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.getEnumObjects(JZZ)J", Any)]
#[async_method]
pub async fn get_enum_objects<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _include_hidden_files = parameters.pop_bool()?;
    let _is_desktop = parameters.pop_bool()?;
    let _p_i_shell_folder = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getEnumObjects(JZZ)J".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/shell/Win32ShellFolder2.getExecutableType(Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_executable_type<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _path = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getExecutableType(Ljava/lang/String;)Ljava/lang/String;"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/shell/Win32ShellFolder2.getFileSystemPath0(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_file_system_path0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _csidl = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getFileSystemPath0(I)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/shell/Win32ShellFolder2.getFolderType(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_folder_type<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_idl = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getFolderType(J)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.getIShellIcon(J)J", Any)]
#[async_method]
pub async fn get_ishell_icon<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _parent_i_shell_folder = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getIShellIcon(J)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.getIcon(Ljava/lang/String;Z)J", Any)]
#[async_method]
pub async fn get_icon<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _get_large_icon = parameters.pop_bool()?;
    let _absolute_path = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getIcon(Ljava/lang/String;Z)J".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/shell/Win32ShellFolder2.getIconBits(J)[I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_icon_bits<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _hicon = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getIconBits(J)[I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.getIconBits(JI)[I", Equal(JAVA_8))]
#[async_method]
pub async fn get_icon_bits_windows_v8<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getIconBits(JI)[I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.getIconIndex(JJ)I", Any)]
#[async_method]
pub async fn get_icon_index<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _relative_pidl = parameters.pop_long()?;
    let _p_i_shell_icon_l = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getIconIndex(JJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/shell/Win32ShellFolder2.getIconResource(Ljava/lang/String;III)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_icon_resource<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cy_desired = parameters.pop_int()?;
    let _cx_desired = parameters.pop_int()?;
    let _icon_id = parameters.pop_int()?;
    let _lib_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getIconResource(Ljava/lang/String;III)J".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/shell/Win32ShellFolder2.getIconResource(Ljava/lang/String;IIIZ)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_icon_resource_windows_le_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _use_vga_colors = parameters.pop_bool()?;
    let _cy_desired = parameters.pop_int()?;
    let _cx_desired = parameters.pop_int()?;
    let _icon_id = parameters.pop_int()?;
    let _lib_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getIconResource(Ljava/lang/String;IIIZ)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.getLinkLocation(JJZ)J", Any)]
#[async_method]
pub async fn get_link_location<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _resolve = parameters.pop_bool()?;
    let _relative_pidl = parameters.pop_long()?;
    let _parent_i_shell_folder = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getLinkLocation(JJZ)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.getNextChild(J)J", Any)]
#[async_method]
pub async fn get_next_child<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_enum_objects = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getNextChild(J)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.getNextPIDLEntry(J)J", Any)]
#[async_method]
pub async fn get_next_pidlentry<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jp_idl = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getNextPIDLEntry(J)J".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/shell/Win32ShellFolder2.getStandardViewButton0(I)[I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn get_standard_view_button0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getStandardViewButton0(I)[I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/shell/Win32ShellFolder2.getStandardViewButton0(IZ)[I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_standard_view_button0_windows_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _small_icon = parameters.pop_bool()?;
    let _icon_index = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getStandardViewButton0(IZ)[I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.getSystemIcon(I)J", Any)]
#[async_method]
pub async fn get_system_icon<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _icon_id = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.getSystemIcon(I)J".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/shell/Win32ShellFolder2.hiResIconAvailable(JJ)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn hi_res_icon_available<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _relative_pidl = parameters.pop_long()?;
    let _p_i_shell_folder_l = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.hiResIconAvailable(JJ)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.initDesktop()V", Any)]
#[async_method]
pub async fn init_desktop<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.initDesktop()V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/shell/Win32ShellFolder2.initIDs()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.initSpecial(JI)V", Any)]
#[async_method]
pub async fn init_special<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _folder_type = parameters.pop_int()?;
    let _desktop_i_shell_folder = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.initSpecial(JI)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/shell/Win32ShellFolder2.loadKnownFolders()[Lsun/awt/shell/Win32ShellFolder2$KnownFolderDefinition;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn load_known_folders<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/shell/Win32ShellFolder2.loadKnownFolders()[Lsun/awt/shell/Win32ShellFolder2$KnownFolderDefinition;".to_string()).into())
}
#[intrinsic_method(
    "sun/awt/shell/Win32ShellFolder2.parseDisplayName0(JLjava/lang/String;)J",
    Any
)]
#[async_method]
pub async fn parse_display_name0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jname = parameters.pop_reference()?;
    let _jp_i_shell_folder = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.parseDisplayName0(JLjava/lang/String;)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.releaseEnumObjects(J)V", Any)]
#[async_method]
pub async fn release_enum_objects<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_enum_objects = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.releaseEnumObjects(J)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.releaseIShellFolder(J)V", Any)]
#[async_method]
pub async fn release_ishell_folder<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_i_shell_folder = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.releaseIShellFolder(J)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/shell/Win32ShellFolder2.releasePIDL(J)V", Any)]
#[async_method]
pub async fn release_pidl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_idl = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/shell/Win32ShellFolder2.releasePIDL(J)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_bind_to_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = bind_to_object(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.bindToObject(JJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_combine_pidls() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = combine_pidls(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.combinePIDLs(JJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_compare_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = compare_ids(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.compareIDs(JJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_compare_ids_by_column() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = compare_ids_by_column(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.compareIDsByColumn(JJJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_copy_first_pidlentry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = copy_first_pidlentry(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.copyFirstPIDLEntry(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_dispose_icon() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose_icon(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.disposeIcon(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_do_get_column_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_get_column_info(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.doGetColumnInfo(J)[Lsun/awt/shell/ShellFolderColumnInfo;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_do_get_column_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_get_column_value(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.doGetColumnValue(JJI)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_extract_icon() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = extract_icon(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.extractIcon(JJIZ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_extract_icon_windows_v8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = extract_icon_windows_v8(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.extractIcon(JJZ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_extract_icon_windows_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = extract_icon_windows_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::from(false),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.extractIcon(JJZZ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_attributes0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_attributes0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getAttributes0(JJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_display_name_of() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_display_name_of(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getDisplayNameOf(JJI)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_enum_objects() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_enum_objects(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getEnumObjects(JZZ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_executable_type() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_executable_type(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getExecutableType(Ljava/lang/String;)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_file_system_path0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_system_path0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getFileSystemPath0(I)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_folder_type() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_folder_type(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getFolderType(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_ishell_icon() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_ishell_icon(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getIShellIcon(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_icon() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_icon(
            thread,
            Parameters::new(vec![Value::Object(None), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getIcon(Ljava/lang/String;Z)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_icon_bits() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_icon_bits(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getIconBits(J)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_icon_bits_windows_v8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_icon_bits_windows_v8(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getIconBits(JI)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_icon_index() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_icon_index(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getIconIndex(JJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_icon_resource() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_icon_resource(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getIconResource(Ljava/lang/String;III)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_icon_resource_windows_le_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_icon_resource_windows_le_v11(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getIconResource(Ljava/lang/String;IIIZ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_link_location() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_link_location(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getLinkLocation(JJZ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_next_child() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_next_child(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getNextChild(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_next_pidlentry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_next_pidlentry(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getNextPIDLEntry(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_standard_view_button0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_standard_view_button0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getStandardViewButton0(I)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_standard_view_button0_windows_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_standard_view_button0_windows_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getStandardViewButton0(IZ)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_system_icon() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_system_icon(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.getSystemIcon(I)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_hi_res_icon_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = hi_res_icon_available(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.hiResIconAvailable(JJ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_desktop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_desktop(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.initDesktop()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_special() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            init_special(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.initSpecial(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_load_known_folders() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_known_folders(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.loadKnownFolders()[Lsun/awt/shell/Win32ShellFolder2$KnownFolderDefinition;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_parse_display_name0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = parse_display_name0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.parseDisplayName0(JLjava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_release_enum_objects() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = release_enum_objects(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.releaseEnumObjects(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_release_ishell_folder() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = release_ishell_folder(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.releaseIShellFolder(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_release_pidl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = release_pidl(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/shell/Win32ShellFolder2.releasePIDL(J)V",
            result.unwrap_err().to_string()
        );
    }
}

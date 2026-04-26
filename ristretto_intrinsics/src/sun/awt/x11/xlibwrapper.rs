use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/X11/XlibWrapper.CallErrorHandler(JJJ)I", Any)]
#[async_method]
pub async fn call_error_handler<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _event_ptr = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    let _handler = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.CallErrorHandler(JJJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.DefaultScreen(J)J", Any)]
#[async_method]
pub async fn default_screen<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.DefaultScreen(J)J".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.DisplayHeight(JJ)J", Any)]
#[async_method]
pub async fn display_height<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.DisplayHeight(JJ)J".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.DisplayHeightMM(JJ)J", Any)]
#[async_method]
pub async fn display_height_mm<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.DisplayHeightMM(JJ)J".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.DisplayWidth(JJ)J", Any)]
#[async_method]
pub async fn display_width<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.DisplayWidth(JJ)J".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.DisplayWidthMM(JJ)J", Any)]
#[async_method]
pub async fn display_width_mm<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.DisplayWidthMM(JJ)J".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.DoesBackingStore(J)I", Any)]
#[async_method]
pub async fn does_backing_store<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.DoesBackingStore(J)I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.ExitSecondaryLoop()V", Any)]
#[async_method]
pub async fn exit_secondary_loop<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.ExitSecondaryLoop()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.GetProperty(JJJ)Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_property<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _atom = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.GetProperty(JJJ)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.InternAtom(JLjava/lang/String;I)J", Any)]
#[async_method]
pub async fn intern_atom<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ife = parameters.pop_int()?;
    let _jstr = parameters.pop_reference()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.InternAtom(JLjava/lang/String;I)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.IsKanaKeyboard(J)Z", Any)]
#[async_method]
pub async fn is_kana_keyboard<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.IsKanaKeyboard(J)Z".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.IsKeypadKey(J)Z", Any)]
#[async_method]
pub async fn is_keypad_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _keysym = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.IsKeypadKey(J)Z".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.IsSunKeyboard(J)Z", Any)]
#[async_method]
pub async fn is_sun_keyboard<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.IsSunKeyboard(J)Z".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.IsXsunKPBehavior(J)Z", Any)]
#[async_method]
pub async fn is_xsun_kpbehavior<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.IsXsunKPBehavior(J)Z".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.PrintXErrorEvent(JJ)V", Any)]
#[async_method]
pub async fn print_xerror_event<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _event_ptr = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.PrintXErrorEvent(JJ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.RootWindow(JJ)J", Any)]
#[async_method]
pub async fn root_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen_number = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.RootWindow(JJ)J".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.ScreenCount(J)I", Any)]
#[async_method]
pub async fn screen_count<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.ScreenCount(J)I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.ScreenOfDisplay(JJ)J", Any)]
#[async_method]
pub async fn screen_of_display<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen_number = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.ScreenOfDisplay(JJ)J".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.ServerVendor(J)Ljava/lang/String;", Any)]
#[async_method]
pub async fn server_vendor<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.ServerVendor(J)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.SetBitmapShape(JJII[I)V", Any)]
#[async_method]
pub async fn set_bitmap_shape<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _bitmap = parameters.pop_reference()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.SetBitmapShape(JJII[I)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.SetProperty(JJJLjava/lang/String;)V", Any)]
#[async_method]
pub async fn set_property<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jstr = parameters.pop_reference()?;
    let _atom = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.SetProperty(JJJLjava/lang/String;)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/X11/XlibWrapper.SetRectangularShape(JJIIIILsun/java2d/pipe/Region;)V",
    Any
)]
#[async_method]
pub async fn set_rectangular_shape<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _region = parameters.pop_reference()?;
    let _hiy = parameters.pop_int()?;
    let _hix = parameters.pop_int()?;
    let _loy = parameters.pop_int()?;
    let _lox = parameters.pop_int()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.SetRectangularShape(JJIIIILsun/java2d/pipe/Region;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.SetToolkitErrorHandler()J", Any)]
#[async_method]
pub async fn set_toolkit_error_handler<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.SetToolkitErrorHandler()J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.SetZOrder(JJJ)V", Any)]
#[async_method]
pub async fn set_zorder<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _above = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.SetZOrder(JJJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.VendorRelease(J)I", Any)]
#[async_method]
pub async fn vendor_release<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.VendorRelease(J)I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XAllocColor(JJJ)Z", Any)]
#[async_method]
pub async fn xalloc_color<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen_in_out = parameters.pop_long()?;
    let _colormap = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XAllocColor(JJJ)Z".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XAllocSizeHints()J", Any)]
#[async_method]
pub async fn xalloc_size_hints<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XAllocSizeHints()J".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XAllocWMHints()J", Any)]
#[async_method]
pub async fn xalloc_wmhints<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XAllocWMHints()J".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XBell(JI)V", Any)]
#[async_method]
pub async fn xbell<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _percent = parameters.pop_int()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XBell(JI)V".to_string()).into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XChangeActivePointerGrab(JIJJ)V", Any)]
#[async_method]
pub async fn xchange_active_pointer_grab<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _time = parameters.pop_long()?;
    let _cursor = parameters.pop_long()?;
    let _mask = parameters.pop_int()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XChangeActivePointerGrab(JIJJ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XChangePropertyImpl(JJJJIIJI)V", Any)]
#[async_method]
pub async fn xchange_property_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _nelements = parameters.pop_int()?;
    let _data = parameters.pop_long()?;
    let _mode = parameters.pop_int()?;
    let _format = parameters.pop_int()?;
    let _type_ = parameters.pop_long()?;
    let _atom = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XChangePropertyImpl(JJJJIIJI)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/X11/XlibWrapper.XChangePropertyS(JJJJIILjava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn xchange_property_s<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_reference()?;
    let _mode = parameters.pop_int()?;
    let _format = parameters.pop_int()?;
    let _type_ = parameters.pop_long()?;
    let _property = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XChangePropertyS(JJJJIILjava/lang/String;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XChangeWindowAttributes(JJJJ)V", Any)]
#[async_method]
pub async fn xchange_window_attributes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _attributes = parameters.pop_long()?;
    let _valuemask = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XChangeWindowAttributes(JJJJ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XClearWindow(JJ)V", Any)]
#[async_method]
pub async fn xclear_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XClearWindow(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XCloseDisplay(J)V", Any)]
#[async_method]
pub async fn xclose_display<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XCloseDisplay(J)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XConfigureWindow(JJJJ)V", Any)]
#[async_method]
pub async fn xconfigure_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _values = parameters.pop_long()?;
    let _value_mask = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XConfigureWindow(JJJJ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XConvertCase(JJJ)V", Any)]
#[async_method]
pub async fn xconvert_case<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _keysym_uppercase = parameters.pop_long()?;
    let _keysym_lowercase = parameters.pop_long()?;
    let _keysym = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XConvertCase(JJJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XConvertSelection(JJJJJJ)V", Any)]
#[async_method]
pub async fn xconvert_selection<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _time = parameters.pop_long()?;
    let _requestor = parameters.pop_long()?;
    let _property = parameters.pop_long()?;
    let _target = parameters.pop_long()?;
    let _selection = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XConvertSelection(JJJJJJ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XCreateBitmapFromData(JJJII)J", Any)]
#[async_method]
pub async fn xcreate_bitmap_from_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _data = parameters.pop_long()?;
    let _drawable = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XCreateBitmapFromData(JJJII)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XCreateFontCursor(JI)I", Any)]
#[async_method]
pub async fn xcreate_font_cursor<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _shape = parameters.pop_int()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XCreateFontCursor(JI)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XCreateGC(JJJJ)J", Any)]
#[async_method]
pub async fn xcreate_gc<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _values = parameters.pop_long()?;
    let _valuemask = parameters.pop_long()?;
    let _drawable = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XCreateGC(JJJJ)J".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XCreateImage(JJIIIJIIII)J", Any)]
#[async_method]
pub async fn xcreate_image<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _bytes_per_line = parameters.pop_int()?;
    let _bitmap_pad = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _data = parameters.pop_long()?;
    let _offset = parameters.pop_int()?;
    let _format = parameters.pop_int()?;
    let _depth = parameters.pop_int()?;
    let _visual_ptr = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XCreateImage(JJIIIJIIII)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XCreatePixmap(JJIII)J", Any)]
#[async_method]
pub async fn xcreate_pixmap<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _depth = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _drawable = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XCreatePixmap(JJIII)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XCreatePixmapCursor(JJJJJII)J", Any)]
#[async_method]
pub async fn xcreate_pixmap_cursor<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _back = parameters.pop_long()?;
    let _fore = parameters.pop_long()?;
    let _mask = parameters.pop_long()?;
    let _source = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XCreatePixmapCursor(JJJJJII)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XCreateWindow(JJIIIIIIJJJJ)J", Any)]
#[async_method]
pub async fn xcreate_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _attributes = parameters.pop_long()?;
    let _valuemask = parameters.pop_long()?;
    let _visual = parameters.pop_long()?;
    let _wclass = parameters.pop_long()?;
    let _depth = parameters.pop_int()?;
    let _border_width = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _parent = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XCreateWindow(JJIIIIIIJJJJ)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XDeleteProperty(JJJ)V", Any)]
#[async_method]
pub async fn xdelete_property<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _atom = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XDeleteProperty(JJJ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XDestroyImage(J)V", Any)]
#[async_method]
pub async fn xdestroy_image<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _image = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XDestroyImage(J)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XDestroyWindow(JJ)V", Any)]
#[async_method]
pub async fn xdestroy_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XDestroyWindow(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XDisplayString(J)J", Any)]
#[async_method]
pub async fn xdisplay_string<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XDisplayString(J)J".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XEventsQueued(JI)I", Any)]
#[async_method]
pub async fn xevents_queued<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _mode = parameters.pop_int()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XEventsQueued(JI)I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XFilterEvent(JJ)Z", Any)]
#[async_method]
pub async fn xfilter_event<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    let _ptr = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XFilterEvent(JJ)Z".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XFlush(J)V", Any)]
#[async_method]
pub async fn xflush<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XFlush(J)V".to_string()).into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XFree(J)V", Any)]
#[async_method]
pub async fn xfree<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XFree(J)V".to_string()).into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XFreeCursor(JJ)V", Any)]
#[async_method]
pub async fn xfree_cursor<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cursor = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XFreeCursor(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XFreeGC(JJ)V", Any)]
#[async_method]
pub async fn xfree_gc<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _gc = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XFreeGC(JJ)V".to_string()).into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XFreeModifiermap(J)V", Any)]
#[async_method]
pub async fn xfree_modifiermap<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _keymap = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XFreeModifiermap(J)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XFreePixmap(JJ)V", Any)]
#[async_method]
pub async fn xfree_pixmap<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixmap = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XFreePixmap(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XGetAtomName(JJ)Ljava/lang/String;", Any)]
#[async_method]
pub async fn xget_atom_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _atom = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XGetAtomName(JJ)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/X11/XlibWrapper.XGetDefault(JLjava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn xget_default<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _option = parameters.pop_reference()?;
    let _program = parameters.pop_reference()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XGetDefault(JLjava/lang/String;Ljava/lang/String;)Ljava/lang/String;".to_string()).into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XGetGeometry(JJJJJJJJJ)I", Any)]
#[async_method]
pub async fn xget_geometry<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _depth_return = parameters.pop_long()?;
    let _border_width_return = parameters.pop_long()?;
    let _height_return = parameters.pop_long()?;
    let _width_return = parameters.pop_long()?;
    let _y_return = parameters.pop_long()?;
    let _x_return = parameters.pop_long()?;
    let _root_return = parameters.pop_long()?;
    let _drawable = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XGetGeometry(JJJJJJJJJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XGetIconSizes(JJJJ)I", Any)]
#[async_method]
pub async fn xget_icon_sizes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ret_count = parameters.pop_long()?;
    let _ret_sizes = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XGetIconSizes(JJJJ)I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XGetInputFocus(J)J", Any)]
#[async_method]
pub async fn xget_input_focus<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XGetInputFocus(J)J".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XGetModifierMapping(J)J", Any)]
#[async_method]
pub async fn xget_modifier_mapping<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XGetModifierMapping(J)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XGetPointerMapping(JJI)I", Any)]
#[async_method]
pub async fn xget_pointer_mapping<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _button_number = parameters.pop_int()?;
    let _map = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XGetPointerMapping(JJI)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XGetSelectionOwner(JJ)J", Any)]
#[async_method]
pub async fn xget_selection_owner<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _selection = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XGetSelectionOwner(JJ)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XGetVisualInfo(JJJJ)J", Any)]
#[async_method]
pub async fn xget_visual_info<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _nitems_return = parameters.pop_long()?;
    let _vinfo_template = parameters.pop_long()?;
    let _vinfo_mask = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XGetVisualInfo(JJJJ)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XGetWMHints(JJJ)V", Any)]
#[async_method]
pub async fn xget_wmhints<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _wmhints = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XGetWMHints(JJJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XGetWMNormalHints(JJJJ)I", Any)]
#[async_method]
pub async fn xget_wmnormal_hints<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _supplied_return = parameters.pop_long()?;
    let _hints = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XGetWMNormalHints(JJJJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XGetWindowAttributes(JJJ)I", Any)]
#[async_method]
pub async fn xget_window_attributes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _attr_ptr = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XGetWindowAttributes(JJJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XGetWindowProperty(JJJJJJJJJJJJ)I", Any)]
#[async_method]
pub async fn xget_window_property<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data_ptr = parameters.pop_long()?;
    let _bytes_after = parameters.pop_long()?;
    let _nitems_ptr = parameters.pop_long()?;
    let _actualy_format = parameters.pop_long()?;
    let _actualy_type = parameters.pop_long()?;
    let _req_type = parameters.pop_long()?;
    let _delete = parameters.pop_long()?;
    let _long_length = parameters.pop_long()?;
    let _long_offset = parameters.pop_long()?;
    let _atom = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XGetWindowProperty(JJJJJJJJJJJJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XGrabKeyboard(JJIIIJ)I", Any)]
#[async_method]
pub async fn xgrab_keyboard<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _time = parameters.pop_long()?;
    let _keyboard_mode = parameters.pop_int()?;
    let _pointer_mode = parameters.pop_int()?;
    let _owner_events = parameters.pop_int()?;
    let _grab_window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XGrabKeyboard(JJIIIJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XGrabPointer(JJIIIIJJJ)I", Any)]
#[async_method]
pub async fn xgrab_pointer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _time = parameters.pop_long()?;
    let _cursor = parameters.pop_long()?;
    let _confine_to = parameters.pop_long()?;
    let _keyboard_mode = parameters.pop_int()?;
    let _pointer_mode = parameters.pop_int()?;
    let _event_mask = parameters.pop_int()?;
    let _owner_events = parameters.pop_int()?;
    let _grab_window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XGrabPointer(JJIIIIJJJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XGrabServer(J)V", Any)]
#[async_method]
pub async fn xgrab_server<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XGrabServer(J)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XIconifyWindow(JJJ)I", Any)]
#[async_method]
pub async fn xiconify_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen_number = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XIconifyWindow(JJJ)I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XInternAtoms(J[Ljava/lang/String;ZJ)I", Any)]
#[async_method]
pub async fn xintern_atoms<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _atoms = parameters.pop_long()?;
    let _only_if_exists = parameters.pop_bool()?;
    let _names_arr = parameters.pop_reference()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XInternAtoms(J[Ljava/lang/String;ZJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XKeycodeToKeysym(JII)J", Any)]
#[async_method]
pub async fn xkeycode_to_keysym<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    let _keycode = parameters.pop_int()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XKeycodeToKeysym(JII)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XKeysymToKeycode(JJ)I", Any)]
#[async_method]
pub async fn xkeysym_to_keycode<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _keysym = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XKeysymToKeycode(JJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XLowerWindow(JJ)V", Any)]
#[async_method]
pub async fn xlower_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XLowerWindow(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XMapRaised(JJ)V", Any)]
#[async_method]
pub async fn xmap_raised<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XMapRaised(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XMapWindow(JJ)V", Any)]
#[async_method]
pub async fn xmap_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XMapWindow(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XMaskEvent(JJJ)V", Any)]
#[async_method]
pub async fn xmask_event<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _event_return = parameters.pop_long()?;
    let _event_mask = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XMaskEvent(JJJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XMaxRequestSize(J)J", Any)]
#[async_method]
pub async fn xmax_request_size<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XMaxRequestSize(J)J".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XMoveResizeWindow(JJIIII)V", Any)]
#[async_method]
pub async fn xmove_resize_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XMoveResizeWindow(JJIIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XMoveWindow(JJII)V", Any)]
#[async_method]
pub async fn xmove_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XMoveWindow(JJII)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XNextEvent(JJ)V", Any)]
#[async_method]
pub async fn xnext_event<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XNextEvent(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XNextSecondaryLoopEvent(JJ)Z", Any)]
#[async_method]
pub async fn xnext_secondary_loop_event<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XNextSecondaryLoopEvent(JJ)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XOpenDisplay(J)J", Any)]
#[async_method]
pub async fn xopen_display<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XOpenDisplay(J)J".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XPeekEvent(JJ)V", Any)]
#[async_method]
pub async fn xpeek_event<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XPeekEvent(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XPutBackEvent(JJ)V", Any)]
#[async_method]
pub async fn xput_back_event<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _event = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XPutBackEvent(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XPutImage(JJJJIIIIII)V", Any)]
#[async_method]
pub async fn xput_image<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _dest_y = parameters.pop_int()?;
    let _dest_x = parameters.pop_int()?;
    let _src_y = parameters.pop_int()?;
    let _src_x = parameters.pop_int()?;
    let _image = parameters.pop_long()?;
    let _gc = parameters.pop_long()?;
    let _drawable = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XPutImage(JJJJIIIIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XQueryBestCursor(JJIIJJ)Z", Any)]
#[async_method]
pub async fn xquery_best_cursor<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height_return = parameters.pop_long()?;
    let _width_return = parameters.pop_long()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _drawable = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XQueryBestCursor(JJIIJJ)Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/X11/XlibWrapper.XQueryExtension(JLjava/lang/String;JJJ)Z",
    Any
)]
#[async_method]
pub async fn xquery_extension<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _err_return = parameters.pop_long()?;
    let _feve_return = parameters.pop_long()?;
    let _mop_return = parameters.pop_long()?;
    let _jstr = parameters.pop_reference()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XQueryExtension(JLjava/lang/String;JJJ)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XQueryKeymap(JJ)V", Any)]
#[async_method]
pub async fn xquery_keymap<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _vector = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XQueryKeymap(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XQueryPointer(JJJJJJJJJ)Z", Any)]
#[async_method]
pub async fn xquery_pointer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _mask_return = parameters.pop_long()?;
    let _win_y_return = parameters.pop_long()?;
    let _win_x_return = parameters.pop_long()?;
    let _root_y_return = parameters.pop_long()?;
    let _root_x_return = parameters.pop_long()?;
    let _child_return = parameters.pop_long()?;
    let _root_return = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XQueryPointer(JJJJJJJJJ)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XQueryTree(JJJJJJ)I", Any)]
#[async_method]
pub async fn xquery_tree<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _nchildren_return = parameters.pop_long()?;
    let _children_return = parameters.pop_long()?;
    let _parent_return = parameters.pop_long()?;
    let _root_return = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XQueryTree(JJJJJJ)I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XRaiseWindow(JJ)V", Any)]
#[async_method]
pub async fn xraise_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XRaiseWindow(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XRefreshKeyboardMapping(J)V", Any)]
#[async_method]
pub async fn xrefresh_keyboard_mapping<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _event = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XRefreshKeyboardMapping(J)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XReparentWindow(JJJII)V", Any)]
#[async_method]
pub async fn xreparent_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _parent = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XReparentWindow(JJJII)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XResizeWindow(JJII)V", Any)]
#[async_method]
pub async fn xresize_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XResizeWindow(JJII)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XRestackWindows(JJI)V", Any)]
#[async_method]
pub async fn xrestack_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _length = parameters.pop_int()?;
    let _windows = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XRestackWindows(JJI)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XScreenNumberOfScreen(J)J", Any)]
#[async_method]
pub async fn xscreen_number_of_screen<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XScreenNumberOfScreen(J)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSelectInput(JJJ)V", Any)]
#[async_method]
pub async fn xselect_input<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _event_mask = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XSelectInput(JJJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSendEvent(JJZJJ)I", Any)]
#[async_method]
pub async fn xsend_event<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _event = parameters.pop_long()?;
    let _event_mask = parameters.pop_long()?;
    let _propagate = parameters.pop_bool()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XSendEvent(JJZJJ)I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSetCloseDownMode(JI)V", Any)]
#[async_method]
pub async fn xset_close_down_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _close_mode = parameters.pop_int()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XSetCloseDownMode(JI)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSetErrorHandler(J)V", Any)]
#[async_method]
pub async fn xset_error_handler<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handler = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XSetErrorHandler(J)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSetInputFocus(JJ)V", Any)]
#[async_method]
pub async fn xset_input_focus<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XSetInputFocus(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSetInputFocus2(JJJ)V", Any)]
#[async_method]
pub async fn xset_input_focus2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _time = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XSetInputFocus2(JJJ)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/X11/XlibWrapper.XSetLocaleModifiers(Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn xset_locale_modifiers<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jstr = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XSetLocaleModifiers(Ljava/lang/String;)Ljava/lang/String;"
            .to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSetMinMaxHints(JJIIIIJ)V", Any)]
#[async_method]
pub async fn xset_min_max_hints<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flags = parameters.pop_long()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XSetMinMaxHints(JJIIIIJ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSetSelectionOwner(JJJJ)V", Any)]
#[async_method]
pub async fn xset_selection_owner<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _time = parameters.pop_long()?;
    let _owner = parameters.pop_long()?;
    let _selection = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XSetSelectionOwner(JJJJ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSetTransientFor(JJJ)V", Any)]
#[async_method]
pub async fn xset_transient_for<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _transient_for_window = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XSetTransientFor(JJJ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSetWMHints(JJJ)V", Any)]
#[async_method]
pub async fn xset_wmhints<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _wmhints = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XSetWMHints(JJJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSetWMNormalHints(JJJ)V", Any)]
#[async_method]
pub async fn xset_wmnormal_hints<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _hints = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XSetWMNormalHints(JJJ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSetWindowBackground(JJJ)V", Any)]
#[async_method]
pub async fn xset_window_background<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _background_pixel = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XSetWindowBackground(JJJ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSetWindowBackgroundPixmap(JJJ)V", Any)]
#[async_method]
pub async fn xset_window_background_pixmap<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixmap = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XSetWindowBackgroundPixmap(JJJ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XShapeQueryExtension(JJJ)Z", Any)]
#[async_method]
pub async fn xshape_query_extension<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _error_base_return = parameters.pop_long()?;
    let _event_base_return = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XShapeQueryExtension(JJJ)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSupportsLocale()Z", Any)]
#[async_method]
pub async fn xsupports_locale<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XSupportsLocale()Z".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSync(JI)V", Any)]
#[async_method]
pub async fn xsync<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _discard = parameters.pop_int()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XSync(JI)V".to_string()).into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XSynchronize(JZ)I", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn xsynchronize<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _onoff = parameters.pop_bool()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XSynchronize(JZ)I".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/X11/XlibWrapper.XTextPropertyToStringList([BJ)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn xtext_property_to_string_list<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _encoding_atom = parameters.pop_long()?;
    let _bytes = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XTextPropertyToStringList([BJ)[Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XTranslateCoordinates(JJJJJJJJ)I", Any)]
#[async_method]
pub async fn xtranslate_coordinates<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _child_return = parameters.pop_long()?;
    let _dest_y_return = parameters.pop_long()?;
    let _dest_x_return = parameters.pop_long()?;
    let _src_y = parameters.pop_long()?;
    let _src_x = parameters.pop_long()?;
    let _dest_w = parameters.pop_long()?;
    let _src_w = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XTranslateCoordinates(JJJJJJJJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XUngrabKeyboard(JJ)V", Any)]
#[async_method]
pub async fn xungrab_keyboard<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _time = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XUngrabKeyboard(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XUngrabPointer(JJ)V", Any)]
#[async_method]
pub async fn xungrab_pointer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _time = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XUngrabPointer(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XUngrabServer(J)V", Any)]
#[async_method]
pub async fn xungrab_server<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XUngrabServer(J)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XUnmapWindow(JJ)V", Any)]
#[async_method]
pub async fn xunmap_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XUnmapWindow(JJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XWindowEvent(JJJJ)V", Any)]
#[async_method]
pub async fn xwindow_event<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _event_return = parameters.pop_long()?;
    let _event_mask = parameters.pop_long()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XWindowEvent(JJJJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XdbeAllocateBackBufferName(JJI)J", Any)]
#[async_method]
pub async fn xdbe_allocate_back_buffer_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _swap_action = parameters.pop_int()?;
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XdbeAllocateBackBufferName(JJI)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XdbeBeginIdiom(J)I", Any)]
#[async_method]
pub async fn xdbe_begin_idiom<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XdbeBeginIdiom(J)I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XdbeDeallocateBackBufferName(JJ)I", Any)]
#[async_method]
pub async fn xdbe_deallocate_back_buffer_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buffer = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XdbeDeallocateBackBufferName(JJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XdbeEndIdiom(J)I", Any)]
#[async_method]
pub async fn xdbe_end_idiom<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XdbeEndIdiom(J)I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XdbeQueryExtension(JJJ)I", Any)]
#[async_method]
pub async fn xdbe_query_extension<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _minor_version_return = parameters.pop_long()?;
    let _major_version_return = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XdbeQueryExtension(JJJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XdbeSwapBuffers(JJI)I", Any)]
#[async_method]
pub async fn xdbe_swap_buffers<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _num_windows = parameters.pop_int()?;
    let _swap_info = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XdbeSwapBuffers(JJI)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XkbFreeKeyboard(JJZ)V", Any)]
#[async_method]
pub async fn xkb_free_keyboard<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _free_all = parameters.pop_bool()?;
    let _which = parameters.pop_long()?;
    let _xkb = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XkbFreeKeyboard(JJZ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XkbGetEffectiveGroup(J)I", Any)]
#[async_method]
pub async fn xkb_get_effective_group<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XkbGetEffectiveGroup(J)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XkbGetMap(JJJ)J", Any)]
#[async_method]
pub async fn xkb_get_map<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _device_spec = parameters.pop_long()?;
    let _which = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.XkbGetMap(JJJ)J".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XkbGetUpdatedMap(JJJ)J", Any)]
#[async_method]
pub async fn xkb_get_updated_map<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xkb = parameters.pop_long()?;
    let _which = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XkbGetUpdatedMap(JJJ)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XkbKeycodeToKeysym(JIII)J", Any)]
#[async_method]
pub async fn xkb_keycode_to_keysym<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _level = parameters.pop_int()?;
    let _group = parameters.pop_int()?;
    let _keycode = parameters.pop_int()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XkbKeycodeToKeysym(JIII)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XkbLibraryVersion(JJ)Z", Any)]
#[async_method]
pub async fn xkb_library_version<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _lib_minor_in_out = parameters.pop_long()?;
    let _lib_major_in_out = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XkbLibraryVersion(JJ)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XkbQueryExtension(JJJJJJ)Z", Any)]
#[async_method]
pub async fn xkb_query_extension<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _minor_in_out = parameters.pop_long()?;
    let _major_in_out = parameters.pop_long()?;
    let _error_rtrn = parameters.pop_long()?;
    let _event_rtrn = parameters.pop_long()?;
    let _opcode_rtrn = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XkbQueryExtension(JJJJJJ)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XkbSelectEventDetails(JJJJJ)V", Any)]
#[async_method]
pub async fn xkb_select_event_details<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _values_for_bits = parameters.pop_long()?;
    let _bits_to_change = parameters.pop_long()?;
    let _event_type = parameters.pop_long()?;
    let _device = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XkbSelectEventDetails(JJJJJ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XkbSelectEvents(JJJJ)V", Any)]
#[async_method]
pub async fn xkb_select_events<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _values_for_bits = parameters.pop_long()?;
    let _bits_to_change = parameters.pop_long()?;
    let _device = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XkbSelectEvents(JJJJ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XkbSetDetectableAutoRepeat(JZ)V", Any)]
#[async_method]
pub async fn xkb_set_detectable_auto_repeat<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _detectable = parameters.pop_bool()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XkbSetDetectableAutoRepeat(JZ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.XkbTranslateKeyCode(JIJJJ)Z", Any)]
#[async_method]
pub async fn xkb_translate_key_code<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _keysym_rtrn = parameters.pop_long()?;
    let _mods_rtrn = parameters.pop_long()?;
    let _mods = parameters.pop_long()?;
    let _keycode = parameters.pop_int()?;
    let _xkb = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.XkbTranslateKeyCode(JIJJJ)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.copyIntArray(JLjava/lang/Object;I)V", Any)]
#[async_method]
pub async fn copy_int_array<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _size = parameters.pop_int()?;
    let _array = parameters.pop_reference()?;
    let _dest = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.copyIntArray(JLjava/lang/Object;I)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.copyLongArray(JLjava/lang/Object;I)V", Any)]
#[async_method]
pub async fn copy_long_array<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _size = parameters.pop_int()?;
    let _array = parameters.pop_reference()?;
    let _dest = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.copyLongArray(JLjava/lang/Object;I)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.getAddress(Ljava/lang/Object;)J", Any)]
#[async_method]
pub async fn get_address<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _o = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.getAddress(Ljava/lang/Object;)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.getScreenOfWindow(JJ)J", Any)]
#[async_method]
pub async fn get_screen_of_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    let _display = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XlibWrapper.getScreenOfWindow(JJ)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.getStringBytes(J)[B", Any)]
#[async_method]
pub async fn get_string_bytes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _str_ptr = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.getStringBytes(J)[B".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XlibWrapper.memcpy(JJJ)V", Any)]
#[async_method]
pub async fn memcpy<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _length = parameters.pop_long()?;
    let _src_ptr = parameters.pop_long()?;
    let _dest_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/XlibWrapper.memcpy(JJJ)V".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_call_error_handler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = call_error_handler(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.CallErrorHandler(JJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_default_screen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = default_screen(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.DefaultScreen(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_display_height() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = display_height(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.DisplayHeight(JJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_display_height_mm() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = display_height_mm(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.DisplayHeightMM(JJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_display_width() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = display_width(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.DisplayWidth(JJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_display_width_mm() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = display_width_mm(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.DisplayWidthMM(JJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_does_backing_store() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = does_backing_store(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.DoesBackingStore(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_exit_secondary_loop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = exit_secondary_loop(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.ExitSecondaryLoop()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_property() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_property(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.GetProperty(JJJ)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_intern_atom() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = intern_atom(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.InternAtom(JLjava/lang/String;I)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_is_kana_keyboard() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_kana_keyboard(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.IsKanaKeyboard(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_is_keypad_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_keypad_key(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.IsKeypadKey(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_is_sun_keyboard() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_sun_keyboard(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.IsSunKeyboard(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_is_xsun_kpbehavior() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_xsun_kpbehavior(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.IsXsunKPBehavior(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_print_xerror_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = print_xerror_event(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.PrintXErrorEvent(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_root_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = root_window(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.RootWindow(JJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_screen_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = screen_count(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.ScreenCount(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_screen_of_display() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = screen_of_display(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.ScreenOfDisplay(JJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_server_vendor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = server_vendor(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.ServerVendor(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_bitmap_shape() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_bitmap_shape(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.SetBitmapShape(JJII[I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_property() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_property(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.SetProperty(JJJLjava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_rectangular_shape() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_rectangular_shape(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.SetRectangularShape(JJIIIILsun/java2d/pipe/Region;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_toolkit_error_handler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_toolkit_error_handler(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.SetToolkitErrorHandler()J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_zorder() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_zorder(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.SetZOrder(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_vendor_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = vendor_release(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.VendorRelease(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xalloc_color() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xalloc_color(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XAllocColor(JJJ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xalloc_size_hints() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xalloc_size_hints(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XAllocSizeHints()J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xalloc_wmhints() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xalloc_wmhints(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XAllocWMHints()J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xbell() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xbell(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XBell(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xchange_active_pointer_grab() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xchange_active_pointer_grab(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XChangeActivePointerGrab(JIJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xchange_property_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xchange_property_impl(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XChangePropertyImpl(JJJJIIJI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xchange_property_s() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xchange_property_s(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XChangePropertyS(JJJJIILjava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xchange_window_attributes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xchange_window_attributes(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XChangeWindowAttributes(JJJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xclear_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xclear_window(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XClearWindow(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xclose_display() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xclose_display(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XCloseDisplay(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xconfigure_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xconfigure_window(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XConfigureWindow(JJJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xconvert_case() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xconvert_case(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XConvertCase(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xconvert_selection() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xconvert_selection(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XConvertSelection(JJJJJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xcreate_bitmap_from_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xcreate_bitmap_from_data(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XCreateBitmapFromData(JJJII)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xcreate_font_cursor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            xcreate_font_cursor(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XCreateFontCursor(JI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xcreate_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xcreate_gc(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XCreateGC(JJJJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xcreate_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xcreate_image(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XCreateImage(JJIIIJIIII)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xcreate_pixmap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xcreate_pixmap(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XCreatePixmap(JJIII)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xcreate_pixmap_cursor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xcreate_pixmap_cursor(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XCreatePixmapCursor(JJJJJII)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xcreate_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xcreate_window(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XCreateWindow(JJIIIIIIJJJJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdelete_property() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdelete_property(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XDeleteProperty(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdestroy_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdestroy_image(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XDestroyImage(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdestroy_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdestroy_window(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XDestroyWindow(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdisplay_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdisplay_string(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XDisplayString(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xevents_queued() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            xevents_queued(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XEventsQueued(JI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xfilter_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xfilter_event(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XFilterEvent(JJ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xflush() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xflush(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XFlush(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xfree() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xfree(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XFree(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xfree_cursor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xfree_cursor(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XFreeCursor(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xfree_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xfree_gc(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XFreeGC(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xfree_modifiermap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xfree_modifiermap(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XFreeModifiermap(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xfree_pixmap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xfree_pixmap(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XFreePixmap(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xget_atom_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xget_atom_name(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGetAtomName(JJ)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xget_default() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xget_default(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGetDefault(JLjava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xget_geometry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xget_geometry(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGetGeometry(JJJJJJJJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xget_icon_sizes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xget_icon_sizes(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGetIconSizes(JJJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xget_input_focus() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xget_input_focus(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGetInputFocus(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xget_modifier_mapping() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xget_modifier_mapping(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGetModifierMapping(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xget_pointer_mapping() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xget_pointer_mapping(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGetPointerMapping(JJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xget_selection_owner() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xget_selection_owner(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGetSelectionOwner(JJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xget_visual_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xget_visual_info(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGetVisualInfo(JJJJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xget_wmhints() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xget_wmhints(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGetWMHints(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xget_wmnormal_hints() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xget_wmnormal_hints(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGetWMNormalHints(JJJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xget_window_attributes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xget_window_attributes(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGetWindowAttributes(JJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xget_window_property() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xget_window_property(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGetWindowProperty(JJJJJJJJJJJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xgrab_keyboard() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xgrab_keyboard(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGrabKeyboard(JJIIIJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xgrab_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xgrab_pointer(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGrabPointer(JJIIIIJJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xgrab_server() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xgrab_server(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XGrabServer(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xiconify_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xiconify_window(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XIconifyWindow(JJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xintern_atoms() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xintern_atoms(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::from(false),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XInternAtoms(J[Ljava/lang/String;ZJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xkeycode_to_keysym() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xkeycode_to_keysym(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XKeycodeToKeysym(JII)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xkeysym_to_keycode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xkeysym_to_keycode(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XKeysymToKeycode(JJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xlower_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xlower_window(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XLowerWindow(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xmap_raised() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xmap_raised(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XMapRaised(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xmap_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xmap_window(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XMapWindow(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xmask_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xmask_event(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XMaskEvent(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xmax_request_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xmax_request_size(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XMaxRequestSize(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xmove_resize_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xmove_resize_window(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XMoveResizeWindow(JJIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xmove_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xmove_window(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XMoveWindow(JJII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xnext_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xnext_event(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XNextEvent(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xnext_secondary_loop_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xnext_secondary_loop_event(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XNextSecondaryLoopEvent(JJ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xopen_display() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xopen_display(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XOpenDisplay(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xpeek_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xpeek_event(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XPeekEvent(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xput_back_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xput_back_event(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XPutBackEvent(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xput_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xput_image(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XPutImage(JJJJIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xquery_best_cursor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xquery_best_cursor(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XQueryBestCursor(JJIIJJ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xquery_extension() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xquery_extension(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XQueryExtension(JLjava/lang/String;JJJ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xquery_keymap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xquery_keymap(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XQueryKeymap(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xquery_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xquery_pointer(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XQueryPointer(JJJJJJJJJ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xquery_tree() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xquery_tree(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XQueryTree(JJJJJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xraise_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xraise_window(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XRaiseWindow(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xrefresh_keyboard_mapping() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xrefresh_keyboard_mapping(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XRefreshKeyboardMapping(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xreparent_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xreparent_window(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XReparentWindow(JJJII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xresize_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xresize_window(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XResizeWindow(JJII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xrestack_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xrestack_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XRestackWindows(JJI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xscreen_number_of_screen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xscreen_number_of_screen(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XScreenNumberOfScreen(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xselect_input() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xselect_input(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSelectInput(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xsend_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xsend_event(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::from(false),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSendEvent(JJZJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_close_down_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            xset_close_down_mode(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSetCloseDownMode(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_error_handler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xset_error_handler(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSetErrorHandler(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_input_focus() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xset_input_focus(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSetInputFocus(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_input_focus2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xset_input_focus2(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSetInputFocus2(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_locale_modifiers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            xset_locale_modifiers(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSetLocaleModifiers(Ljava/lang/String;)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_min_max_hints() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xset_min_max_hints(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSetMinMaxHints(JJIIIIJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_selection_owner() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xset_selection_owner(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSetSelectionOwner(JJJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_transient_for() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xset_transient_for(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSetTransientFor(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_wmhints() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xset_wmhints(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSetWMHints(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_wmnormal_hints() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xset_wmnormal_hints(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSetWMNormalHints(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_window_background() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xset_window_background(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSetWindowBackground(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_window_background_pixmap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xset_window_background_pixmap(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSetWindowBackgroundPixmap(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xshape_query_extension() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xshape_query_extension(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XShapeQueryExtension(JJJ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xsupports_locale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xsupports_locale(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSupportsLocale()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xsync() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xsync(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSync(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xsynchronize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xsynchronize(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XSynchronize(JZ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xtext_property_to_string_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xtext_property_to_string_list(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XTextPropertyToStringList([BJ)[Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xtranslate_coordinates() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xtranslate_coordinates(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XTranslateCoordinates(JJJJJJJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xungrab_keyboard() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xungrab_keyboard(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XUngrabKeyboard(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xungrab_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xungrab_pointer(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XUngrabPointer(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xungrab_server() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xungrab_server(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XUngrabServer(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xunmap_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xunmap_window(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XUnmapWindow(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xwindow_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xwindow_event(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XWindowEvent(JJJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdbe_allocate_back_buffer_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdbe_allocate_back_buffer_name(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XdbeAllocateBackBufferName(JJI)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdbe_begin_idiom() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdbe_begin_idiom(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XdbeBeginIdiom(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdbe_deallocate_back_buffer_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdbe_deallocate_back_buffer_name(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XdbeDeallocateBackBufferName(JJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdbe_end_idiom() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdbe_end_idiom(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XdbeEndIdiom(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdbe_query_extension() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdbe_query_extension(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XdbeQueryExtension(JJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdbe_swap_buffers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdbe_swap_buffers(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XdbeSwapBuffers(JJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xkb_free_keyboard() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xkb_free_keyboard(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XkbFreeKeyboard(JJZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xkb_get_effective_group() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xkb_get_effective_group(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XkbGetEffectiveGroup(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xkb_get_map() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xkb_get_map(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XkbGetMap(JJJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xkb_get_updated_map() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xkb_get_updated_map(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XkbGetUpdatedMap(JJJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xkb_keycode_to_keysym() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xkb_keycode_to_keysym(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XkbKeycodeToKeysym(JIII)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xkb_library_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xkb_library_version(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XkbLibraryVersion(JJ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xkb_query_extension() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xkb_query_extension(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XkbQueryExtension(JJJJJJ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xkb_select_event_details() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xkb_select_event_details(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XkbSelectEventDetails(JJJJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xkb_select_events() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xkb_select_events(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XkbSelectEvents(JJJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xkb_set_detectable_auto_repeat() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xkb_set_detectable_auto_repeat(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XkbSetDetectableAutoRepeat(JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xkb_translate_key_code() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xkb_translate_key_code(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.XkbTranslateKeyCode(JIJJJ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_copy_int_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = copy_int_array(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.copyIntArray(JLjava/lang/Object;I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_copy_long_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = copy_long_array(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.copyLongArray(JLjava/lang/Object;I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_address() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_address(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.getAddress(Ljava/lang/Object;)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_screen_of_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_screen_of_window(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.getScreenOfWindow(JJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_string_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_string_bytes(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.getStringBytes(J)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_memcpy() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = memcpy(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XlibWrapper.memcpy(JJJ)V",
            result.unwrap_err().to_string()
        );
    }
}

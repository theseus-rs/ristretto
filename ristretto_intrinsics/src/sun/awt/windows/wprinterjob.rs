use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WPrinterJob._startDoc(Ljava/lang/String;Ljava/lang/String;)Z",
    Any
)]
#[async_method]
pub async fn start_doc<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob._startDoc(Ljava/lang/String;Ljava/lang/String;)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.abortDoc()V", Any)]
#[async_method]
pub async fn abort_doc<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.abortDoc()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.beginPath(J)V", Any)]
#[async_method]
pub async fn begin_path<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _print_dc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.beginPath(J)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.closeFigure(J)V", Any)]
#[async_method]
pub async fn close_figure<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _print_dc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.closeFigure(J)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.deleteDC(JJJ)V", Any)]
#[async_method]
pub async fn delete_dc<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _devnames = parameters.pop_long()?;
    let _devmode = parameters.pop_long()?;
    let _dc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.deleteDC(JJJ)V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WPrinterJob.deviceEndPage(Ljava/awt/print/PageFormat;Ljava/awt/print/Printable;I)V",
    Any
)]
#[async_method]
pub async fn device_end_page<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _page_index = parameters.pop_int()?;
    let _painter = parameters.pop_reference()?;
    let _format = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.deviceEndPage(Ljava/awt/print/PageFormat;Ljava/awt/print/Printable;I)V".to_string()).into())
}
#[intrinsic_method(
    "sun/awt/windows/WPrinterJob.deviceStartPage(Ljava/awt/print/PageFormat;Ljava/awt/print/Printable;IZ)V",
    Any
)]
#[async_method]
pub async fn device_start_page<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _page_changed = parameters.pop_bool()?;
    let _page_index = parameters.pop_int()?;
    let _painter = parameters.pop_reference()?;
    let _format = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.deviceStartPage(Ljava/awt/print/PageFormat;Ljava/awt/print/Printable;IZ)V".to_string()).into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.drawDIBImage(J[BFFFFFFFFI[B)V", Any)]
#[async_method]
pub async fn draw_dibimage<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _bmi_colors_array = parameters.pop_reference()?;
    let _bit_count = parameters.pop_int()?;
    let _src_height = parameters.pop_float()?;
    let _src_width = parameters.pop_float()?;
    let _src_y = parameters.pop_float()?;
    let _src_x = parameters.pop_float()?;
    let _dest_height = parameters.pop_float()?;
    let _dest_width = parameters.pop_float()?;
    let _dest_y = parameters.pop_float()?;
    let _dest_x = parameters.pop_float()?;
    let _image = parameters.pop_reference()?;
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.drawDIBImage(J[BFFFFFFFFI[B)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.endDoc()V", Any)]
#[async_method]
pub async fn end_doc<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.endDoc()V".to_string()).into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.endPath(J)V", Any)]
#[async_method]
pub async fn end_path<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _print_dc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.endPath(J)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.fillPath(J)V", Any)]
#[async_method]
pub async fn fill_path<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _print_dc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.fillPath(J)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.fillRect(JFFFFIII)V", Any)]
#[async_method]
pub async fn fill_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _blue = parameters.pop_int()?;
    let _green = parameters.pop_int()?;
    let _red = parameters.pop_int()?;
    let _height = parameters.pop_float()?;
    let _width = parameters.pop_float()?;
    let _y = parameters.pop_float()?;
    let _x = parameters.pop_float()?;
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.fillRect(JFFFFIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.frameRect(JFFFF)V", Any)]
#[async_method]
pub async fn frame_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_float()?;
    let _width = parameters.pop_float()?;
    let _y = parameters.pop_float()?;
    let _x = parameters.pop_float()?;
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.frameRect(JFFFF)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WPrinterJob.getDefaultPage(Ljava/awt/print/PageFormat;)V",
    Any
)]
#[async_method]
pub async fn get_default_page<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _page = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.getDefaultPage(Ljava/awt/print/PageFormat;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.getGDIAdvance(JLjava/lang/String;)I", Any)]
#[async_method]
pub async fn get_gdiadvance<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _text = parameters.pop_reference()?;
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.getGDIAdvance(JLjava/lang/String;)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WPrinterJob.getNativePrintService()Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_native_print_service<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.getNativePrintService()Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.getPenX(J)I", Any)]
#[async_method]
pub async fn get_pen_x<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _print_dc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.getPenX(J)I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.getPenY(J)I", Any)]
#[async_method]
pub async fn get_pen_y<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _print_dc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.getPenY(J)I".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WPrinterJob.getWorldTransform(J[D)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_world_transform<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _transform = parameters.pop_reference()?;
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.getWorldTransform(J[D)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.initIDs()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.initPrinter()V", Any)]
#[async_method]
pub async fn init_printer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.initPrinter()V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WPrinterJob.jobSetup(Ljava/awt/print/Pageable;Z)Z",
    Any
)]
#[async_method]
pub async fn job_setup<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_bool()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.jobSetup(Ljava/awt/print/Pageable;Z)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.lineTo(JFF)V", Any)]
#[async_method]
pub async fn line_to<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_float()?;
    let _x = parameters.pop_float()?;
    let _print_dc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.lineTo(JFF)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.moveTo(JFF)V", Any)]
#[async_method]
pub async fn move_to<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_float()?;
    let _x = parameters.pop_float()?;
    let _print_dc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.moveTo(JFF)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.polyBezierTo(JFFFFFF)V", Any)]
#[async_method]
pub async fn poly_bezier_to<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _end_y = parameters.pop_float()?;
    let _end_x = parameters.pop_float()?;
    let _control2y = parameters.pop_float()?;
    let _control2x = parameters.pop_float()?;
    let _control1y = parameters.pop_float()?;
    let _control1x = parameters.pop_float()?;
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.polyBezierTo(JFFFFFF)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.printBand([BIIII)V", Any)]
#[async_method]
pub async fn print_band<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _image_array = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.printBand([BIIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.scale(JDD)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn scale<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _scale_y = parameters.pop_double()?;
    let _scale_x = parameters.pop_double()?;
    let _print_dc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.scale(JDD)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.selectClipPath(J)V", Any)]
#[async_method]
pub async fn select_clip_path<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.selectClipPath(J)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.selectPen(JFIII)V", Any)]
#[async_method]
pub async fn select_pen<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _blue = parameters.pop_int()?;
    let _green = parameters.pop_int()?;
    let _red = parameters.pop_int()?;
    let _width = parameters.pop_float()?;
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.selectPen(JFIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.selectSolidBrush(JIII)V", Any)]
#[async_method]
pub async fn select_solid_brush<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _blue = parameters.pop_int()?;
    let _green = parameters.pop_int()?;
    let _red = parameters.pop_int()?;
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.selectSolidBrush(JIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.selectStylePen(JJJFIII)Z", Any)]
#[async_method]
pub async fn select_style_pen<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _blue = parameters.pop_int()?;
    let _green = parameters.pop_int()?;
    let _red = parameters.pop_int()?;
    let _width = parameters.pop_float()?;
    let _join = parameters.pop_long()?;
    let _cap = parameters.pop_long()?;
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.selectStylePen(JJJFIII)Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WPrinterJob.setAdvancedGraphicsMode(J)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_advanced_graphics_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.setAdvancedGraphicsMode(J)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.setFont(JLjava/lang/String;FZZIF)Z", Any)]
#[async_method]
pub async fn set_font<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _aw_scale = parameters.pop_float()?;
    let _rotation = parameters.pop_int()?;
    let _is_italic = parameters.pop_bool()?;
    let _is_bold = parameters.pop_bool()?;
    let _font_size = parameters.pop_float()?;
    let _font_name = parameters.pop_reference()?;
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.setFont(JLjava/lang/String;FZZIF)Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WPrinterJob.setGraphicsMode(JI)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_graphics_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _mode = parameters.pop_int()?;
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.setGraphicsMode(JI)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.setNativeCopies(I)V", Any)]
#[async_method]
pub async fn set_native_copies<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _copies = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.setNativeCopies(I)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WPrinterJob.setNativePrintService(Ljava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn set_native_print_service<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _printer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.setNativePrintService(Ljava/lang/String;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.setPolyFillMode(JI)V", Any)]
#[async_method]
pub async fn set_poly_fill_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fill_rule = parameters.pop_int()?;
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.setPolyFillMode(JI)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.setTextColor(JIII)V", Any)]
#[async_method]
pub async fn set_text_color<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _blue = parameters.pop_int()?;
    let _green = parameters.pop_int()?;
    let _red = parameters.pop_int()?;
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.setTextColor(JIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WPrinterJob.setWorldTransform(J[D)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_world_transform<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _transform = parameters.pop_reference()?;
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.setWorldTransform(J[D)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WPrinterJob.showDocProperties(JLjavax/print/attribute/PrintRequestAttributeSet;ISSSSSSSSS)Z",
    Any
)]
#[async_method]
pub async fn show_doc_properties<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _yres = parameters.pop_int()?;
    let _xres_quality = parameters.pop_int()?;
    let _bin = parameters.pop_int()?;
    let _paper = parameters.pop_int()?;
    let _orient = parameters.pop_int()?;
    let _duplex = parameters.pop_int()?;
    let _color = parameters.pop_int()?;
    let _collate = parameters.pop_int()?;
    let _copies = parameters.pop_int()?;
    let _dm_fields = parameters.pop_int()?;
    let _attr_set = parameters.pop_reference()?;
    let _h_wnd_parent = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrinterJob.showDocProperties(JLjavax/print/attribute/PrintRequestAttributeSet;ISSSSSSSSS)Z".to_string()).into())
}
#[intrinsic_method("sun/awt/windows/WPrinterJob.textOut(JLjava/lang/String;IZFF[F)V", Any)]
#[async_method]
pub async fn text_out<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _positions = parameters.pop_reference()?;
    let _y = parameters.pop_float()?;
    let _x = parameters.pop_float()?;
    let _glyph_codes = parameters.pop_bool()?;
    let _str_len = parameters.pop_int()?;
    let _text = parameters.pop_reference()?;
    let _print_dc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.textOut(JLjava/lang/String;IZFF[F)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WPrinterJob.validatePaper(Ljava/awt/print/Paper;Ljava/awt/print/Paper;)V",
    Any
)]
#[async_method]
pub async fn validate_paper<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _new_paper = parameters.pop_reference()?;
    let _orig_paper = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPrinterJob.validatePaper(Ljava/awt/print/Paper;Ljava/awt/print/Paper;)V"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_start_doc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = start_doc(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob._startDoc(Ljava/lang/String;Ljava/lang/String;)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_abort_doc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = abort_doc(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.abortDoc()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_begin_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = begin_path(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.beginPath(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close_figure() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_figure(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.closeFigure(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_delete_dc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = delete_dc(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.deleteDC(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_device_end_page() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = device_end_page(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.deviceEndPage(Ljava/awt/print/PageFormat;Ljava/awt/print/Printable;I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_device_start_page() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = device_start_page(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.deviceStartPage(Ljava/awt/print/PageFormat;Ljava/awt/print/Printable;IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_draw_dibimage() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = draw_dibimage(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.drawDIBImage(J[BFFFFFFFFI[B)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_end_doc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = end_doc(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.endDoc()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_end_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = end_path(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.endPath(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_fill_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fill_path(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.fillPath(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_fill_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fill_rect(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.fillRect(JFFFFIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_frame_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = frame_rect(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.frameRect(JFFFF)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_default_page() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_default_page(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.getDefaultPage(Ljava/awt/print/PageFormat;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_gdiadvance() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_gdiadvance(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.getGDIAdvance(JLjava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_native_print_service() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_print_service(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.getNativePrintService()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_pen_x() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_pen_x(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.getPenX(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_pen_y() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_pen_y(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.getPenY(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_world_transform() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_world_transform(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.getWorldTransform(J[D)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_printer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_printer(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.initPrinter()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_job_setup() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = job_setup(
            thread,
            Parameters::new(vec![Value::Object(None), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.jobSetup(Ljava/awt/print/Pageable;Z)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_line_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = line_to(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Float(0.0), Value::Float(0.0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.lineTo(JFF)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_move_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = move_to(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Float(0.0), Value::Float(0.0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.moveTo(JFF)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_poly_bezier_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = poly_bezier_to(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.polyBezierTo(JFFFFFF)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_print_band() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = print_band(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.printBand([BIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_scale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = scale(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Double(0.0), Value::Double(0.0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.scale(JDD)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_select_clip_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = select_clip_path(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.selectClipPath(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_select_pen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = select_pen(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Float(0.0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.selectPen(JFIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_select_solid_brush() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = select_solid_brush(
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
            "sun/awt/windows/WPrinterJob.selectSolidBrush(JIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_select_style_pen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = select_style_pen(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Float(0.0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.selectStylePen(JJJFIII)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_advanced_graphics_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_advanced_graphics_mode(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.setAdvancedGraphicsMode(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_font() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_font(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Float(0.0),
                Value::from(false),
                Value::from(false),
                Value::Int(0),
                Value::Float(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.setFont(JLjava/lang/String;FZZIF)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_graphics_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_graphics_mode(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.setGraphicsMode(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_native_copies() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_native_copies(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.setNativeCopies(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_native_print_service() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_native_print_service(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.setNativePrintService(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_poly_fill_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_poly_fill_mode(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.setPolyFillMode(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_text_color() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_text_color(
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
            "sun/awt/windows/WPrinterJob.setTextColor(JIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_world_transform() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_world_transform(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.setWorldTransform(J[D)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_show_doc_properties() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = show_doc_properties(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
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
            "sun/awt/windows/WPrinterJob.showDocProperties(JLjavax/print/attribute/PrintRequestAttributeSet;ISSSSSSSSS)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_text_out() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = text_out(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.textOut(JLjava/lang/String;IZFF[F)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_validate_paper() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = validate_paper(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WPrinterJob.validatePaper(Ljava/awt/print/Paper;Ljava/awt/print/Paper;)V",
            result.unwrap_err().to_string()
        );
    }
}

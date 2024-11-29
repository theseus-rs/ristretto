use crate::arguments::Arguments;
use crate::native_methods::{
    apple_applescript_applescriptengine, apple_applescript_applescriptenginefactory,
    apple_laf_jrsuiconstants, apple_laf_jrsuicontrol, apple_laf_jrsuifocus,
    apple_laf_jrsuiutils_scrollbar, apple_launcher_javaapplauncher, apple_security_keychainstore,
    com_apple_concurrent_libdispatchnative, com_apple_eawt_appdockiconhandler,
    com_apple_eawt_appeventhandler, com_apple_eawt_application, com_apple_eawt_appmenubarhandler,
    com_apple_eawt_appmischandlers, com_apple_eio_filemanager, com_apple_laf_aquafileview,
    com_apple_laf_aquanativeresources, com_apple_laf_screenmenu, com_apple_laf_screenpopupfactory,
    com_sun_demo_jvmti_hprof_tracker, com_sun_imageio_plugins_jpeg_jpegimagereader,
    com_sun_imageio_plugins_jpeg_jpegimagewriter, com_sun_java_swing_plaf_gtk_gtkengine,
    com_sun_java_swing_plaf_gtk_gtkstyle, com_sun_java_util_jar_pack_nativeunpack,
    com_sun_management_internal_diagnosticcommandimpl, com_sun_management_internal_flag,
    com_sun_management_internal_garbagecollectorextimpl, com_sun_management_internal_gcinfobuilder,
    com_sun_management_internal_operatingsystemimpl, com_sun_media_sound_directaudiodevice,
    com_sun_media_sound_directaudiodeviceprovider, com_sun_media_sound_midiindevice,
    com_sun_media_sound_midiindeviceprovider, com_sun_media_sound_midioutdevice,
    com_sun_media_sound_midioutdeviceprovider, com_sun_media_sound_platform,
    com_sun_media_sound_portmixer, com_sun_media_sound_portmixerprovider,
    com_sun_security_auth_module_ntsystem, com_sun_security_auth_module_unixsystem,
    java_awt_awtevent, java_awt_button, java_awt_checkbox, java_awt_checkboxmenuitem,
    java_awt_choice, java_awt_color, java_awt_component, java_awt_container, java_awt_cursor,
    java_awt_dialog, java_awt_dimension, java_awt_event, java_awt_event_inputevent,
    java_awt_event_keyevent, java_awt_event_mouseevent, java_awt_filedialog, java_awt_font,
    java_awt_fontmetrics, java_awt_frame, java_awt_image_bufferedimage, java_awt_image_colormodel,
    java_awt_image_componentsamplemodel, java_awt_image_indexcolormodel, java_awt_image_kernel,
    java_awt_image_raster, java_awt_image_samplemodel, java_awt_image_singlepixelpackedsamplemodel,
    java_awt_insets, java_awt_keyboardfocusmanager, java_awt_label, java_awt_menu,
    java_awt_menubar, java_awt_menucomponent, java_awt_menuitem, java_awt_rectangle,
    java_awt_scrollbar, java_awt_scrollpane, java_awt_scrollpaneadjustable, java_awt_splashscreen,
    java_awt_textarea, java_awt_textfield, java_awt_toolkit, java_awt_trayicon, java_awt_window,
    java_io_console, java_io_filecleanable, java_io_filedescriptor, java_io_fileinputstream,
    java_io_fileoutputstream, java_io_objectinputstream, java_io_objectoutputstream,
    java_io_objectstreamclass, java_io_randomaccessfile, java_io_unixfilesystem,
    java_io_winntfilesystem, java_lang_class, java_lang_classloader,
    java_lang_classloader_nativelibrary, java_lang_compiler, java_lang_double, java_lang_float,
    java_lang_invoke_lambdaproxyclassarchive, java_lang_invoke_methodhandle,
    java_lang_invoke_methodhandlenatives, java_lang_invoke_varhandle, java_lang_module,
    java_lang_nullpointerexception, java_lang_object, java_lang_package,
    java_lang_processenvironment, java_lang_processhandleimpl, java_lang_processhandleimpl_info,
    java_lang_processimpl, java_lang_ref_finalizer, java_lang_ref_phantomreference,
    java_lang_ref_reference, java_lang_reflect_array, java_lang_reflect_executable,
    java_lang_reflect_field, java_lang_reflect_proxy, java_lang_runtime, java_lang_securitymanager,
    java_lang_shutdown, java_lang_stackframeinfo, java_lang_stackstreamfactory,
    java_lang_stackstreamfactory_abstractstackwalker, java_lang_stacktraceelement,
    java_lang_strictmath, java_lang_string, java_lang_stringcoding, java_lang_stringutf16,
    java_lang_system, java_lang_thread, java_lang_throwable, java_lang_unixprocess,
    java_lang_virtualthread, java_net_abstractplaindatagramsocketimpl,
    java_net_abstractplainsocketimpl, java_net_datagrampacket, java_net_inet4address,
    java_net_inet4addressimpl, java_net_inet6address, java_net_inet6addressimpl,
    java_net_inetaddress, java_net_inetaddressimplfactory, java_net_networkinterface,
    java_net_plaindatagramsocketimpl, java_net_plainsocketimpl, java_net_socketcleanable,
    java_net_socketinputstream, java_net_socketoutputstream, java_nio_bits,
    java_nio_mappedbytebuffer, java_nio_mappedmemoryutils, java_security_accesscontroller,
    java_util_concurrent_atomic_atomiclong, java_util_jar_jarfile, java_util_logging_filehandler,
    java_util_prefs_filesystempreferences, java_util_prefs_macosxpreferencesfile,
    java_util_timezone, java_util_zip_adler32, java_util_zip_crc32, java_util_zip_deflater,
    java_util_zip_inflater, java_util_zip_zipfile, jdk_internal_agent_filesystemimpl,
    jdk_internal_foreign_abi_fallback_libfallback, jdk_internal_foreign_abi_nativeentrypoint,
    jdk_internal_foreign_abi_programmableinvoker,
    jdk_internal_foreign_abi_programmableupcallhandler, jdk_internal_foreign_abi_upcalllinker,
    jdk_internal_foreign_abi_upcallstubs, jdk_internal_invoke_nativeentrypoint,
    jdk_internal_io_jdkconsoleimpl, jdk_internal_jimage_nativeimagebuffer,
    jdk_internal_loader_bootloader, jdk_internal_loader_nativelibraries,
    jdk_internal_loader_nativelibrary, jdk_internal_loader_rawnativelibraries,
    jdk_internal_misc_cds, jdk_internal_misc_previewfeatures, jdk_internal_misc_scopedmemoryaccess,
    jdk_internal_misc_signal, jdk_internal_misc_unsafe, jdk_internal_misc_vm,
    jdk_internal_module_modulebootstrap, jdk_internal_org_jline_terminal_impl_jna_osx_clibraryimpl,
    jdk_internal_perf_perf, jdk_internal_reflect_constantpool,
    jdk_internal_reflect_directconstructorhandleaccessor_nativeaccessor,
    jdk_internal_reflect_directmethodhandleaccessor_nativeaccessor,
    jdk_internal_reflect_nativeconstructoraccessorimpl,
    jdk_internal_reflect_nativemethodaccessorimpl, jdk_internal_reflect_reflection,
    jdk_internal_util_systemprops_raw, jdk_internal_vm_continuation,
    jdk_internal_vm_continuationsupport, jdk_internal_vm_foreignlinkersupport,
    jdk_internal_vm_vector_vectorsupport, jdk_internal_vm_vmsupport, jdk_jfr_internal_jvm,
    jdk_net_macosxsocketoptions, jdk_vm_ci_runtime_jvmci, jdk_vm_ci_services_services,
    sun_awt_cgraphicsconfig, sun_awt_cgraphicsdevice, sun_awt_cgraphicsenvironment,
    sun_awt_debugsettings, sun_awt_defaultmouseinfopeer, sun_awt_fcfontmanager,
    sun_awt_fontdescriptor, sun_awt_image_bufimgsurfacedata, sun_awt_image_bytecomponentraster,
    sun_awt_image_bytepackedraster, sun_awt_image_databuffernative, sun_awt_image_gifimagedecoder,
    sun_awt_image_imagerepresentation, sun_awt_image_imaginglib,
    sun_awt_image_integercomponentraster, sun_awt_image_jpegimagedecoder,
    sun_awt_image_shortcomponentraster, sun_awt_platformfont, sun_awt_platformgraphicsinfo,
    sun_awt_suntoolkit, sun_awt_unixtoolkit, sun_awt_x11graphicsconfig, sun_awt_x11graphicsdevice,
    sun_awt_x11graphicsenvironment, sun_awt_x11inputmethod, sun_font_cchartoglyphmapper,
    sun_font_cfont, sun_font_cfontmanager, sun_font_colorglyphsurfacedata, sun_font_cstrike,
    sun_font_cstrikedisposer, sun_font_filefontstrike, sun_font_fontconfigmanager,
    sun_font_freetypefontscaler, sun_font_nativefont, sun_font_nativestrike,
    sun_font_nativestrikedisposer, sun_font_nullfontscaler, sun_font_strikecache,
    sun_font_sunfontmanager, sun_font_sunlayoutengine, sun_font_x11textrenderer,
    sun_instrument_instrumentationimpl, sun_io_win32errormode, sun_java2d_cmm_lcms_lcms,
    sun_java2d_crenderer, sun_java2d_defaultdisposerrecord, sun_java2d_disposer,
    sun_java2d_jules_julesaatilegenerator, sun_java2d_jules_julespathbuf, sun_java2d_loops_blit,
    sun_java2d_loops_blitbg, sun_java2d_loops_drawglyphlist, sun_java2d_loops_drawglyphlistaa,
    sun_java2d_loops_drawglyphlistlcd, sun_java2d_loops_drawline,
    sun_java2d_loops_drawparallelogram, sun_java2d_loops_drawpath, sun_java2d_loops_drawpolygons,
    sun_java2d_loops_drawrect, sun_java2d_loops_fillparallelogram, sun_java2d_loops_fillpath,
    sun_java2d_loops_fillrect, sun_java2d_loops_fillspans, sun_java2d_loops_graphicsprimitivemgr,
    sun_java2d_loops_maskblit, sun_java2d_loops_maskfill, sun_java2d_loops_scaledblit,
    sun_java2d_loops_transformblit, sun_java2d_loops_transformhelper,
    sun_java2d_metal_mtlgraphicsconfig, sun_java2d_metal_mtllayer, sun_java2d_metal_mtlmaskfill,
    sun_java2d_metal_mtlrenderer, sun_java2d_metal_mtlrenderqueue, sun_java2d_metal_mtlsurfacedata,
    sun_java2d_metal_mtltextrenderer, sun_java2d_opengl_cglgraphicsconfig,
    sun_java2d_opengl_cgllayer, sun_java2d_opengl_cglsurfacedata,
    sun_java2d_opengl_glxgraphicsconfig, sun_java2d_opengl_glxsurfacedata,
    sun_java2d_opengl_oglcontext, sun_java2d_opengl_oglmaskfill, sun_java2d_opengl_oglrenderer,
    sun_java2d_opengl_oglrenderqueue, sun_java2d_opengl_oglsurfacedata,
    sun_java2d_opengl_ogltextrenderer, sun_java2d_osxoffscreensurfacedata,
    sun_java2d_pipe_bufferedmaskblit, sun_java2d_pipe_bufferedrenderpipe, sun_java2d_pipe_region,
    sun_java2d_pipe_shapespaniterator, sun_java2d_pipe_spancliprenderer, sun_java2d_surfacedata,
    sun_java2d_x11_x11pmblitbgloops, sun_java2d_x11_x11pmblitloops, sun_java2d_x11_x11renderer,
    sun_java2d_x11_x11surfacedata, sun_java2d_x11_xsurfacedata, sun_java2d_xr_xidgenerator,
    sun_java2d_xr_xrbackendnative, sun_java2d_xr_xrmaskblit, sun_java2d_xr_xrmaskfill,
    sun_java2d_xr_xrsurfacedata, sun_lwawt_macosx_caccessibility, sun_lwawt_macosx_caccessible,
    sun_lwawt_macosx_ccheckboxmenuitem, sun_lwawt_macosx_cclipboard,
    sun_lwawt_macosx_ccursormanager, sun_lwawt_macosx_cdatatransferer,
    sun_lwawt_macosx_cdesktoppeer, sun_lwawt_macosx_cdragsourcecontextpeer,
    sun_lwawt_macosx_cdroptarget, sun_lwawt_macosx_cdroptargetcontextpeer,
    sun_lwawt_macosx_cfiledialog, sun_lwawt_macosx_cfretainedresource, sun_lwawt_macosx_cimage,
    sun_lwawt_macosx_cinputmethod, sun_lwawt_macosx_cinputmethoddescriptor, sun_lwawt_macosx_cmenu,
    sun_lwawt_macosx_cmenubar, sun_lwawt_macosx_cmenuitem, sun_lwawt_macosx_cplatformcomponent,
    sun_lwawt_macosx_cplatformview, sun_lwawt_macosx_cplatformwindow, sun_lwawt_macosx_cpopupmenu,
    sun_lwawt_macosx_cprinterjob, sun_lwawt_macosx_cprinterjobdialog,
    sun_lwawt_macosx_cprinterpagedialog, sun_lwawt_macosx_cprintersurfacedata,
    sun_lwawt_macosx_crobot, sun_lwawt_macosx_ctextpipe, sun_lwawt_macosx_ctrayicon,
    sun_lwawt_macosx_cwrapper_nsview, sun_lwawt_macosx_cwrapper_nswindow,
    sun_lwawt_macosx_lwctoolkit, sun_lwawt_macosx_nsevent, sun_management_classloadingimpl,
    sun_management_diagnosticcommandimpl, sun_management_filesystemimpl, sun_management_flag,
    sun_management_garbagecollectorimpl, sun_management_gcinfobuilder, sun_management_memoryimpl,
    sun_management_memorymanagerimpl, sun_management_memorypoolimpl,
    sun_management_operatingsystemimpl, sun_management_threadimpl, sun_management_vmmanagementimpl,
    sun_misc_gc, sun_misc_messageutils, sun_misc_nativesignalhandler, sun_misc_perf,
    sun_misc_signal, sun_misc_unsafe, sun_misc_urlclasspath, sun_misc_version, sun_misc_vm,
    sun_misc_vmsupport, sun_net_dns_resolverconfigurationimpl, sun_net_extendedoptionsimpl,
    sun_net_portconfig, sun_net_sdp_sdpsupport, sun_net_spi_defaultproxyselector,
    sun_nio_ch_datagramchannelimpl, sun_nio_ch_datagramdispatcher, sun_nio_ch_filechannelimpl,
    sun_nio_ch_filedispatcherimpl, sun_nio_ch_filekey, sun_nio_ch_inheritedchannel,
    sun_nio_ch_ioutil, sun_nio_ch_kqueue, sun_nio_ch_kqueuearraywrapper, sun_nio_ch_kqueueport,
    sun_nio_ch_nativesocketaddress, sun_nio_ch_nativethread, sun_nio_ch_net,
    sun_nio_ch_pollarraywrapper, sun_nio_ch_pollselectorimpl, sun_nio_ch_sctp_sctpnet,
    sun_nio_ch_serversocketchannelimpl, sun_nio_ch_socketchannelimpl, sun_nio_ch_socketdispatcher,
    sun_nio_ch_unixasynchronousserversocketchannelimpl,
    sun_nio_ch_unixasynchronoussocketchannelimpl, sun_nio_ch_unixdispatcher,
    sun_nio_ch_unixdomainsockets, sun_nio_ch_unixfiledispatcherimpl, sun_nio_fs_bsdfilesystem,
    sun_nio_fs_bsdnativedispatcher, sun_nio_fs_macosxnativedispatcher, sun_nio_fs_unixcopyfile,
    sun_nio_fs_unixfilesystem, sun_nio_fs_unixnativedispatcher, sun_nio_fs_utifiletypedetector,
    sun_print_cupsprinter, sun_reflect_constantpool, sun_reflect_nativeconstructoraccessorimpl,
    sun_reflect_nativemethodaccessorimpl, sun_reflect_reflection, sun_rmi_transport_gc,
    sun_security_ec_ecdhkeyagreement, sun_security_ec_ecdsasignature,
    sun_security_ec_eckeypairgenerator, sun_security_jgss_wrapper_gsslibstub,
    sun_security_krb5_config, sun_security_krb5_credentials,
    sun_security_krb5_scdynamicstoreconfig, sun_security_pkcs11_secmod,
    sun_security_pkcs11_wrapper_pkcs11, sun_security_smartcardio_pcsc,
    sun_security_smartcardio_platformpcsc, sun_tools_attach_virtualmachineimpl,
    sun_tracing_dtrace_jvm, sun_util_locale_provider_hostlocaleprovideradapterimpl,
};
use crate::thread::Thread;
use crate::Result;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };
const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };
const JAVA_18: Version = Version::Java18 { minor: 0 };
const JAVA_19: Version = Version::Java19 { minor: 0 };
const JAVA_20: Version = Version::Java20 { minor: 0 };
const JAVA_21: Version = Version::Java21 { minor: 0 };
const JAVA_22: Version = Version::Java22 { minor: 0 };

/// A Rust method is a method that is implemented in Rust and is called from Java code instead of
/// being implemented in Java byte code.
pub type RustMethod = fn(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>>;

#[expect(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct MethodRegistry {
    java_version: Version,
    methods: HashMap<String, RustMethod>,
}

impl MethodRegistry {
    /// Create a new registry.
    #[expect(clippy::too_many_lines)]
    pub fn new(java_version: &Version) -> Self {
        let java_version = java_version.clone();
        let mut method_registry = MethodRegistry {
            java_version: java_version.clone(),
            methods: HashMap::new(),
        };

        if java_version <= JAVA_8 {
            apple_applescript_applescriptengine::register(&mut method_registry);
            apple_applescript_applescriptenginefactory::register(&mut method_registry);
            apple_launcher_javaapplauncher::register(&mut method_registry);
            com_apple_concurrent_libdispatchnative::register(&mut method_registry);
            com_apple_laf_screenpopupfactory::register(&mut method_registry);
            com_sun_demo_jvmti_hprof_tracker::register(&mut method_registry);
            com_sun_java_swing_plaf_gtk_gtkengine::register(&mut method_registry);
            com_sun_java_swing_plaf_gtk_gtkstyle::register(&mut method_registry);
            java_awt_image_componentsamplemodel::register(&mut method_registry);
            java_lang_compiler::register(&mut method_registry);
            java_lang_package::register(&mut method_registry);
            java_lang_reflect_proxy::register(&mut method_registry);
            java_lang_unixprocess::register(&mut method_registry);
            java_nio_bits::register(&mut method_registry);
            java_util_jar_jarfile::register(&mut method_registry);
            java_util_logging_filehandler::register(&mut method_registry);
            java_util_zip_zipfile::register(&mut method_registry);
            sun_awt_cgraphicsconfig::register(&mut method_registry);
            sun_awt_defaultmouseinfopeer::register(&mut method_registry);
            sun_awt_fcfontmanager::register(&mut method_registry);
            sun_awt_unixtoolkit::register(&mut method_registry);
            sun_awt_x11graphicsconfig::register(&mut method_registry);
            sun_awt_x11graphicsdevice::register(&mut method_registry);
            sun_awt_x11graphicsenvironment::register(&mut method_registry);
            sun_awt_x11inputmethod::register(&mut method_registry);
            sun_font_fontconfigmanager::register(&mut method_registry);
            sun_font_nativefont::register(&mut method_registry);
            sun_font_nativestrike::register(&mut method_registry);
            sun_font_nativestrikedisposer::register(&mut method_registry);
            sun_font_x11textrenderer::register(&mut method_registry);
            sun_java2d_jules_julesaatilegenerator::register(&mut method_registry);
            sun_java2d_jules_julespathbuf::register(&mut method_registry);
            sun_java2d_opengl_glxgraphicsconfig::register(&mut method_registry);
            sun_java2d_opengl_glxsurfacedata::register(&mut method_registry);
            sun_java2d_x11_x11pmblitbgloops::register(&mut method_registry);
            sun_java2d_x11_x11pmblitloops::register(&mut method_registry);
            sun_java2d_x11_x11renderer::register(&mut method_registry);
            sun_java2d_x11_x11surfacedata::register(&mut method_registry);
            sun_java2d_x11_xsurfacedata::register(&mut method_registry);
            sun_java2d_xr_xidgenerator::register(&mut method_registry);
            sun_java2d_xr_xrbackendnative::register(&mut method_registry);
            sun_java2d_xr_xrmaskblit::register(&mut method_registry);
            sun_java2d_xr_xrmaskfill::register(&mut method_registry);
            sun_java2d_xr_xrsurfacedata::register(&mut method_registry);
            sun_management_diagnosticcommandimpl::register(&mut method_registry);
            sun_management_filesystemimpl::register(&mut method_registry);
            sun_management_flag::register(&mut method_registry);
            sun_management_gcinfobuilder::register(&mut method_registry);
            sun_management_operatingsystemimpl::register(&mut method_registry);
            sun_misc_gc::register(&mut method_registry);
            sun_misc_messageutils::register(&mut method_registry);
            sun_misc_nativesignalhandler::register(&mut method_registry);
            sun_misc_perf::register(&mut method_registry);
            sun_misc_signal::register(&mut method_registry);
            sun_misc_unsafe::register(&mut method_registry);
            sun_misc_urlclasspath::register(&mut method_registry);
            sun_misc_version::register(&mut method_registry);
            sun_misc_vm::register(&mut method_registry);
            sun_misc_vmsupport::register(&mut method_registry);
            sun_net_extendedoptionsimpl::register(&mut method_registry);
            sun_nio_ch_kqueuearraywrapper::register(&mut method_registry);
            sun_nio_ch_kqueueport::register(&mut method_registry);
            sun_nio_ch_pollarraywrapper::register(&mut method_registry);
            sun_nio_ch_sctp_sctpnet::register(&mut method_registry);
            sun_reflect_constantpool::register(&mut method_registry);
            sun_reflect_nativeconstructoraccessorimpl::register(&mut method_registry);
            sun_reflect_nativemethodaccessorimpl::register(&mut method_registry);
            sun_reflect_reflection::register(&mut method_registry);
            sun_tracing_dtrace_jvm::register(&mut method_registry);
        }

        if java_version == JAVA_11 {
            com_sun_java_util_jar_pack_nativeunpack::register(&mut method_registry);
            java_io_objectinputstream::register(&mut method_registry);
            java_io_objectoutputstream::register(&mut method_registry);
            java_lang_classloader_nativelibrary::register(&mut method_registry);
            java_lang_stringcoding::register(&mut method_registry);
            java_net_abstractplaindatagramsocketimpl::register(&mut method_registry);
            java_net_abstractplainsocketimpl::register(&mut method_registry);
            java_net_socketcleanable::register(&mut method_registry);
            java_nio_mappedbytebuffer::register(&mut method_registry);
            sun_nio_ch_serversocketchannelimpl::register(&mut method_registry);
            sun_nio_ch_socketchannelimpl::register(&mut method_registry);
            sun_nio_ch_unixasynchronousserversocketchannelimpl::register(&mut method_registry);
            sun_security_ec_ecdhkeyagreement::register(&mut method_registry);
            sun_security_ec_ecdsasignature::register(&mut method_registry);
            sun_security_ec_eckeypairgenerator::register(&mut method_registry);
        }
        if java_version >= JAVA_11 {
            com_apple_eawt_application::register(&mut method_registry);
            com_sun_management_internal_diagnosticcommandimpl::register(&mut method_registry);
            com_sun_management_internal_flag::register(&mut method_registry);
            com_sun_management_internal_garbagecollectorextimpl::register(&mut method_registry);
            com_sun_management_internal_gcinfobuilder::register(&mut method_registry);
            com_sun_management_internal_operatingsystemimpl::register(&mut method_registry);
            com_sun_security_auth_module_ntsystem::register(&mut method_registry);
            java_awt_scrollbar::register(&mut method_registry);
            java_awt_event_inputevent::register(&mut method_registry);
            java_awt_event_keyevent::register(&mut method_registry);
            java_awt_event_mouseevent::register(&mut method_registry);
            java_awt_image_bufferedimage::register(&mut method_registry);
            java_awt_image_colormodel::register(&mut method_registry);
            java_awt_image_indexcolormodel::register(&mut method_registry);
            java_awt_image_kernel::register(&mut method_registry);
            java_awt_image_raster::register(&mut method_registry);
            java_awt_image_samplemodel::register(&mut method_registry);
            java_awt_image_singlepixelpackedsamplemodel::register(&mut method_registry);
            java_io_filecleanable::register(&mut method_registry);
            java_lang_module::register(&mut method_registry);
            java_lang_processhandleimpl::register(&mut method_registry);
            java_lang_processhandleimpl_info::register(&mut method_registry);
            java_lang_processimpl::register(&mut method_registry);
            java_lang_stackstreamfactory::register(&mut method_registry);
            java_lang_stackstreamfactory_abstractstackwalker::register(&mut method_registry);
            java_lang_stacktraceelement::register(&mut method_registry);
            java_lang_stringutf16::register(&mut method_registry);
            java_lang_invoke_methodhandle::register(&mut method_registry);
            java_lang_invoke_methodhandlenatives::register(&mut method_registry);
            java_lang_invoke_varhandle::register(&mut method_registry);
            java_lang_ref_reference::register(&mut method_registry);
            java_lang_reflect_array::register(&mut method_registry);
            java_lang_reflect_executable::register(&mut method_registry);
            java_lang_reflect_field::register(&mut method_registry);
            java_util_timezone::register(&mut method_registry);
            jdk_internal_agent_filesystemimpl::register(&mut method_registry);
            jdk_internal_jimage_nativeimagebuffer::register(&mut method_registry);
            jdk_internal_loader_bootloader::register(&mut method_registry);
            jdk_internal_misc_signal::register(&mut method_registry);
            jdk_internal_misc_unsafe::register(&mut method_registry);
            jdk_internal_misc_vm::register(&mut method_registry);
            jdk_internal_perf_perf::register(&mut method_registry);
            jdk_internal_reflect_constantpool::register(&mut method_registry);

            if java_version <= JAVA_22 {
                jdk_internal_reflect_nativeconstructoraccessorimpl::register(&mut method_registry);
                jdk_internal_reflect_nativemethodaccessorimpl::register(&mut method_registry);
            }

            jdk_internal_reflect_reflection::register(&mut method_registry);
            jdk_internal_vm_vmsupport::register(&mut method_registry);
            jdk_jfr_internal_jvm::register(&mut method_registry);
            jdk_net_macosxsocketoptions::register(&mut method_registry);
            jdk_vm_ci_runtime_jvmci::register(&mut method_registry);
            sun_awt_platformfont::register(&mut method_registry);
            sun_awt_suntoolkit::register(&mut method_registry);
            sun_java2d_osxoffscreensurfacedata::register(&mut method_registry);
            sun_java2d_surfacedata::register(&mut method_registry);
            sun_java2d_cmm_lcms_lcms::register(&mut method_registry);
            sun_java2d_opengl_oglrenderer::register(&mut method_registry);
            sun_nio_ch_pollselectorimpl::register(&mut method_registry);
            sun_rmi_transport_gc::register(&mut method_registry);
            sun_security_pkcs11_secmod::register(&mut method_registry);
            sun_security_pkcs11_wrapper_pkcs11::register(&mut method_registry);
            sun_tools_attach_virtualmachineimpl::register(&mut method_registry);
        }

        if java_version <= JAVA_17 {
            java_net_datagrampacket::register(&mut method_registry);
            java_net_plaindatagramsocketimpl::register(&mut method_registry);
            java_net_plainsocketimpl::register(&mut method_registry);
            java_net_socketinputstream::register(&mut method_registry);
            java_net_socketoutputstream::register(&mut method_registry);
        }
        if java_version == JAVA_17 {
            jdk_internal_foreign_abi_programmableinvoker::register(&mut method_registry);
            jdk_internal_foreign_abi_programmableupcallhandler::register(&mut method_registry);
            jdk_internal_invoke_nativeentrypoint::register(&mut method_registry);
        }
        if java_version >= JAVA_17 {
            java_lang_invoke_lambdaproxyclassarchive::register(&mut method_registry);
            java_lang_nullpointerexception::register(&mut method_registry);
            java_lang_ref_phantomreference::register(&mut method_registry);
            java_nio_mappedmemoryutils::register(&mut method_registry);
            jdk_internal_foreign_abi_upcallstubs::register(&mut method_registry);
            jdk_internal_loader_nativelibraries::register(&mut method_registry);
            jdk_internal_misc_cds::register(&mut method_registry);
            jdk_internal_misc_scopedmemoryaccess::register(&mut method_registry);
            jdk_internal_util_systemprops_raw::register(&mut method_registry);
            jdk_internal_vm_vector_vectorsupport::register(&mut method_registry);
            sun_awt_platformgraphicsinfo::register(&mut method_registry);
            sun_font_colorglyphsurfacedata::register(&mut method_registry);
            sun_java2d_metal_mtlgraphicsconfig::register(&mut method_registry);
            sun_java2d_metal_mtllayer::register(&mut method_registry);
            sun_java2d_metal_mtlmaskfill::register(&mut method_registry);
            sun_java2d_metal_mtlrenderqueue::register(&mut method_registry);
            sun_java2d_metal_mtlrenderer::register(&mut method_registry);
            sun_java2d_metal_mtlsurfacedata::register(&mut method_registry);
            sun_java2d_metal_mtltextrenderer::register(&mut method_registry);
            sun_nio_ch_nativesocketaddress::register(&mut method_registry);
            sun_nio_ch_socketdispatcher::register(&mut method_registry);
            sun_nio_ch_unixdomainsockets::register(&mut method_registry);
        }

        if java_version <= JAVA_18 {
            java_net_inetaddressimplfactory::register(&mut method_registry);
        }
        if java_version >= JAVA_18 {
            java_lang_ref_finalizer::register(&mut method_registry);
            jdk_internal_reflect_directconstructorhandleaccessor_nativeaccessor::register(
                &mut method_registry,
            );
            jdk_internal_reflect_directmethodhandleaccessor_nativeaccessor::register(
                &mut method_registry,
            );
        }

        if java_version <= JAVA_19 {
            sun_nio_ch_filechannelimpl::register(&mut method_registry);
            sun_nio_fs_unixcopyfile::register(&mut method_registry);
        }
        if java_version >= JAVA_19 {
            java_lang_virtualthread::register(&mut method_registry);
            jdk_internal_foreign_abi_nativeentrypoint::register(&mut method_registry);
            jdk_internal_foreign_abi_upcalllinker::register(&mut method_registry);
            jdk_internal_loader_nativelibrary::register(&mut method_registry);
            jdk_internal_loader_rawnativelibraries::register(&mut method_registry);
            jdk_internal_misc_previewfeatures::register(&mut method_registry);
            jdk_internal_vm_continuation::register(&mut method_registry);
            jdk_internal_vm_continuationsupport::register(&mut method_registry);
        }

        if java_version <= JAVA_20 {
            java_lang_strictmath::register(&mut method_registry);
        }
        if java_version >= JAVA_20 {
            sun_nio_ch_unixdispatcher::register(&mut method_registry);
            sun_nio_ch_unixfiledispatcherimpl::register(&mut method_registry);
            sun_nio_fs_bsdfilesystem::register(&mut method_registry);
            sun_nio_fs_unixfilesystem::register(&mut method_registry);
        }

        if java_version <= JAVA_21 {
            java_awt_button::register(&mut method_registry);
            java_awt_color::register(&mut method_registry);
            java_awt_filedialog::register(&mut method_registry);
            java_awt_keyboardfocusmanager::register(&mut method_registry);
            java_awt_menucomponent::register(&mut method_registry);
            java_awt_rectangle::register(&mut method_registry);
            java_awt_textfield::register(&mut method_registry);
            java_util_concurrent_atomic_atomiclong::register(&mut method_registry);
        }
        if java_version >= JAVA_21 {
            jdk_internal_foreign_abi_fallback_libfallback::register(&mut method_registry);
            jdk_internal_io_jdkconsoleimpl::register(&mut method_registry);
            jdk_internal_org_jline_terminal_impl_jna_osx_clibraryimpl::register(
                &mut method_registry,
            );
            jdk_internal_vm_foreignlinkersupport::register(&mut method_registry);
        }

        if java_version >= JAVA_22 {
            java_lang_stackframeinfo::register(&mut method_registry);
            jdk_vm_ci_services_services::register(&mut method_registry);
        }

        apple_laf_jrsuiconstants::register(&mut method_registry);
        apple_laf_jrsuicontrol::register(&mut method_registry);
        apple_laf_jrsuifocus::register(&mut method_registry);
        apple_laf_jrsuiutils_scrollbar::register(&mut method_registry);
        apple_security_keychainstore::register(&mut method_registry);
        com_apple_eawt_appdockiconhandler::register(&mut method_registry);
        com_apple_eawt_appeventhandler::register(&mut method_registry);
        com_apple_eawt_appmenubarhandler::register(&mut method_registry);
        com_apple_eawt_appmischandlers::register(&mut method_registry);
        com_apple_eio_filemanager::register(&mut method_registry);
        com_apple_laf_aquafileview::register(&mut method_registry);
        com_apple_laf_aquanativeresources::register(&mut method_registry);
        com_apple_laf_screenmenu::register(&mut method_registry);
        com_sun_imageio_plugins_jpeg_jpegimagereader::register(&mut method_registry);
        com_sun_imageio_plugins_jpeg_jpegimagewriter::register(&mut method_registry);
        com_sun_media_sound_directaudiodevice::register(&mut method_registry);
        com_sun_media_sound_directaudiodeviceprovider::register(&mut method_registry);
        com_sun_media_sound_midiindevice::register(&mut method_registry);
        com_sun_media_sound_midiindeviceprovider::register(&mut method_registry);
        com_sun_media_sound_midioutdevice::register(&mut method_registry);
        com_sun_media_sound_midioutdeviceprovider::register(&mut method_registry);
        com_sun_media_sound_platform::register(&mut method_registry);
        com_sun_media_sound_portmixer::register(&mut method_registry);
        com_sun_media_sound_portmixerprovider::register(&mut method_registry);
        com_sun_security_auth_module_unixsystem::register(&mut method_registry);
        java_awt_awtevent::register(&mut method_registry);
        java_awt_checkbox::register(&mut method_registry);
        java_awt_checkboxmenuitem::register(&mut method_registry);
        java_awt_choice::register(&mut method_registry);
        java_awt_component::register(&mut method_registry);
        java_awt_container::register(&mut method_registry);
        java_awt_cursor::register(&mut method_registry);
        java_awt_dialog::register(&mut method_registry);
        java_awt_dimension::register(&mut method_registry);
        java_awt_event::register(&mut method_registry);
        java_awt_font::register(&mut method_registry);
        java_awt_fontmetrics::register(&mut method_registry);
        java_awt_frame::register(&mut method_registry);
        java_awt_insets::register(&mut method_registry);
        java_awt_label::register(&mut method_registry);
        java_awt_menu::register(&mut method_registry);
        java_awt_menubar::register(&mut method_registry);
        java_awt_menuitem::register(&mut method_registry);
        java_awt_scrollpane::register(&mut method_registry);
        java_awt_scrollpaneadjustable::register(&mut method_registry);
        java_awt_splashscreen::register(&mut method_registry);
        java_awt_textarea::register(&mut method_registry);
        java_awt_toolkit::register(&mut method_registry);
        java_awt_trayicon::register(&mut method_registry);
        java_awt_window::register(&mut method_registry);
        java_io_console::register(&mut method_registry);
        java_io_filedescriptor::register(&mut method_registry);
        java_io_fileinputstream::register(&mut method_registry);
        java_io_fileoutputstream::register(&mut method_registry);
        java_io_objectstreamclass::register(&mut method_registry);
        java_io_randomaccessfile::register(&mut method_registry);
        java_io_unixfilesystem::register(&mut method_registry);
        java_io_winntfilesystem::register(&mut method_registry);
        java_lang_class::register(&mut method_registry);
        java_lang_classloader::register(&mut method_registry);
        java_lang_double::register(&mut method_registry);
        java_lang_float::register(&mut method_registry);
        java_lang_object::register(&mut method_registry);
        java_lang_processenvironment::register(&mut method_registry);
        java_lang_runtime::register(&mut method_registry);
        java_lang_securitymanager::register(&mut method_registry);
        java_lang_shutdown::register(&mut method_registry);
        java_lang_string::register(&mut method_registry);
        java_lang_system::register(&mut method_registry);
        java_lang_thread::register(&mut method_registry);
        java_lang_throwable::register(&mut method_registry);
        java_net_inet4address::register(&mut method_registry);
        java_net_inet4addressimpl::register(&mut method_registry);
        java_net_inet6address::register(&mut method_registry);
        java_net_inet6addressimpl::register(&mut method_registry);
        java_net_inetaddress::register(&mut method_registry);
        java_net_networkinterface::register(&mut method_registry);
        java_security_accesscontroller::register(&mut method_registry);
        java_util_prefs_filesystempreferences::register(&mut method_registry);
        java_util_prefs_macosxpreferencesfile::register(&mut method_registry);
        java_util_zip_adler32::register(&mut method_registry);
        java_util_zip_crc32::register(&mut method_registry);
        java_util_zip_deflater::register(&mut method_registry);
        java_util_zip_inflater::register(&mut method_registry);
        jdk_internal_module_modulebootstrap::register(&mut method_registry);
        sun_awt_cgraphicsdevice::register(&mut method_registry);
        sun_awt_cgraphicsenvironment::register(&mut method_registry);
        sun_awt_debugsettings::register(&mut method_registry);
        sun_awt_fontdescriptor::register(&mut method_registry);
        sun_awt_image_bufimgsurfacedata::register(&mut method_registry);
        sun_awt_image_bytecomponentraster::register(&mut method_registry);
        sun_awt_image_bytepackedraster::register(&mut method_registry);
        sun_awt_image_databuffernative::register(&mut method_registry);
        sun_awt_image_gifimagedecoder::register(&mut method_registry);
        sun_awt_image_imagerepresentation::register(&mut method_registry);
        sun_awt_image_imaginglib::register(&mut method_registry);
        sun_awt_image_integercomponentraster::register(&mut method_registry);
        sun_awt_image_jpegimagedecoder::register(&mut method_registry);
        sun_awt_image_shortcomponentraster::register(&mut method_registry);
        sun_font_cchartoglyphmapper::register(&mut method_registry);
        sun_font_cfont::register(&mut method_registry);
        sun_font_cfontmanager::register(&mut method_registry);
        sun_font_cstrike::register(&mut method_registry);
        sun_font_cstrikedisposer::register(&mut method_registry);
        sun_font_filefontstrike::register(&mut method_registry);
        sun_font_freetypefontscaler::register(&mut method_registry);
        sun_font_nullfontscaler::register(&mut method_registry);
        sun_font_strikecache::register(&mut method_registry);
        sun_font_sunfontmanager::register(&mut method_registry);
        sun_font_sunlayoutengine::register(&mut method_registry);
        sun_instrument_instrumentationimpl::register(&mut method_registry);
        sun_io_win32errormode::register(&mut method_registry);
        sun_java2d_crenderer::register(&mut method_registry);
        sun_java2d_defaultdisposerrecord::register(&mut method_registry);
        sun_java2d_disposer::register(&mut method_registry);
        sun_java2d_loops_blit::register(&mut method_registry);
        sun_java2d_loops_blitbg::register(&mut method_registry);
        sun_java2d_loops_drawglyphlist::register(&mut method_registry);
        sun_java2d_loops_drawglyphlistaa::register(&mut method_registry);
        sun_java2d_loops_drawglyphlistlcd::register(&mut method_registry);
        sun_java2d_loops_drawline::register(&mut method_registry);
        sun_java2d_loops_drawparallelogram::register(&mut method_registry);
        sun_java2d_loops_drawpath::register(&mut method_registry);
        sun_java2d_loops_drawpolygons::register(&mut method_registry);
        sun_java2d_loops_drawrect::register(&mut method_registry);
        sun_java2d_loops_fillparallelogram::register(&mut method_registry);
        sun_java2d_loops_fillpath::register(&mut method_registry);
        sun_java2d_loops_fillrect::register(&mut method_registry);
        sun_java2d_loops_fillspans::register(&mut method_registry);
        sun_java2d_loops_graphicsprimitivemgr::register(&mut method_registry);
        sun_java2d_loops_maskblit::register(&mut method_registry);
        sun_java2d_loops_maskfill::register(&mut method_registry);
        sun_java2d_loops_scaledblit::register(&mut method_registry);
        sun_java2d_loops_transformblit::register(&mut method_registry);
        sun_java2d_loops_transformhelper::register(&mut method_registry);
        sun_java2d_opengl_cglgraphicsconfig::register(&mut method_registry);
        sun_java2d_opengl_cgllayer::register(&mut method_registry);
        sun_java2d_opengl_cglsurfacedata::register(&mut method_registry);
        sun_java2d_opengl_oglcontext::register(&mut method_registry);
        sun_java2d_opengl_oglmaskfill::register(&mut method_registry);
        sun_java2d_opengl_oglrenderqueue::register(&mut method_registry);
        sun_java2d_opengl_oglsurfacedata::register(&mut method_registry);
        sun_java2d_opengl_ogltextrenderer::register(&mut method_registry);
        sun_java2d_pipe_bufferedmaskblit::register(&mut method_registry);
        sun_java2d_pipe_bufferedrenderpipe::register(&mut method_registry);
        sun_java2d_pipe_region::register(&mut method_registry);
        sun_java2d_pipe_shapespaniterator::register(&mut method_registry);
        sun_java2d_pipe_spancliprenderer::register(&mut method_registry);
        sun_lwawt_macosx_caccessibility::register(&mut method_registry);
        sun_lwawt_macosx_caccessible::register(&mut method_registry);
        sun_lwawt_macosx_ccheckboxmenuitem::register(&mut method_registry);
        sun_lwawt_macosx_cclipboard::register(&mut method_registry);
        sun_lwawt_macosx_ccursormanager::register(&mut method_registry);
        sun_lwawt_macosx_cdatatransferer::register(&mut method_registry);
        sun_lwawt_macosx_cdesktoppeer::register(&mut method_registry);
        sun_lwawt_macosx_cdragsourcecontextpeer::register(&mut method_registry);
        sun_lwawt_macosx_cdroptarget::register(&mut method_registry);
        sun_lwawt_macosx_cdroptargetcontextpeer::register(&mut method_registry);
        sun_lwawt_macosx_cfretainedresource::register(&mut method_registry);
        sun_lwawt_macosx_cfiledialog::register(&mut method_registry);
        sun_lwawt_macosx_cimage::register(&mut method_registry);
        sun_lwawt_macosx_cinputmethod::register(&mut method_registry);
        sun_lwawt_macosx_cinputmethoddescriptor::register(&mut method_registry);
        sun_lwawt_macosx_cmenu::register(&mut method_registry);
        sun_lwawt_macosx_cmenubar::register(&mut method_registry);
        sun_lwawt_macosx_cmenuitem::register(&mut method_registry);
        sun_lwawt_macosx_cplatformcomponent::register(&mut method_registry);
        sun_lwawt_macosx_cplatformview::register(&mut method_registry);
        sun_lwawt_macosx_cplatformwindow::register(&mut method_registry);
        sun_lwawt_macosx_cpopupmenu::register(&mut method_registry);
        sun_lwawt_macosx_cprinterjob::register(&mut method_registry);
        sun_lwawt_macosx_cprinterjobdialog::register(&mut method_registry);
        sun_lwawt_macosx_cprinterpagedialog::register(&mut method_registry);
        sun_lwawt_macosx_cprintersurfacedata::register(&mut method_registry);
        sun_lwawt_macosx_crobot::register(&mut method_registry);
        sun_lwawt_macosx_ctextpipe::register(&mut method_registry);
        sun_lwawt_macosx_ctrayicon::register(&mut method_registry);
        sun_lwawt_macosx_cwrapper_nsview::register(&mut method_registry);
        sun_lwawt_macosx_cwrapper_nswindow::register(&mut method_registry);
        sun_lwawt_macosx_lwctoolkit::register(&mut method_registry);
        sun_lwawt_macosx_nsevent::register(&mut method_registry);
        sun_management_classloadingimpl::register(&mut method_registry);
        sun_management_garbagecollectorimpl::register(&mut method_registry);
        sun_management_memoryimpl::register(&mut method_registry);
        sun_management_memorymanagerimpl::register(&mut method_registry);
        sun_management_memorypoolimpl::register(&mut method_registry);
        sun_management_threadimpl::register(&mut method_registry);
        sun_management_vmmanagementimpl::register(&mut method_registry);
        sun_net_portconfig::register(&mut method_registry);
        sun_net_dns_resolverconfigurationimpl::register(&mut method_registry);
        sun_net_sdp_sdpsupport::register(&mut method_registry);
        sun_net_spi_defaultproxyselector::register(&mut method_registry);
        sun_nio_ch_datagramchannelimpl::register(&mut method_registry);
        sun_nio_ch_datagramdispatcher::register(&mut method_registry);
        sun_nio_ch_filedispatcherimpl::register(&mut method_registry);
        sun_nio_ch_filekey::register(&mut method_registry);
        sun_nio_ch_ioutil::register(&mut method_registry);
        sun_nio_ch_inheritedchannel::register(&mut method_registry);
        sun_nio_ch_kqueue::register(&mut method_registry);
        sun_nio_ch_nativethread::register(&mut method_registry);
        sun_nio_ch_net::register(&mut method_registry);
        sun_nio_ch_unixasynchronoussocketchannelimpl::register(&mut method_registry);
        sun_nio_fs_bsdnativedispatcher::register(&mut method_registry);
        sun_nio_fs_macosxnativedispatcher::register(&mut method_registry);
        sun_nio_fs_utifiletypedetector::register(&mut method_registry);
        sun_nio_fs_unixnativedispatcher::register(&mut method_registry);
        sun_print_cupsprinter::register(&mut method_registry);
        sun_security_jgss_wrapper_gsslibstub::register(&mut method_registry);
        sun_security_krb5_config::register(&mut method_registry);
        sun_security_krb5_credentials::register(&mut method_registry);
        sun_security_krb5_scdynamicstoreconfig::register(&mut method_registry);
        sun_security_smartcardio_pcsc::register(&mut method_registry);
        sun_security_smartcardio_platformpcsc::register(&mut method_registry);
        sun_util_locale_provider_hostlocaleprovideradapterimpl::register(&mut method_registry);

        method_registry
    }

    /// Get the java version.
    pub fn java_version(&self) -> &Version {
        &self.java_version
    }

    /// Register a new Rust method.
    pub(crate) fn register(
        &mut self,
        class_name: &str,
        method_name: &str,
        method_descriptor: &str,
        method: RustMethod,
    ) {
        self.methods.insert(
            format!("{class_name}.{method_name}{method_descriptor}"),
            method,
        );
    }

    /// Get a Rust method by class and method name.
    ///
    /// # Errors
    /// if the method is not found.
    pub(crate) fn method(
        &self,
        class_name: &str,
        method_name: &str,
        method_descriptor: &str,
    ) -> Option<&RustMethod> {
        let method_signature = format!("{class_name}.{method_name}{method_descriptor}");
        self.methods.get(&method_signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_recursion::async_recursion;

    #[async_recursion(?Send)]
    async fn test_none(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
        Ok(None)
    }

    #[tokio::test]
    async fn test_register() -> Result<()> {
        let mut method_registry = MethodRegistry::new(&Version::Java21 { minor: 0 });
        let class_name = "java/lang/Object";
        let method_name = "foo";
        let method_descriptor = "()V";
        method_registry.register(class_name, method_name, method_descriptor, test_none);
        let result = method_registry.method(class_name, method_name, method_descriptor);
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_method() -> Result<()> {
        let method_registry = MethodRegistry::new(&Version::Java21 { minor: 0 });
        let result = method_registry.method("java/lang/Object", "hashCode", "()I");
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_method_not_found() -> Result<()> {
        let method_registry = MethodRegistry::new(&Version::Java21 { minor: 0 });
        let result = method_registry.method("foo", "hashCode", "()I");
        assert!(result.is_none());
        Ok(())
    }
}

#[cfg(target_os = "macos")]
use crate::native_methods::apple;
use crate::native_methods::{com, java, jdk, sun};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use ristretto_classloader::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub(crate) const JAVA_8: u16 = 8;
pub(crate) const JAVA_11: u16 = 11;
pub(crate) const JAVA_17: u16 = 17;
pub(crate) const JAVA_21: u16 = 21;
pub(crate) const JAVA_24: u16 = 24;

/// A Rust method is a method that is implemented in Rust and is called from Java code instead of
/// being implemented in Java byte code.
pub type RustMethod = fn(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>>;

#[derive(Debug, Default)]
pub struct MethodRegistry {
    java_major_version: u16,
    use_optimizations: bool,
    methods: HashMap<String, RustMethod>,
}

impl MethodRegistry {
    /// Create a new registry.
    pub fn new(java_major_version: u16, use_optimizations: bool) -> Self {
        MethodRegistry {
            java_major_version,
            use_optimizations,
            methods: HashMap::new(),
        }
    }

    /// Initialize the registry with all the native methods.
    #[expect(clippy::too_many_lines)]
    pub fn initialize(&mut self) {
        if self.java_major_version <= JAVA_8 {
            #[cfg(target_os = "macos")]
            {
                apple::applescript::applescriptengine::register(self);
                apple::applescript::applescriptenginefactory::register(self);
                apple::launcher::javaapplauncher::register(self);
                com::apple::concurrent::libdispatchnative::register(self);
                com::apple::laf::screenpopupfactory::register(self);
                sun::awt::cgraphicsconfig::register(self);
            }
            #[cfg(not(target_os = "windows"))]
            {
                com::sun::java::swing::plaf::gtk::gtkengine::register(self);
                com::sun::java::swing::plaf::gtk::gtkstyle::register(self);
                java::lang::unixprocess::register(self);
                sun::awt::fcfontmanager::register(self);
                sun::awt::unixtoolkit::register(self);
                sun::awt::x11graphicsconfig::register(self);
                sun::awt::x11graphicsdevice::register(self);
                sun::awt::x11graphicsenvironment::register(self);
                sun::awt::x11inputmethod::register(self);
                sun::font::fontconfigmanager::register(self);
                sun::font::nativefont::register(self);
                sun::font::nativestrike::register(self);
                sun::font::nativestrikedisposer::register(self);
                sun::font::x11textrenderer::register(self);
                sun::java2d::jules::julesaatilegenerator::register(self);
                sun::java2d::jules::julespathbuf::register(self);
                sun::java2d::opengl::glxgraphicsconfig::register(self);
                sun::java2d::opengl::glxsurfacedata::register(self);
                sun::java2d::x11::x11pmblitbgloops::register(self);
                sun::java2d::x11::x11pmblitloops::register(self);
                sun::java2d::x11::x11renderer::register(self);
                sun::java2d::x11::x11surfacedata::register(self);
                sun::java2d::x11::xsurfacedata::register(self);
                sun::java2d::xr::xidgenerator::register(self);
                sun::java2d::xr::xrbackendnative::register(self);
                sun::java2d::xr::xrmaskblit::register(self);
                sun::java2d::xr::xrmaskfill::register(self);
                sun::java2d::xr::xrsurfacedata::register(self);
                sun::management::operatingsystemimpl::register(self);
            }

            com::sun::demo::jvmti::hprof::tracker::register(self);
            java::awt::image::componentsamplemodel::register(self);
            java::lang::compiler::register(self);
            java::lang::package::register(self);
            java::lang::reflect::proxy::register(self);
            java::nio::bits::register(self);
            java::util::jar::jarfile::register(self);
            java::util::logging::filehandler::register(self);
            java::util::zip::zipfile::register(self);
            sun::awt::defaultmouseinfopeer::register(self);
            sun::management::diagnosticcommandimpl::register(self);
            sun::management::filesystemimpl::register(self);
            sun::management::flag::register(self);
            sun::management::gcinfobuilder::register(self);
            sun::misc::gc::register(self);
            sun::misc::messageutils::register(self);
            sun::misc::nativesignalhandler::register(self);
            sun::misc::perf::register(self);
            sun::misc::signal::register(self);
            sun::misc::r#unsafe::register(self);
            sun::misc::urlclasspath::register(self);
            sun::misc::version::register(self);
            sun::misc::vm::register(self);
            sun::misc::vmsupport::register(self);
            sun::net::extendedoptionsimpl::register(self);

            #[cfg(target_os = "macos")]
            {
                sun::nio::ch::kqueuearraywrapper::register(self);
                sun::nio::ch::kqueueport::register(self);
                sun::nio::ch::pollarraywrapper::register(self);
                sun::nio::ch::sctp::sctpnet::register(self);
            }

            sun::reflect::constantpool::register(self);
            sun::reflect::nativeconstructoraccessorimpl::register(self);
            sun::reflect::nativemethodaccessorimpl::register(self);
            sun::reflect::reflection::register(self);
            sun::tracing::dtrace::jvm::register(self);
        }

        if self.java_major_version <= JAVA_11 {
            #[cfg(not(target_os = "windows"))]
            {
                sun::nio::ch::unixasynchronousserversocketchannelimpl::register(self);
            }

            java::lang::classloader_nativelibrary::register(self);
            java::nio::mappedbytebuffer::register(self);
            sun::nio::ch::serversocketchannelimpl::register(self);
            sun::nio::ch::socketchannelimpl::register(self);
        }
        if self.java_major_version == JAVA_11 {
            java::lang::stringcoding::register(self);
            sun::security::ec::ecdhkeyagreement::register(self);
            sun::security::ec::ecdsasignature::register(self);
            sun::security::ec::eckeypairgenerator::register(self);
        }
        if self.java_major_version >= JAVA_11 {
            #[cfg(not(target_os = "windows"))]
            {
                com::sun::management::internal::operatingsystemimpl::register(self);
                java::lang::processimpl::register(self);
            }

            com::sun::management::internal::diagnosticcommandimpl::register(self);
            com::sun::management::internal::flag::register(self);
            com::sun::management::internal::garbagecollectorextimpl::register(self);
            com::sun::management::internal::gcinfobuilder::register(self);
            com::sun::security::auth::module::ntsystem::register(self);
            java::io::filecleanable::register(self);
            java::lang::module::register(self);
            java::lang::processhandleimpl::register(self);
            java::lang::processhandleimpl_info::register(self);
            java::lang::stackstreamfactory::register(self);
            java::lang::stackstreamfactory_abstractstackwalker::register(self);
            java::lang::stacktraceelement::register(self);
            java::lang::invoke::varhandle::register(self);
            java::lang::r#ref::reference::register(self);
            jdk::internal::agent::filesystemimpl::register(self);
            jdk::internal::jimage::nativeimagebuffer::register(self);
            jdk::internal::loader::bootloader::register(self);
            jdk::internal::misc::signal::register(self);
            jdk::internal::misc::r#unsafe::register(self);
            jdk::internal::misc::vm::register(self);
            jdk::internal::perf::perf::register(self);
            jdk::internal::reflect::constantpool::register(self);

            if self.java_major_version <= JAVA_21 {
                java::lang::stringutf16::register(self);
                jdk::internal::reflect::nativeconstructoraccessorimpl::register(self);
                jdk::internal::reflect::nativemethodaccessorimpl::register(self);
            }

            #[cfg(target_os = "macos")]
            {
                jdk::net::macosxsocketoptions::register(self);
                sun::nio::ch::pollselectorimpl::register(self);
                sun::nio::fs::utifiletypedetector::register(self);
                sun::tools::attach::virtualmachineimpl::register(self);
            }

            jdk::internal::reflect::reflection::register(self);
            jdk::internal::vm::vmsupport::register(self);
            jdk::jfr::internal::jvm::register(self);
            jdk::vm::ci::runtime::jvmci::register(self);
            sun::rmi::transport::gc::register(self);
            sun::security::pkcs11::secmod::register(self);
            sun::security::pkcs11::wrapper::pkcs11::register(self);
        }

        if self.java_major_version <= JAVA_17 {
            #[cfg(not(target_os = "windows"))]
            {
                java::net::plaindatagramsocketimpl::register(self);
                java::net::plainsocketimpl::register(self);
                sun::nio::fs::unixcopyfile::register(self);
            }
            java::lang::strictmath::register(self);
            java::net::datagrampacket::register(self);
            java::net::inetaddressimplfactory::register(self);
            java::net::socketinputstream::register(self);
            java::net::socketoutputstream::register(self);
            sun::nio::ch::filechannelimpl::register(self);
        }
        if self.java_major_version == JAVA_17 {
            jdk::internal::foreign::abi::programmableinvoker::register(self);
            jdk::internal::foreign::abi::programmableupcallhandler::register(self);
            jdk::internal::invoke::nativeentrypoint::register(self);
        }
        if self.java_major_version >= JAVA_17 {
            #[cfg(target_os = "macos")]
            {
                sun::awt::platformgraphicsinfo::register(self);
                sun::java2d::metal::mtlgraphicsconfig::register(self);
                sun::java2d::metal::mtllayer::register(self);
                sun::java2d::metal::mtlmaskfill::register(self);
                sun::java2d::metal::mtlrenderqueue::register(self);
                sun::java2d::metal::mtlrenderer::register(self);
                sun::java2d::metal::mtlsurfacedata::register(self);
                sun::java2d::metal::mtltextrenderer::register(self);
            }

            java::lang::invoke::lambdaproxyclassarchive::register(self);
            java::lang::nullpointerexception::register(self);
            java::lang::r#ref::phantomreference::register(self);
            java::nio::mappedmemoryutils::register(self);
            jdk::internal::foreign::abi::upcallstubs::register(self);
            jdk::internal::loader::nativelibraries::register(self);
            jdk::internal::misc::cds::register(self);
            jdk::internal::misc::scopedmemoryaccess::register(self);
            jdk::internal::util::systemprops_raw::register(self);
            jdk::internal::vm::vector::vectorsupport::register(self);
            sun::font::colorglyphsurfacedata::register(self);
            sun::nio::ch::nativesocketaddress::register(self);
            sun::nio::ch::socketdispatcher::register(self);
            sun::nio::ch::unixdomainsockets::register(self);
        }

        if self.java_major_version <= JAVA_21 {
            java::awt::button::register(self);
            java::awt::color::register(self);
            java::awt::filedialog::register(self);
            java::awt::keyboardfocusmanager::register(self);
            java::awt::menucomponent::register(self);
            java::awt::rectangle::register(self);
            java::awt::textfield::register(self);
            java::util::concurrent::atomic::atomiclong::register(self);
        }
        if self.java_major_version >= JAVA_21 {
            #[cfg(target_os = "macos")]
            {
                sun::nio::fs::bsdfilesystem::register(self);
            }

            #[cfg(not(target_os = "windows"))]
            {
                sun::nio::ch::unixdispatcher::register(self);
                sun::nio::ch::unixfiledispatcherimpl::register(self);
                sun::nio::fs::unixfilesystem::register(self);
            }

            java::lang::r#ref::finalizer::register(self);
            java::lang::virtualthread::register(self);
            jdk::internal::foreign::abi::nativeentrypoint::register(self);
            jdk::internal::foreign::abi::upcalllinker::register(self);
            jdk::internal::foreign::abi::fallback::libfallback::register(self);
            jdk::internal::io::jdkconsoleimpl::register(self);
            jdk::internal::loader::nativelibrary::register(self);
            jdk::internal::loader::rawnativelibraries::register(self);
            jdk::internal::misc::previewfeatures::register(self);
            jdk::internal::reflect::directconstructorhandleaccessor_nativeaccessor::register(self);
            jdk::internal::reflect::directmethodhandleaccessor_nativeaccessor::register(self);
            jdk::internal::vm::continuation::register(self);
            jdk::internal::vm::continuationsupport::register(self);
            jdk::internal::vm::foreignlinkersupport::register(self);
        }

        if self.java_major_version >= JAVA_24 {
            java::lang::stackframeinfo::register(self);
            jdk::vm::ci::services::services::register(self);
        }

        #[cfg(target_os = "macos")]
        {
            apple::laf::jrsuiconstants::register(self);
            apple::laf::jrsuicontrol::register(self);
            apple::laf::jrsuifocus::register(self);
            apple::laf::jrsuiutils_scrollbar::register(self);
            apple::security::keychainstore::register(self);
            com::apple::eawt::appdockiconhandler::register(self);
            com::apple::eawt::appeventhandler::register(self);
            com::apple::eawt::application::register(self);
            com::apple::eawt::appmenubarhandler::register(self);
            com::apple::eawt::appmischandlers::register(self);
            com::apple::eio::filemanager::register(self);
            com::apple::laf::aquafileview::register(self);
            com::apple::laf::aquanativeresources::register(self);
            com::apple::laf::screenmenu::register(self);
            java::util::prefs::macosxpreferencesfile::register(self);
            jdk::internal::org::jline::terminal::r#impl::jna::osx::clibraryimpl::register(self);
            sun::awt::cgraphicsdevice::register(self);
            sun::awt::cgraphicsenvironment::register(self);
            sun::font::cchartoglyphmapper::register(self);
            sun::font::cfont::register(self);
            sun::font::cfontmanager::register(self);
            sun::font::cstrike::register(self);
            sun::font::cstrikedisposer::register(self);
            sun::java2d::crenderer::register(self);
            sun::java2d::opengl::cglgraphicsconfig::register(self);
            sun::java2d::opengl::cgllayer::register(self);
            sun::java2d::opengl::cglsurfacedata::register(self);
            sun::java2d::osxoffscreensurfacedata::register(self);
            sun::java2d::surfacedata::register(self);
            sun::java2d::cmm::lcms::lcms::register(self);
            sun::java2d::opengl::oglrenderer::register(self);
            sun::lwawt::macosx::caccessibility::register(self);
            sun::lwawt::macosx::caccessible::register(self);
            sun::lwawt::macosx::ccheckboxmenuitem::register(self);
            sun::lwawt::macosx::cclipboard::register(self);
            sun::lwawt::macosx::ccursormanager::register(self);
            sun::lwawt::macosx::cdatatransferer::register(self);
            sun::lwawt::macosx::cdesktoppeer::register(self);
            sun::lwawt::macosx::cdragsourcecontextpeer::register(self);
            sun::lwawt::macosx::cdroptarget::register(self);
            sun::lwawt::macosx::cdroptargetcontextpeer::register(self);
            sun::lwawt::macosx::cfretainedresource::register(self);
            sun::lwawt::macosx::cfiledialog::register(self);
            sun::lwawt::macosx::cimage::register(self);
            sun::lwawt::macosx::cinputmethod::register(self);
            sun::lwawt::macosx::cinputmethoddescriptor::register(self);
            sun::lwawt::macosx::cmenu::register(self);
            sun::lwawt::macosx::cmenubar::register(self);
            sun::lwawt::macosx::cmenuitem::register(self);
            sun::lwawt::macosx::cplatformcomponent::register(self);
            sun::lwawt::macosx::cplatformview::register(self);
            sun::lwawt::macosx::cplatformwindow::register(self);
            sun::lwawt::macosx::cpopupmenu::register(self);
            sun::lwawt::macosx::cprinterjob::register(self);
            sun::lwawt::macosx::cprinterjobdialog::register(self);
            sun::lwawt::macosx::cprinterpagedialog::register(self);
            sun::lwawt::macosx::cprintersurfacedata::register(self);
            sun::lwawt::macosx::crobot::register(self);
            sun::lwawt::macosx::ctextpipe::register(self);
            sun::lwawt::macosx::ctrayicon::register(self);
            sun::lwawt::macosx::cwrapper_nsview::register(self);
            sun::lwawt::macosx::cwrapper_nswindow::register(self);
            sun::lwawt::macosx::lwctoolkit::register(self);
            sun::lwawt::macosx::nsevent::register(self);
            sun::nio::ch::kqueue::register(self);
            sun::nio::fs::bsdnativedispatcher::register(self);
            sun::nio::fs::macosxnativedispatcher::register(self);
            sun::util::locale::provider::hostlocaleprovideradapterimpl::register(self);
        }
        #[cfg(not(target_os = "windows"))]
        {
            java::io::unixfilesystem::register(self);
            java::lang::processenvironment::register(self);
            java::util::prefs::filesystempreferences::register(self);
            com::sun::security::auth::module::unixsystem::register(self);
            sun::net::dns::resolverconfigurationimpl::register(self);
            sun::net::portconfig::register(self);
            sun::nio::ch::datagramdispatcher::register(self);
            sun::nio::ch::inheritedchannel::register(self);
            sun::nio::ch::nativethread::register(self);
            sun::nio::ch::unixasynchronoussocketchannelimpl::register(self);
            sun::nio::fs::unixnativedispatcher::register(self);
            sun::print::cupsprinter::register(self);
            sun::security::smartcardio::platformpcsc::register(self);
        }
        #[cfg(target_os = "windows")]
        {
            java::io::winntfilesystem::register(self);
            sun::io::win32errormode::register(self);
        }

        com::sun::imageio::plugins::jpeg::jpegimagereader::register(self);
        com::sun::imageio::plugins::jpeg::jpegimagewriter::register(self);
        com::sun::java::util::jar::pack::nativeunpack::register(self);
        com::sun::media::sound::directaudiodevice::register(self);
        com::sun::media::sound::directaudiodeviceprovider::register(self);
        com::sun::media::sound::midiindevice::register(self);
        com::sun::media::sound::midiindeviceprovider::register(self);
        com::sun::media::sound::midioutdevice::register(self);
        com::sun::media::sound::midioutdeviceprovider::register(self);
        com::sun::media::sound::platform::register(self);
        com::sun::media::sound::portmixer::register(self);
        com::sun::media::sound::portmixerprovider::register(self);
        java::awt::event_mod::inputevent::register(self);
        java::awt::event_mod::keyevent::register(self);
        java::awt::event_mod::mouseevent::register(self);
        java::awt::awtevent::register(self);
        java::awt::checkbox::register(self);
        java::awt::checkboxmenuitem::register(self);
        java::awt::choice::register(self);
        java::awt::component::register(self);
        java::awt::container::register(self);
        java::awt::cursor::register(self);
        java::awt::dialog::register(self);
        java::awt::dimension::register(self);
        java::awt::event::register(self);
        java::awt::font::register(self);
        java::awt::fontmetrics::register(self);
        java::awt::frame::register(self);
        java::awt::insets::register(self);
        java::awt::label::register(self);
        java::awt::menu::register(self);
        java::awt::menubar::register(self);
        java::awt::menuitem::register(self);
        java::awt::scrollbar::register(self);
        java::awt::scrollpane::register(self);
        java::awt::scrollpaneadjustable::register(self);
        java::awt::splashscreen::register(self);
        java::awt::textarea::register(self);
        java::awt::toolkit::register(self);
        java::awt::trayicon::register(self);
        java::awt::window::register(self);
        java::awt::image::bufferedimage::register(self);
        java::awt::image::colormodel::register(self);
        java::awt::image::indexcolormodel::register(self);
        java::awt::image::kernel::register(self);
        java::awt::image::raster::register(self);
        java::awt::image::samplemodel::register(self);
        java::awt::image::singlepixelpackedsamplemodel::register(self);
        java::io::console::register(self);
        java::io::filedescriptor::register(self);
        java::io::fileinputstream::register(self);
        java::io::fileoutputstream::register(self);
        java::io::objectinputstream::register(self);
        java::io::objectoutputstream::register(self);
        java::io::objectstreamclass::register(self);
        java::io::randomaccessfile::register(self);
        java::lang::class::register(self);
        java::lang::classloader::register(self);
        java::lang::double::register(self);
        java::lang::float::register(self);
        java::lang::invoke::methodhandle::register(self);
        java::lang::invoke::methodhandlenatives::register(self);
        java::lang::object::register(self);
        java::lang::reflect::array::register(self);
        java::lang::reflect::executable::register(self);
        java::lang::reflect::field::register(self);
        java::lang::runtime::register(self);
        java::lang::securitymanager::register(self);
        java::lang::shutdown::register(self);
        java::lang::string::register(self);
        java::lang::system::register(self);
        java::lang::thread::register(self);
        java::lang::throwable::register(self);
        java::net::abstractplaindatagramsocketimpl::register(self);
        java::net::abstractplainsocketimpl::register(self);
        java::net::inet4address::register(self);
        java::net::inet4addressimpl::register(self);
        java::net::inet6address::register(self);
        java::net::inet6addressimpl::register(self);
        java::net::inetaddress::register(self);
        java::net::networkinterface::register(self);
        java::net::socketcleanable::register(self);
        java::security::accesscontroller::register(self);
        java::util::zip::adler32::register(self);
        java::util::zip::crc32::register(self);
        java::util::zip::deflater::register(self);
        java::util::zip::inflater::register(self);
        java::util::timezone::register(self);
        jdk::internal::module::modulebootstrap::register(self);
        sun::awt::debugsettings::register(self);
        sun::awt::fontdescriptor::register(self);
        sun::awt::platformfont::register(self);
        sun::awt::suntoolkit::register(self);
        sun::awt::image::bufimgsurfacedata::register(self);
        sun::awt::image::bytecomponentraster::register(self);
        sun::awt::image::bytepackedraster::register(self);
        sun::awt::image::databuffernative::register(self);
        sun::awt::image::gifimagedecoder::register(self);
        sun::awt::image::imagerepresentation::register(self);
        sun::awt::image::imaginglib::register(self);
        sun::awt::image::integercomponentraster::register(self);
        sun::awt::image::jpegimagedecoder::register(self);
        sun::awt::image::shortcomponentraster::register(self);
        sun::font::filefontstrike::register(self);
        sun::font::freetypefontscaler::register(self);
        sun::font::nullfontscaler::register(self);
        sun::font::strikecache::register(self);
        sun::font::sunfontmanager::register(self);
        sun::font::sunlayoutengine::register(self);
        sun::instrument::instrumentationimpl::register(self);
        sun::java2d::defaultdisposerrecord::register(self);
        sun::java2d::disposer::register(self);
        sun::java2d::loops::blit::register(self);
        sun::java2d::loops::blitbg::register(self);
        sun::java2d::loops::drawglyphlist::register(self);
        sun::java2d::loops::drawglyphlistaa::register(self);
        sun::java2d::loops::drawglyphlistlcd::register(self);
        sun::java2d::loops::drawline::register(self);
        sun::java2d::loops::drawparallelogram::register(self);
        sun::java2d::loops::drawpath::register(self);
        sun::java2d::loops::drawpolygons::register(self);
        sun::java2d::loops::drawrect::register(self);
        sun::java2d::loops::fillparallelogram::register(self);
        sun::java2d::loops::fillpath::register(self);
        sun::java2d::loops::fillrect::register(self);
        sun::java2d::loops::fillspans::register(self);
        sun::java2d::loops::graphicsprimitivemgr::register(self);
        sun::java2d::loops::maskblit::register(self);
        sun::java2d::loops::maskfill::register(self);
        sun::java2d::loops::scaledblit::register(self);
        sun::java2d::loops::transformblit::register(self);
        sun::java2d::loops::transformhelper::register(self);
        sun::java2d::opengl::oglcontext::register(self);
        sun::java2d::opengl::oglmaskfill::register(self);
        sun::java2d::opengl::oglrenderqueue::register(self);
        sun::java2d::opengl::oglsurfacedata::register(self);
        sun::java2d::opengl::ogltextrenderer::register(self);
        sun::java2d::pipe::bufferedmaskblit::register(self);
        sun::java2d::pipe::bufferedrenderpipe::register(self);
        sun::java2d::pipe::region::register(self);
        sun::java2d::pipe::shapespaniterator::register(self);
        sun::java2d::pipe::spancliprenderer::register(self);
        sun::management::classloadingimpl::register(self);
        sun::management::garbagecollectorimpl::register(self);
        sun::management::memoryimpl::register(self);
        sun::management::memorymanagerimpl::register(self);
        sun::management::memorypoolimpl::register(self);
        sun::management::threadimpl::register(self);
        sun::management::vmmanagementimpl::register(self);
        sun::net::sdp::sdpsupport::register(self);
        sun::net::spi::defaultproxyselector::register(self);
        sun::nio::ch::datagramchannelimpl::register(self);
        sun::nio::ch::filedispatcherimpl::register(self);
        sun::nio::ch::filekey::register(self);
        sun::nio::ch::ioutil::register(self);
        sun::nio::ch::net::register(self);
        sun::security::jgss::wrapper::gsslibstub::register(self);
        sun::security::krb5::config::register(self);
        sun::security::krb5::credentials::register(self);
        sun::security::krb5::scdynamicstoreconfig::register(self);
        sun::security::smartcardio::pcsc::register(self);

        if self.use_optimizations {
            java::lang::math::register(self);
        }
    }

    /// Determine if optimizations should be used.
    pub fn use_optimizations(&self) -> bool {
        self.use_optimizations
    }

    /// Get the java version.
    pub fn java_major_version(&self) -> u16 {
        self.java_major_version
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

    /// Return a map of all the registered Rust methods.
    pub(crate) fn methods(&self) -> &HashMap<String, RustMethod> {
        &self.methods
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
    use ristretto_classloader::runtime;

    #[tokio::test]
    async fn test_register() -> Result<()> {
        let mut method_registry = MethodRegistry::default();
        let class_name = "java/lang/Object";
        let method_name = "foo";
        let method_descriptor = "()V";
        method_registry.register(
            class_name,
            method_name,
            method_descriptor,
            java::lang::strictmath::abs_i,
        );
        let result = method_registry.method(class_name, method_name, method_descriptor);
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_method() -> Result<()> {
        let mut method_registry = MethodRegistry::new(JAVA_21, true);
        method_registry.initialize();
        let result = method_registry.method("java/lang/Object", "hashCode", "()I");
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_method_not_found() -> Result<()> {
        let mut method_registry = MethodRegistry::new(JAVA_21, true);
        method_registry.initialize();
        let result = method_registry.method("foo", "hashCode", "()I");
        assert!(result.is_none());
        Ok(())
    }

    /// Get all the native methods for a given Java runtime.
    async fn get_native_methods(version: &str) -> Result<Vec<String>> {
        let (_java_home, _java_version, class_loader) =
            runtime::version_class_loader(version).await?;
        let class_path = class_loader.class_path();
        let class_names = class_path.class_names().await?;
        let mut native_methods = Vec::new();
        for class_name in class_names {
            let lower_class_name = class_name.to_lowercase();
            // Skip GraalVM and Hotspot classes
            if lower_class_name.contains("graalvm") || lower_class_name.contains("hotspot") {
                continue;
            }

            let class = class_loader.load(&class_name).await?;
            for method in class.methods() {
                if method.is_native() {
                    let method_name = method.name();
                    let method_descriptor = method.descriptor();
                    native_methods.push(format!("{class_name}.{method_name}{method_descriptor}"));
                }
            }
        }
        native_methods.sort();
        Ok(native_methods)
    }

    /// Get all the non-optimization methods for a given Java version.
    async fn get_registry_methods(version: &str) -> Result<Vec<String>> {
        let version_major = version.split_once('.').unwrap_or_default().0;
        let java_major_version: u16 = version_major.parse()?;
        let mut method_registry = MethodRegistry::new(java_major_version, false);
        method_registry.initialize();
        let mut registry_methods = method_registry
            .methods()
            .keys()
            .cloned()
            .collect::<Vec<String>>();
        registry_methods.sort();
        Ok(registry_methods)
    }

    /// Verify that all the native methods are registered for a given runtime
    async fn test_runtime(version: &str) -> Result<()> {
        let native_methods = get_native_methods(version).await?;
        let registry_methods = get_registry_methods(version).await?;
        // Required methods for ristretto
        #[expect(unused_mut)]
        #[expect(clippy::useless_vec)]
        let mut required_methods = vec![
            "java/lang/ClassLoader.initSystemClassLoader()Ljava/lang/ClassLoader;".to_string(),
            "java/lang/System.allowSecurityManager()Z".to_string(),
            "java/lang/System.getSecurityManager()Ljava/lang/SecurityManager;".to_string(),
            "java/lang/System.setSecurityManager(Ljava/lang/SecurityManager;)V".to_string(),
            "jdk/internal/module/ModuleBootstrap.boot()Ljava/lang/ModuleLayer;".to_string(),
        ];
        #[cfg(target_os = "windows")]
        {
            required_methods.push("java/io/WinNTFileSystem.initIDs()V".to_string());
            required_methods.push("sun/io/Win32ErrorMode.setErrorMode(J)J".to_string());
        }

        let missing_required_methods = required_methods
            .iter()
            .filter(|method| !registry_methods.contains(method))
            .cloned()
            .collect::<Vec<String>>();
        #[cfg(target_os = "macos")]
        let missing_methods = native_methods
            .iter()
            .filter(|method| !registry_methods.contains(method))
            .cloned()
            .collect::<Vec<String>>();
        let extra_methods = registry_methods
            .iter()
            .filter(|method| !native_methods.contains(method) && !required_methods.contains(method))
            .cloned()
            .collect::<Vec<String>>();

        let mut errors = Vec::new();
        if !missing_required_methods.is_empty() {
            errors.push(format!(
                "Missing required methods {}:\n{}\n",
                missing_required_methods.len(),
                missing_required_methods.join("\n"),
            ));
        };
        #[cfg(target_os = "macos")]
        if !missing_methods.is_empty() {
            errors.push(format!(
                "Missing methods {}:\n{}\n",
                missing_methods.len(),
                missing_methods.join("\n"),
            ));
        };
        if !extra_methods.is_empty() {
            errors.push(format!(
                "Extra methods {}:\n{}\n",
                extra_methods.len(),
                extra_methods.join("\n"),
            ));
        };
        let errors = errors.join("\n");
        assert_eq!("", errors);
        Ok(())
    }

    #[tokio::test]
    async fn test_runtime_v8() -> Result<()> {
        test_runtime("8.442.06.1").await
    }

    #[tokio::test]
    async fn test_runtime_v11() -> Result<()> {
        test_runtime("11.0.26.4.1").await
    }

    #[tokio::test]
    async fn test_runtime_v17() -> Result<()> {
        test_runtime("17.0.14.7.1").await
    }

    #[tokio::test]
    async fn test_runtime_v21() -> Result<()> {
        test_runtime("21.0.6.7.1").await
    }

    #[tokio::test]
    async fn test_runtime_v24() -> Result<()> {
        test_runtime("24.0.0.36.1").await
    }
}

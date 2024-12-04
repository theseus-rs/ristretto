use crate::arguments::Arguments;
use crate::native_methods::{apple, com, java, jdk, sun};
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
            apple::applescript::applescriptengine::register(&mut method_registry);
            apple::applescript::applescriptenginefactory::register(&mut method_registry);
            apple::launcher::javaapplauncher::register(&mut method_registry);
            com::apple::concurrent::libdispatchnative::register(&mut method_registry);
            com::apple::laf::screenpopupfactory::register(&mut method_registry);
            com::sun::demo::jvmti::hprof::tracker::register(&mut method_registry);
            com::sun::java::swing::plaf::gtk::gtkengine::register(&mut method_registry);
            com::sun::java::swing::plaf::gtk::gtkstyle::register(&mut method_registry);
            java::awt::image::componentsamplemodel::register(&mut method_registry);
            java::lang::compiler::register(&mut method_registry);
            java::lang::package::register(&mut method_registry);
            java::lang::reflect::proxy::register(&mut method_registry);
            java::lang::unixprocess::register(&mut method_registry);
            java::nio::bits::register(&mut method_registry);
            java::util::jar::jarfile::register(&mut method_registry);
            java::util::logging::filehandler::register(&mut method_registry);
            java::util::zip::zipfile::register(&mut method_registry);
            sun::awt::cgraphicsconfig::register(&mut method_registry);
            sun::awt::defaultmouseinfopeer::register(&mut method_registry);
            sun::awt::fcfontmanager::register(&mut method_registry);
            sun::awt::unixtoolkit::register(&mut method_registry);
            sun::awt::x11graphicsconfig::register(&mut method_registry);
            sun::awt::x11graphicsdevice::register(&mut method_registry);
            sun::awt::x11graphicsenvironment::register(&mut method_registry);
            sun::awt::x11inputmethod::register(&mut method_registry);
            sun::font::fontconfigmanager::register(&mut method_registry);
            sun::font::nativefont::register(&mut method_registry);
            sun::font::nativestrike::register(&mut method_registry);
            sun::font::nativestrikedisposer::register(&mut method_registry);
            sun::font::x11textrenderer::register(&mut method_registry);
            sun::java2d::jules::julesaatilegenerator::register(&mut method_registry);
            sun::java2d::jules::julespathbuf::register(&mut method_registry);
            sun::java2d::opengl::glxgraphicsconfig::register(&mut method_registry);
            sun::java2d::opengl::glxsurfacedata::register(&mut method_registry);
            sun::java2d::x11::x11pmblitbgloops::register(&mut method_registry);
            sun::java2d::x11::x11pmblitloops::register(&mut method_registry);
            sun::java2d::x11::x11renderer::register(&mut method_registry);
            sun::java2d::x11::x11surfacedata::register(&mut method_registry);
            sun::java2d::x11::xsurfacedata::register(&mut method_registry);
            sun::java2d::xr::xidgenerator::register(&mut method_registry);
            sun::java2d::xr::xrbackendnative::register(&mut method_registry);
            sun::java2d::xr::xrmaskblit::register(&mut method_registry);
            sun::java2d::xr::xrmaskfill::register(&mut method_registry);
            sun::java2d::xr::xrsurfacedata::register(&mut method_registry);
            sun::management::diagnosticcommandimpl::register(&mut method_registry);
            sun::management::filesystemimpl::register(&mut method_registry);
            sun::management::flag::register(&mut method_registry);
            sun::management::gcinfobuilder::register(&mut method_registry);
            sun::management::operatingsystemimpl::register(&mut method_registry);
            sun::misc::gc::register(&mut method_registry);
            sun::misc::messageutils::register(&mut method_registry);
            sun::misc::nativesignalhandler::register(&mut method_registry);
            sun::misc::perf::register(&mut method_registry);
            sun::misc::signal::register(&mut method_registry);
            sun::misc::r#unsafe::register(&mut method_registry);
            sun::misc::urlclasspath::register(&mut method_registry);
            sun::misc::version::register(&mut method_registry);
            sun::misc::vm::register(&mut method_registry);
            sun::misc::vmsupport::register(&mut method_registry);
            sun::net::extendedoptionsimpl::register(&mut method_registry);
            sun::nio::ch::kqueuearraywrapper::register(&mut method_registry);
            sun::nio::ch::kqueueport::register(&mut method_registry);
            sun::nio::ch::pollarraywrapper::register(&mut method_registry);
            sun::nio::ch::sctp::sctpnet::register(&mut method_registry);
            sun::reflect::constantpool::register(&mut method_registry);
            sun::reflect::nativeconstructoraccessorimpl::register(&mut method_registry);
            sun::reflect::nativemethodaccessorimpl::register(&mut method_registry);
            sun::reflect::reflection::register(&mut method_registry);
            sun::tracing::dtrace::jvm::register(&mut method_registry);
        }

        if java_version == JAVA_11 {
            com::sun::java::util::jar::pack::nativeunpack::register(&mut method_registry);
            java::io::objectinputstream::register(&mut method_registry);
            java::io::objectoutputstream::register(&mut method_registry);
            java::lang::classloader_nativelibrary::register(&mut method_registry);
            java::lang::stringcoding::register(&mut method_registry);
            java::net::abstractplaindatagramsocketimpl::register(&mut method_registry);
            java::net::abstractplainsocketimpl::register(&mut method_registry);
            java::net::socketcleanable::register(&mut method_registry);
            java::nio::mappedbytebuffer::register(&mut method_registry);
            sun::nio::ch::serversocketchannelimpl::register(&mut method_registry);
            sun::nio::ch::socketchannelimpl::register(&mut method_registry);
            sun::nio::ch::unixasynchronousserversocketchannelimpl::register(&mut method_registry);
            sun::security::ec::ecdhkeyagreement::register(&mut method_registry);
            sun::security::ec::ecdsasignature::register(&mut method_registry);
            sun::security::ec::eckeypairgenerator::register(&mut method_registry);
        }
        if java_version >= JAVA_11 {
            com::apple::eawt::application::register(&mut method_registry);
            com::sun::management::internal::diagnosticcommandimpl::register(&mut method_registry);
            com::sun::management::internal::flag::register(&mut method_registry);
            com::sun::management::internal::garbagecollectorextimpl::register(&mut method_registry);
            com::sun::management::internal::gcinfobuilder::register(&mut method_registry);
            com::sun::management::internal::operatingsystemimpl::register(&mut method_registry);
            com::sun::security::auth::module::ntsystem::register(&mut method_registry);
            java::awt::scrollbar::register(&mut method_registry);
            java::awt::event_mod::inputevent::register(&mut method_registry);
            java::awt::event_mod::keyevent::register(&mut method_registry);
            java::awt::event_mod::mouseevent::register(&mut method_registry);
            java::awt::image::bufferedimage::register(&mut method_registry);
            java::awt::image::colormodel::register(&mut method_registry);
            java::awt::image::indexcolormodel::register(&mut method_registry);
            java::awt::image::kernel::register(&mut method_registry);
            java::awt::image::raster::register(&mut method_registry);
            java::awt::image::samplemodel::register(&mut method_registry);
            java::awt::image::singlepixelpackedsamplemodel::register(&mut method_registry);
            java::io::filecleanable::register(&mut method_registry);
            java::lang::module::register(&mut method_registry);
            java::lang::processhandleimpl::register(&mut method_registry);
            java::lang::processhandleimpl_info::register(&mut method_registry);
            java::lang::processimpl::register(&mut method_registry);
            java::lang::stackstreamfactory::register(&mut method_registry);
            java::lang::stackstreamfactory_abstractstackwalker::register(&mut method_registry);
            java::lang::stacktraceelement::register(&mut method_registry);
            java::lang::stringutf16::register(&mut method_registry);
            java::lang::invoke::methodhandle::register(&mut method_registry);
            java::lang::invoke::methodhandlenatives::register(&mut method_registry);
            java::lang::invoke::varhandle::register(&mut method_registry);
            java::lang::r#ref::reference::register(&mut method_registry);
            java::lang::reflect::array::register(&mut method_registry);
            java::lang::reflect::executable::register(&mut method_registry);
            java::lang::reflect::field::register(&mut method_registry);
            java::util::timezone::register(&mut method_registry);
            jdk::internal::agent::filesystemimpl::register(&mut method_registry);
            jdk::internal::jimage::nativeimagebuffer::register(&mut method_registry);
            jdk::internal::loader::bootloader::register(&mut method_registry);
            jdk::internal::misc::signal::register(&mut method_registry);
            jdk::internal::misc::r#unsafe::register(&mut method_registry);
            jdk::internal::misc::vm::register(&mut method_registry);
            jdk::internal::perf::perf::register(&mut method_registry);
            jdk::internal::reflect::constantpool::register(&mut method_registry);

            if java_version <= JAVA_22 {
                jdk::internal::reflect::nativeconstructoraccessorimpl::register(
                    &mut method_registry,
                );
                jdk::internal::reflect::nativemethodaccessorimpl::register(&mut method_registry);
            }

            jdk::internal::reflect::reflection::register(&mut method_registry);
            jdk::internal::vm::vmsupport::register(&mut method_registry);
            jdk::jfr::internal::jvm::register(&mut method_registry);
            jdk::net::macosxsocketoptions::register(&mut method_registry);
            jdk::vm::ci::runtime::jvmci::register(&mut method_registry);
            sun::awt::platformfont::register(&mut method_registry);
            sun::awt::suntoolkit::register(&mut method_registry);
            sun::java2d::osxoffscreensurfacedata::register(&mut method_registry);
            sun::java2d::surfacedata::register(&mut method_registry);
            sun::java2d::cmm::lcms::lcms::register(&mut method_registry);
            sun::java2d::opengl::oglrenderer::register(&mut method_registry);
            sun::nio::ch::pollselectorimpl::register(&mut method_registry);
            sun::rmi::transport::gc::register(&mut method_registry);
            sun::security::pkcs11::secmod::register(&mut method_registry);
            sun::security::pkcs11::wrapper::pkcs11::register(&mut method_registry);
            sun::tools::attach::virtualmachineimpl::register(&mut method_registry);
        }

        if java_version <= JAVA_17 {
            java::net::datagrampacket::register(&mut method_registry);
            java::net::plaindatagramsocketimpl::register(&mut method_registry);
            java::net::plainsocketimpl::register(&mut method_registry);
            java::net::socketinputstream::register(&mut method_registry);
            java::net::socketoutputstream::register(&mut method_registry);
        }
        if java_version == JAVA_17 {
            jdk::internal::foreign::abi::programmableinvoker::register(&mut method_registry);
            jdk::internal::foreign::abi::programmableupcallhandler::register(&mut method_registry);
            jdk::internal::invoke::nativeentrypoint::register(&mut method_registry);
        }
        if java_version >= JAVA_17 {
            java::lang::invoke::lambdaproxyclassarchive::register(&mut method_registry);
            java::lang::nullpointerexception::register(&mut method_registry);
            java::lang::r#ref::phantomreference::register(&mut method_registry);
            java::nio::mappedmemoryutils::register(&mut method_registry);
            jdk::internal::foreign::abi::upcallstubs::register(&mut method_registry);
            jdk::internal::loader::nativelibraries::register(&mut method_registry);
            jdk::internal::misc::cds::register(&mut method_registry);
            jdk::internal::misc::scopedmemoryaccess::register(&mut method_registry);
            jdk::internal::util::systemprops_raw::register(&mut method_registry);
            jdk::internal::vm::vector::vectorsupport::register(&mut method_registry);
            sun::awt::platformgraphicsinfo::register(&mut method_registry);
            sun::font::colorglyphsurfacedata::register(&mut method_registry);
            sun::java2d::metal::mtlgraphicsconfig::register(&mut method_registry);
            sun::java2d::metal::mtllayer::register(&mut method_registry);
            sun::java2d::metal::mtlmaskfill::register(&mut method_registry);
            sun::java2d::metal::mtlrenderqueue::register(&mut method_registry);
            sun::java2d::metal::mtlrenderer::register(&mut method_registry);
            sun::java2d::metal::mtlsurfacedata::register(&mut method_registry);
            sun::java2d::metal::mtltextrenderer::register(&mut method_registry);
            sun::nio::ch::nativesocketaddress::register(&mut method_registry);
            sun::nio::ch::socketdispatcher::register(&mut method_registry);
            sun::nio::ch::unixdomainsockets::register(&mut method_registry);
        }

        if java_version <= JAVA_18 {
            java::net::inetaddressimplfactory::register(&mut method_registry);
        }
        if java_version >= JAVA_18 {
            java::lang::r#ref::finalizer::register(&mut method_registry);
            jdk::internal::reflect::directconstructorhandleaccessor_nativeaccessor::register(
                &mut method_registry,
            );
            jdk::internal::reflect::directmethodhandleaccessor_nativeaccessor::register(
                &mut method_registry,
            );
        }

        if java_version <= JAVA_19 {
            sun::nio::ch::filechannelimpl::register(&mut method_registry);
            sun::nio::fs::unixcopyfile::register(&mut method_registry);
        }
        if java_version >= JAVA_19 {
            java::lang::virtualthread::register(&mut method_registry);
            jdk::internal::foreign::abi::nativeentrypoint::register(&mut method_registry);
            jdk::internal::foreign::abi::upcalllinker::register(&mut method_registry);
            jdk::internal::loader::nativelibrary::register(&mut method_registry);
            jdk::internal::loader::rawnativelibraries::register(&mut method_registry);
            jdk::internal::misc::previewfeatures::register(&mut method_registry);
            jdk::internal::vm::continuation::register(&mut method_registry);
            jdk::internal::vm::continuationsupport::register(&mut method_registry);
        }

        if java_version <= JAVA_20 {
            java::lang::strictmath::register(&mut method_registry);
        }
        if java_version >= JAVA_20 {
            sun::nio::ch::unixdispatcher::register(&mut method_registry);
            sun::nio::ch::unixfiledispatcherimpl::register(&mut method_registry);
            sun::nio::fs::bsdfilesystem::register(&mut method_registry);
            sun::nio::fs::unixfilesystem::register(&mut method_registry);
        }

        if java_version <= JAVA_21 {
            java::awt::button::register(&mut method_registry);
            java::awt::color::register(&mut method_registry);
            java::awt::filedialog::register(&mut method_registry);
            java::awt::keyboardfocusmanager::register(&mut method_registry);
            java::awt::menucomponent::register(&mut method_registry);
            java::awt::rectangle::register(&mut method_registry);
            java::awt::textfield::register(&mut method_registry);
            java::util::concurrent::atomic::atomiclong::register(&mut method_registry);
        }
        if java_version >= JAVA_21 {
            jdk::internal::foreign::abi::fallback::libfallback::register(&mut method_registry);
            jdk::internal::io::jdkconsoleimpl::register(&mut method_registry);
            jdk::internal::org::jline::terminal::r#impl::jna::osx::clibraryimpl::register(
                &mut method_registry,
            );
            jdk::internal::vm::foreignlinkersupport::register(&mut method_registry);
        }

        if java_version >= JAVA_22 {
            java::lang::stackframeinfo::register(&mut method_registry);
            jdk::vm::ci::services::services::register(&mut method_registry);
        }

        apple::laf::jrsuiconstants::register(&mut method_registry);
        apple::laf::jrsuicontrol::register(&mut method_registry);
        apple::laf::jrsuifocus::register(&mut method_registry);
        apple::laf::jrsuiutils_scrollbar::register(&mut method_registry);
        apple::security::keychainstore::register(&mut method_registry);
        com::apple::eawt::appdockiconhandler::register(&mut method_registry);
        com::apple::eawt::appeventhandler::register(&mut method_registry);
        com::apple::eawt::appmenubarhandler::register(&mut method_registry);
        com::apple::eawt::appmischandlers::register(&mut method_registry);
        com::apple::eio::filemanager::register(&mut method_registry);
        com::apple::laf::aquafileview::register(&mut method_registry);
        com::apple::laf::aquanativeresources::register(&mut method_registry);
        com::apple::laf::screenmenu::register(&mut method_registry);
        com::sun::imageio::plugins::jpeg::jpegimagereader::register(&mut method_registry);
        com::sun::imageio::plugins::jpeg::jpegimagewriter::register(&mut method_registry);
        com::sun::media::sound::directaudiodevice::register(&mut method_registry);
        com::sun::media::sound::directaudiodeviceprovider::register(&mut method_registry);
        com::sun::media::sound::midiindevice::register(&mut method_registry);
        com::sun::media::sound::midiindeviceprovider::register(&mut method_registry);
        com::sun::media::sound::midioutdevice::register(&mut method_registry);
        com::sun::media::sound::midioutdeviceprovider::register(&mut method_registry);
        com::sun::media::sound::platform::register(&mut method_registry);
        com::sun::media::sound::portmixer::register(&mut method_registry);
        com::sun::media::sound::portmixerprovider::register(&mut method_registry);
        com::sun::security::auth::module::unixsystem::register(&mut method_registry);
        java::awt::awtevent::register(&mut method_registry);
        java::awt::checkbox::register(&mut method_registry);
        java::awt::checkboxmenuitem::register(&mut method_registry);
        java::awt::choice::register(&mut method_registry);
        java::awt::component::register(&mut method_registry);
        java::awt::container::register(&mut method_registry);
        java::awt::cursor::register(&mut method_registry);
        java::awt::dialog::register(&mut method_registry);
        java::awt::dimension::register(&mut method_registry);
        java::awt::event::register(&mut method_registry);
        java::awt::font::register(&mut method_registry);
        java::awt::fontmetrics::register(&mut method_registry);
        java::awt::frame::register(&mut method_registry);
        java::awt::insets::register(&mut method_registry);
        java::awt::label::register(&mut method_registry);
        java::awt::menu::register(&mut method_registry);
        java::awt::menubar::register(&mut method_registry);
        java::awt::menuitem::register(&mut method_registry);
        java::awt::scrollpane::register(&mut method_registry);
        java::awt::scrollpaneadjustable::register(&mut method_registry);
        java::awt::splashscreen::register(&mut method_registry);
        java::awt::textarea::register(&mut method_registry);
        java::awt::toolkit::register(&mut method_registry);
        java::awt::trayicon::register(&mut method_registry);
        java::awt::window::register(&mut method_registry);
        java::io::console::register(&mut method_registry);
        java::io::filedescriptor::register(&mut method_registry);
        java::io::fileinputstream::register(&mut method_registry);
        java::io::fileoutputstream::register(&mut method_registry);
        java::io::objectstreamclass::register(&mut method_registry);
        java::io::randomaccessfile::register(&mut method_registry);
        java::io::unixfilesystem::register(&mut method_registry);
        java::io::winntfilesystem::register(&mut method_registry);
        java::lang::class::register(&mut method_registry);
        java::lang::classloader::register(&mut method_registry);
        java::lang::double::register(&mut method_registry);
        java::lang::float::register(&mut method_registry);
        java::lang::object::register(&mut method_registry);
        java::lang::processenvironment::register(&mut method_registry);
        java::lang::runtime::register(&mut method_registry);
        java::lang::securitymanager::register(&mut method_registry);
        java::lang::shutdown::register(&mut method_registry);
        java::lang::string::register(&mut method_registry);
        java::lang::system::register(&mut method_registry);
        java::lang::thread::register(&mut method_registry);
        java::lang::throwable::register(&mut method_registry);
        java::net::inet4address::register(&mut method_registry);
        java::net::inet4addressimpl::register(&mut method_registry);
        java::net::inet6address::register(&mut method_registry);
        java::net::inet6addressimpl::register(&mut method_registry);
        java::net::inetaddress::register(&mut method_registry);
        java::net::networkinterface::register(&mut method_registry);
        java::security::accesscontroller::register(&mut method_registry);
        java::util::prefs::filesystempreferences::register(&mut method_registry);
        java::util::prefs::macosxpreferencesfile::register(&mut method_registry);
        java::util::zip::adler32::register(&mut method_registry);
        java::util::zip::crc32::register(&mut method_registry);
        java::util::zip::deflater::register(&mut method_registry);
        java::util::zip::inflater::register(&mut method_registry);
        jdk::internal::module::modulebootstrap::register(&mut method_registry);
        sun::awt::cgraphicsdevice::register(&mut method_registry);
        sun::awt::cgraphicsenvironment::register(&mut method_registry);
        sun::awt::debugsettings::register(&mut method_registry);
        sun::awt::fontdescriptor::register(&mut method_registry);
        sun::awt::image::bufimgsurfacedata::register(&mut method_registry);
        sun::awt::image::bytecomponentraster::register(&mut method_registry);
        sun::awt::image::bytepackedraster::register(&mut method_registry);
        sun::awt::image::databuffernative::register(&mut method_registry);
        sun::awt::image::gifimagedecoder::register(&mut method_registry);
        sun::awt::image::imagerepresentation::register(&mut method_registry);
        sun::awt::image::imaginglib::register(&mut method_registry);
        sun::awt::image::integercomponentraster::register(&mut method_registry);
        sun::awt::image::jpegimagedecoder::register(&mut method_registry);
        sun::awt::image::shortcomponentraster::register(&mut method_registry);
        sun::font::cchartoglyphmapper::register(&mut method_registry);
        sun::font::cfont::register(&mut method_registry);
        sun::font::cfontmanager::register(&mut method_registry);
        sun::font::cstrike::register(&mut method_registry);
        sun::font::cstrikedisposer::register(&mut method_registry);
        sun::font::filefontstrike::register(&mut method_registry);
        sun::font::freetypefontscaler::register(&mut method_registry);
        sun::font::nullfontscaler::register(&mut method_registry);
        sun::font::strikecache::register(&mut method_registry);
        sun::font::sunfontmanager::register(&mut method_registry);
        sun::font::sunlayoutengine::register(&mut method_registry);
        sun::instrument::instrumentationimpl::register(&mut method_registry);
        sun::io::win32errormode::register(&mut method_registry);
        sun::java2d::crenderer::register(&mut method_registry);
        sun::java2d::defaultdisposerrecord::register(&mut method_registry);
        sun::java2d::disposer::register(&mut method_registry);
        sun::java2d::loops::blit::register(&mut method_registry);
        sun::java2d::loops::blitbg::register(&mut method_registry);
        sun::java2d::loops::drawglyphlist::register(&mut method_registry);
        sun::java2d::loops::drawglyphlistaa::register(&mut method_registry);
        sun::java2d::loops::drawglyphlistlcd::register(&mut method_registry);
        sun::java2d::loops::drawline::register(&mut method_registry);
        sun::java2d::loops::drawparallelogram::register(&mut method_registry);
        sun::java2d::loops::drawpath::register(&mut method_registry);
        sun::java2d::loops::drawpolygons::register(&mut method_registry);
        sun::java2d::loops::drawrect::register(&mut method_registry);
        sun::java2d::loops::fillparallelogram::register(&mut method_registry);
        sun::java2d::loops::fillpath::register(&mut method_registry);
        sun::java2d::loops::fillrect::register(&mut method_registry);
        sun::java2d::loops::fillspans::register(&mut method_registry);
        sun::java2d::loops::graphicsprimitivemgr::register(&mut method_registry);
        sun::java2d::loops::maskblit::register(&mut method_registry);
        sun::java2d::loops::maskfill::register(&mut method_registry);
        sun::java2d::loops::scaledblit::register(&mut method_registry);
        sun::java2d::loops::transformblit::register(&mut method_registry);
        sun::java2d::loops::transformhelper::register(&mut method_registry);
        sun::java2d::opengl::cglgraphicsconfig::register(&mut method_registry);
        sun::java2d::opengl::cgllayer::register(&mut method_registry);
        sun::java2d::opengl::cglsurfacedata::register(&mut method_registry);
        sun::java2d::opengl::oglcontext::register(&mut method_registry);
        sun::java2d::opengl::oglmaskfill::register(&mut method_registry);
        sun::java2d::opengl::oglrenderqueue::register(&mut method_registry);
        sun::java2d::opengl::oglsurfacedata::register(&mut method_registry);
        sun::java2d::opengl::ogltextrenderer::register(&mut method_registry);
        sun::java2d::pipe::bufferedmaskblit::register(&mut method_registry);
        sun::java2d::pipe::bufferedrenderpipe::register(&mut method_registry);
        sun::java2d::pipe::region::register(&mut method_registry);
        sun::java2d::pipe::shapespaniterator::register(&mut method_registry);
        sun::java2d::pipe::spancliprenderer::register(&mut method_registry);
        sun::lwawt::macosx::caccessibility::register(&mut method_registry);
        sun::lwawt::macosx::caccessible::register(&mut method_registry);
        sun::lwawt::macosx::ccheckboxmenuitem::register(&mut method_registry);
        sun::lwawt::macosx::cclipboard::register(&mut method_registry);
        sun::lwawt::macosx::ccursormanager::register(&mut method_registry);
        sun::lwawt::macosx::cdatatransferer::register(&mut method_registry);
        sun::lwawt::macosx::cdesktoppeer::register(&mut method_registry);
        sun::lwawt::macosx::cdragsourcecontextpeer::register(&mut method_registry);
        sun::lwawt::macosx::cdroptarget::register(&mut method_registry);
        sun::lwawt::macosx::cdroptargetcontextpeer::register(&mut method_registry);
        sun::lwawt::macosx::cfretainedresource::register(&mut method_registry);
        sun::lwawt::macosx::cfiledialog::register(&mut method_registry);
        sun::lwawt::macosx::cimage::register(&mut method_registry);
        sun::lwawt::macosx::cinputmethod::register(&mut method_registry);
        sun::lwawt::macosx::cinputmethoddescriptor::register(&mut method_registry);
        sun::lwawt::macosx::cmenu::register(&mut method_registry);
        sun::lwawt::macosx::cmenubar::register(&mut method_registry);
        sun::lwawt::macosx::cmenuitem::register(&mut method_registry);
        sun::lwawt::macosx::cplatformcomponent::register(&mut method_registry);
        sun::lwawt::macosx::cplatformview::register(&mut method_registry);
        sun::lwawt::macosx::cplatformwindow::register(&mut method_registry);
        sun::lwawt::macosx::cpopupmenu::register(&mut method_registry);
        sun::lwawt::macosx::cprinterjob::register(&mut method_registry);
        sun::lwawt::macosx::cprinterjobdialog::register(&mut method_registry);
        sun::lwawt::macosx::cprinterpagedialog::register(&mut method_registry);
        sun::lwawt::macosx::cprintersurfacedata::register(&mut method_registry);
        sun::lwawt::macosx::crobot::register(&mut method_registry);
        sun::lwawt::macosx::ctextpipe::register(&mut method_registry);
        sun::lwawt::macosx::ctrayicon::register(&mut method_registry);
        sun::lwawt::macosx::cwrapper_nsview::register(&mut method_registry);
        sun::lwawt::macosx::cwrapper_nswindow::register(&mut method_registry);
        sun::lwawt::macosx::lwctoolkit::register(&mut method_registry);
        sun::lwawt::macosx::nsevent::register(&mut method_registry);
        sun::management::classloadingimpl::register(&mut method_registry);
        sun::management::garbagecollectorimpl::register(&mut method_registry);
        sun::management::memoryimpl::register(&mut method_registry);
        sun::management::memorymanagerimpl::register(&mut method_registry);
        sun::management::memorypoolimpl::register(&mut method_registry);
        sun::management::threadimpl::register(&mut method_registry);
        sun::management::vmmanagementimpl::register(&mut method_registry);
        sun::net::portconfig::register(&mut method_registry);
        sun::net::dns::resolverconfigurationimpl::register(&mut method_registry);
        sun::net::sdp::sdpsupport::register(&mut method_registry);
        sun::net::spi::defaultproxyselector::register(&mut method_registry);
        sun::nio::ch::datagramchannelimpl::register(&mut method_registry);
        sun::nio::ch::datagramdispatcher::register(&mut method_registry);
        sun::nio::ch::filedispatcherimpl::register(&mut method_registry);
        sun::nio::ch::filekey::register(&mut method_registry);
        sun::nio::ch::ioutil::register(&mut method_registry);
        sun::nio::ch::inheritedchannel::register(&mut method_registry);
        sun::nio::ch::kqueue::register(&mut method_registry);
        sun::nio::ch::nativethread::register(&mut method_registry);
        sun::nio::ch::net::register(&mut method_registry);
        sun::nio::ch::unixasynchronoussocketchannelimpl::register(&mut method_registry);
        sun::nio::fs::bsdnativedispatcher::register(&mut method_registry);
        sun::nio::fs::macosxnativedispatcher::register(&mut method_registry);
        sun::nio::fs::utifiletypedetector::register(&mut method_registry);
        sun::nio::fs::unixnativedispatcher::register(&mut method_registry);
        sun::print::cupsprinter::register(&mut method_registry);
        sun::security::jgss::wrapper::gsslibstub::register(&mut method_registry);
        sun::security::krb5::config::register(&mut method_registry);
        sun::security::krb5::credentials::register(&mut method_registry);
        sun::security::krb5::scdynamicstoreconfig::register(&mut method_registry);
        sun::security::smartcardio::pcsc::register(&mut method_registry);
        sun::security::smartcardio::platformpcsc::register(&mut method_registry);
        sun::util::locale::provider::hostlocaleprovideradapterimpl::register(&mut method_registry);

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

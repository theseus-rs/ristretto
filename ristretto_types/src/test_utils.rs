use crate::handles::{FileHandle, HandleManager, ThreadHandle};
#[cfg(not(target_family = "wasm"))]
use crate::handles::{SocketHandle, SocketType};
use crate::module_access::{AccessCheckResult, DefinedModule, ModuleAccess};
use crate::monitor::MonitorRegistry;
use crate::{Frame, NativeMemory, ResourceManager, Result, Thread, VM};
use ahash::{AHashMap, AHashSet};
use ristretto_classfile::{
    ClassAccessFlags, ClassFile, ConstantPool, FieldAccessFlags, FieldType, JAVA_17, VerifyMode,
    Version,
};
use ristretto_classloader::module::ResolvedConfiguration;
use ristretto_classloader::{Class, ClassLoader, ClassPath, Method, Object, Reference, Value};
use ristretto_gc::GarbageCollector;
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::{Mutex as TokioMutex, RwLock};

#[expect(clippy::unnecessary_wraps)]
pub(crate) fn class_file(
    name: &str,
    fields: &[(&str, &str)],
    code_source_url: Option<&str>,
) -> Result<ClassFile<'static>> {
    let mut constant_pool = ConstantPool::new();
    let this_class = constant_pool.add_class(name).expect("class name");
    let mut class_fields = Vec::with_capacity(fields.len());
    for (field_name, descriptor) in fields {
        let name_index = constant_pool.add_utf8(field_name).expect("field name");
        let descriptor_index = constant_pool
            .add_utf8(descriptor)
            .expect("field descriptor");
        let field_type = FieldType::parse(descriptor).expect("field descriptor type");
        class_fields.push(ristretto_classfile::Field {
            access_flags: FieldAccessFlags::PUBLIC,
            name_index,
            descriptor_index,
            field_type,
            attributes: Vec::new(),
        });
    }
    Ok(ClassFile {
        version: JAVA_17,
        constant_pool,
        access_flags: ClassAccessFlags::PUBLIC,
        this_class,
        fields: class_fields,
        code_source_url: code_source_url.map(ToString::to_string),
        ..Default::default()
    })
}

#[expect(clippy::unnecessary_wraps)]
pub(crate) fn class(name: &str, fields: &[(&str, &str)]) -> Result<Arc<Class>> {
    let class_file = class_file(name, fields, None).expect("class file");
    let class = Class::from(None, class_file).expect("class");
    Ok(class)
}

#[expect(clippy::unnecessary_wraps)]
pub(crate) fn class_with_code_source(
    name: &str,
    fields: &[(&str, &str)],
    code_source_url: &str,
) -> Result<Arc<Class>> {
    let class_file =
        class_file(name, fields, Some(code_source_url)).expect("class file with code source");
    let class = Class::from(None, class_file).expect("class with code source");
    Ok(class)
}

pub(crate) fn method(name: &str, descriptor: &str) -> Arc<Method> {
    let definition = ristretto_classfile::Method {
        access_flags: ristretto_classfile::MethodAccessFlags::PUBLIC,
        name_index: 0,
        descriptor_index: 0,
        attributes: Vec::new(),
    };
    Arc::new(Method::new_synthetic(
        definition,
        name.to_string(),
        descriptor.to_string(),
        Vec::new(),
        None,
    ))
}

#[derive(Debug)]
pub(crate) struct MockFrame {
    class: Arc<Class>,
    method: Arc<Method>,
    program_counter: usize,
}

impl MockFrame {
    pub(crate) fn new(class: Arc<Class>, method: Arc<Method>, program_counter: usize) -> Self {
        Self {
            class,
            method,
            program_counter,
        }
    }
}

impl Frame for MockFrame {
    fn class(&self) -> &Arc<Class> {
        &self.class
    }

    fn method(&self) -> &Arc<Method> {
        &self.method
    }

    fn program_counter(&self) -> usize {
        self.program_counter
    }
}

#[derive(Debug)]
pub(crate) struct MockModuleSystem {
    modules: Mutex<AHashMap<String, DefinedModule>>,
    package_modules: Mutex<AHashMap<String, Value>>,
    boot_unnamed_module: Mutex<Option<Value>>,
    resolved_configuration: ResolvedConfiguration,
}

impl MockModuleSystem {
    pub(crate) fn new() -> Self {
        Self::with_resolved_configuration(ResolvedConfiguration::empty())
    }

    pub(crate) fn with_resolved_configuration(
        resolved_configuration: ResolvedConfiguration,
    ) -> Self {
        Self {
            modules: Mutex::new(AHashMap::default()),
            package_modules: Mutex::new(AHashMap::default()),
            boot_unnamed_module: Mutex::new(None),
            resolved_configuration,
        }
    }

    pub(crate) fn set_module_for_package(&self, package: &str, module: Value) {
        self.package_modules
            .lock()
            .expect("package modules lock")
            .insert(package.to_string(), module);
    }
}

impl Default for MockModuleSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleAccess for MockModuleSystem {
    fn add_export(&self, _source_module: &str, _package: &str, _target_module: Option<&str>) {}

    fn add_export_to_all(&self, _source_module: &str, _package: &str) {}

    fn add_export_to_all_unnamed(&self, _source_module: &str, _package: &str) {}

    fn add_opens(&self, _source_module: &str, _package: &str, _target_module: Option<&str>) {}

    fn add_opens_to_all(&self, _source_module: &str, _package: &str) {}

    fn add_opens_to_all_unnamed(&self, _source_module: &str, _package: &str) {}

    fn add_read(&self, _source_module: &str, _target_module: &str) {}

    fn define_module(&self, module: DefinedModule) {
        self.modules
            .lock()
            .expect("modules lock")
            .insert(module.name.clone(), module);
    }

    fn get_module(&self, name: &str) -> Option<DefinedModule> {
        self.modules
            .lock()
            .expect("modules lock")
            .get(name)
            .cloned()
    }

    fn is_module_open(&self, name: &str) -> bool {
        self.get_module(name).is_some_and(|module| module.is_open)
    }

    fn check_access(
        &self,
        _from_module: Option<&str>,
        _to_module: Option<&str>,
        _to_class_name: &str,
    ) -> AccessCheckResult {
        AccessCheckResult::Allowed
    }

    fn check_reflection_access(
        &self,
        _from_module: Option<&str>,
        _to_module: Option<&str>,
        _to_class_name: &str,
    ) -> AccessCheckResult {
        AccessCheckResult::Allowed
    }

    fn require_reflection_access(
        &self,
        from_module: Option<&str>,
        to_module: Option<&str>,
        to_class_name: &str,
    ) -> Result<()> {
        let _result = self.check_reflection_access(from_module, to_module, to_class_name);
        Ok(())
    }

    fn set_boot_unnamed_module(&self, module: Value) {
        *self
            .boot_unnamed_module
            .lock()
            .expect("boot unnamed module lock") = Some(module);
    }

    fn boot_unnamed_module(&self) -> Option<Value> {
        self.boot_unnamed_module
            .lock()
            .expect("boot unnamed module lock")
            .clone()
    }

    fn get_module_for_package(&self, package: &str) -> Option<Value> {
        self.package_modules
            .lock()
            .expect("package modules lock")
            .get(package)
            .cloned()
    }

    fn resolved_configuration(&self) -> &ResolvedConfiguration {
        &self.resolved_configuration
    }

    fn all_defined_packages(&self) -> Vec<String> {
        self.modules
            .lock()
            .expect("modules lock")
            .values()
            .flat_map(|module| module.packages.iter().cloned())
            .collect()
    }
}

pub(crate) struct MockVm {
    garbage_collector: Arc<GarbageCollector>,
    java_home: PathBuf,
    java_version: String,
    java_major_version: u16,
    java_class_file_version: Version,
    system_properties: AHashMap<String, String>,
    next_thread_id: AtomicU64,
    next_hidden_class_suffix: AtomicU64,
    next_nio_fd: AtomicUsize,
    module_system: MockModuleSystem,
    class_path: ClassPath,
    stdin: Arc<TokioMutex<dyn Read + Send + Sync>>,
    stdout: Arc<TokioMutex<dyn Write + Send + Sync>>,
    stderr: Arc<TokioMutex<dyn Write + Send + Sync>>,
    native_memory: NativeMemory,
    resource_manager: ResourceManager,
    file_handles: HandleManager<i64, FileHandle>,
    #[cfg(not(target_family = "wasm"))]
    socket_handles: HandleManager<i32, SocketHandle>,
    thread_handles: HandleManager<u64, ThreadHandle<MockThread>>,
    monitor_registry: MonitorRegistry,
    class_loader: Arc<RwLock<Arc<ClassLoader>>>,
    classes: Mutex<AHashMap<String, Arc<Class>>>,
}

impl MockVm {
    pub(crate) fn new(version: Version) -> Arc<Self> {
        Self::with_resolved_configuration(version, ResolvedConfiguration::empty())
    }

    pub(crate) fn with_resolved_configuration(
        version: Version,
        resolved_configuration: ResolvedConfiguration,
    ) -> Arc<Self> {
        Arc::new(Self {
            garbage_collector: GarbageCollector::new(),
            java_home: PathBuf::from("/mock/java/home"),
            java_version: "mock-java".to_string(),
            java_major_version: version.major(),
            java_class_file_version: version,
            system_properties: AHashMap::default(),
            next_thread_id: AtomicU64::new(1),
            next_hidden_class_suffix: AtomicU64::new(1),
            next_nio_fd: AtomicUsize::new(3),
            module_system: MockModuleSystem::with_resolved_configuration(resolved_configuration),
            class_path: ClassPath::new(Vec::new()),
            stdin: Arc::new(TokioMutex::new(Cursor::new(Vec::new()))),
            stdout: Arc::new(TokioMutex::new(Cursor::new(Vec::new()))),
            stderr: Arc::new(TokioMutex::new(Cursor::new(Vec::new()))),
            native_memory: NativeMemory::new(),
            resource_manager: ResourceManager::new(),
            file_handles: HandleManager::new(),
            #[cfg(not(target_family = "wasm"))]
            socket_handles: HandleManager::new(),
            thread_handles: HandleManager::new(),
            monitor_registry: MonitorRegistry::new(),
            class_loader: Arc::new(RwLock::new(ClassLoader::new(
                "bootstrap",
                ClassPath::new(Vec::new()),
            ))),
            classes: Mutex::new(AHashMap::default()),
        })
    }

    pub(crate) fn module_system(&self) -> &MockModuleSystem {
        &self.module_system
    }

    pub(crate) fn garbage_collector(&self) -> &Arc<GarbageCollector> {
        &self.garbage_collector
    }

    fn normalized_class_name(name: &str) -> String {
        name.replace('.', "/")
    }

    fn class_fields(name: &str) -> &'static [(&'static str, &'static str)] {
        match name {
            "java/lang/Class" => &[
                ("name", "Ljava/lang/String;"),
                ("module", "Ljava/lang/Module;"),
                ("componentType", "Ljava/lang/Class;"),
            ],
            "java/lang/String" => &[
                ("value", "[B"),
                ("hash", "I"),
                ("coder", "I"),
                ("hashIsZero", "I"),
            ],
            "java/lang/Module" => &[
                ("name", "Ljava/lang/String;"),
                ("loader", "Ljava/lang/ClassLoader;"),
                ("descriptor", "Ljava/lang/module/ModuleDescriptor;"),
            ],
            "java/lang/module/ModuleDescriptor" => &[
                ("name", "Ljava/lang/String;"),
                ("modifiers", "Ljava/util/Set;"),
                ("open", "Z"),
                ("automatic", "Z"),
                ("requires", "Ljava/util/Set;"),
                ("exports", "Ljava/util/Set;"),
                ("opens", "Ljava/util/Set;"),
                ("uses", "Ljava/util/Set;"),
                ("provides", "Ljava/util/Map;"),
                ("packages", "Ljava/util/Set;"),
            ],
            "java/lang/ClassLoader" | "jdk/internal/loader/BuiltinClassLoader" => {
                &[("unnamedModule", "Ljava/lang/Module;")]
            }
            _ => &[],
        }
    }

    pub(crate) fn get_or_create_class(&self, name: &str) -> Result<Arc<Class>> {
        let normalized_name = Self::normalized_class_name(name);
        let mut classes = self.classes.lock().expect("classes lock");
        if let Some(class) = classes.get(&normalized_name) {
            return Ok(class.clone());
        }
        let class = class(&normalized_name, Self::class_fields(&normalized_name))?;
        classes.insert(normalized_name, class.clone());
        Ok(class)
    }

    pub(crate) fn object_value(&self, class_name: &str) -> Result<Value> {
        let class = self.get_or_create_class(class_name)?;
        let object = Object::new(class)?;
        Ok(Value::from_object(&self.garbage_collector, object))
    }
}

impl VM for MockVm {
    type ThreadType = MockThread;
    type ModuleSystem = MockModuleSystem;

    fn garbage_collector(&self) -> &Arc<GarbageCollector> {
        &self.garbage_collector
    }

    fn java_home(&self) -> &PathBuf {
        &self.java_home
    }

    fn java_version(&self) -> &str {
        &self.java_version
    }

    fn java_major_version(&self) -> u16 {
        self.java_major_version
    }

    fn java_class_file_version(&self) -> &Version {
        &self.java_class_file_version
    }

    fn system_properties(&self) -> &AHashMap<String, String> {
        &self.system_properties
    }

    fn next_thread_id(&self) -> Result<u64> {
        Ok(self.next_thread_id.fetch_add(1, Ordering::SeqCst))
    }

    fn next_hidden_class_suffix(&self) -> Result<u64> {
        Ok(self.next_hidden_class_suffix.fetch_add(1, Ordering::SeqCst))
    }

    fn next_nio_fd(&self) -> i32 {
        i32::try_from(self.next_nio_fd.fetch_add(1, Ordering::SeqCst)).unwrap_or(i32::MAX)
    }

    fn class<'a>(&'a self, class_name: &'a str) -> crate::BoxFuture<'a, Result<Arc<Class>>> {
        Box::pin(async move { self.get_or_create_class(class_name) })
    }

    fn invoke_main<'a>(
        &'a self,
        _parameters: &'a [&'a str],
    ) -> crate::BoxFuture<'a, Result<Option<Value>>> {
        Box::pin(async move { Ok(Some(Value::Int(0))) })
    }

    fn module_system(&self) -> &Self::ModuleSystem {
        &self.module_system
    }

    fn class_path(&self) -> &ClassPath {
        &self.class_path
    }

    fn verify_mode(&self) -> VerifyMode {
        VerifyMode::Remote
    }

    fn preview_features(&self) -> bool {
        false
    }

    fn stdin(&self) -> Arc<TokioMutex<dyn Read + Send + Sync>> {
        self.stdin.clone()
    }

    fn stdout(&self) -> Arc<TokioMutex<dyn Write + Send + Sync>> {
        self.stdout.clone()
    }

    fn stderr(&self) -> Arc<TokioMutex<dyn Write + Send + Sync>> {
        self.stderr.clone()
    }

    fn native_memory(&self) -> &NativeMemory {
        &self.native_memory
    }

    fn resource_manager(&self) -> &ResourceManager {
        &self.resource_manager
    }

    fn file_handles(&self) -> &HandleManager<i64, FileHandle> {
        &self.file_handles
    }

    #[cfg(not(target_family = "wasm"))]
    fn socket_handles(&self) -> &HandleManager<i32, SocketHandle> {
        &self.socket_handles
    }

    fn thread_handles(&self) -> &HandleManager<u64, ThreadHandle<Self::ThreadType>> {
        &self.thread_handles
    }

    fn monitor_registry(&self) -> &MonitorRegistry {
        &self.monitor_registry
    }

    fn class_loader(&self) -> Arc<RwLock<Arc<ClassLoader>>> {
        self.class_loader.clone()
    }

    fn intern_string<'a>(
        &'a self,
        thread: &'a Self::ThreadType,
        string: &'a str,
    ) -> crate::BoxFuture<'a, Result<Value>> {
        Box::pin(async move { thread.string_object(string).await })
    }

    fn object<'a>(
        &'a self,
        class_name: &'a str,
        _descriptor: &'a str,
        _parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Value>> {
        Box::pin(async move { self.object_value(class_name) })
    }

    fn create_thread(&self, id: u64) -> Result<Arc<Self::ThreadType>> {
        Ok(MockThread::new_with_id(
            Arc::new(self.clone_for_thread()),
            id,
        ))
    }
}

impl MockVm {
    fn clone_for_thread(&self) -> Self {
        Self {
            garbage_collector: self.garbage_collector.clone(),
            java_home: self.java_home.clone(),
            java_version: self.java_version.clone(),
            java_major_version: self.java_major_version,
            java_class_file_version: self.java_class_file_version.clone(),
            system_properties: self.system_properties.clone(),
            next_thread_id: AtomicU64::new(self.next_thread_id.load(Ordering::SeqCst)),
            next_hidden_class_suffix: AtomicU64::new(
                self.next_hidden_class_suffix.load(Ordering::SeqCst),
            ),
            next_nio_fd: AtomicUsize::new(self.next_nio_fd.load(Ordering::SeqCst)),
            module_system: MockModuleSystem::new(),
            class_path: self.class_path.clone(),
            stdin: self.stdin.clone(),
            stdout: self.stdout.clone(),
            stderr: self.stderr.clone(),
            native_memory: NativeMemory::new(),
            resource_manager: ResourceManager::new(),
            file_handles: HandleManager::new(),
            #[cfg(not(target_family = "wasm"))]
            socket_handles: HandleManager::new(),
            thread_handles: HandleManager::new(),
            monitor_registry: MonitorRegistry::new(),
            class_loader: self.class_loader.clone(),
            classes: Mutex::new(self.classes.lock().expect("classes lock").clone()),
        }
    }
}

pub(crate) struct MockThread {
    id: u64,
    vm: Arc<MockVm>,
    name: Mutex<String>,
    java_object: Mutex<Value>,
    frames: Mutex<Vec<Arc<MockFrame>>>,
    interrupted: AtomicBool,
    fail_next_try_invoke: AtomicBool,
}

impl MockThread {
    pub(crate) fn new(vm: Arc<MockVm>) -> Arc<Self> {
        Self::new_with_id(vm, 7)
    }

    pub(crate) fn new_with_id(vm: Arc<MockVm>, id: u64) -> Arc<Self> {
        Arc::new(Self {
            id,
            vm,
            name: Mutex::new("mock-thread".to_string()),
            java_object: Mutex::new(Value::Object(None)),
            frames: Mutex::new(Vec::new()),
            interrupted: AtomicBool::new(false),
            fail_next_try_invoke: AtomicBool::new(false),
        })
    }

    pub(crate) fn push_frame(&self, frame: Arc<MockFrame>) {
        self.frames.lock().expect("frames lock").push(frame);
    }

    pub(crate) fn fail_next_try_invoke(&self) {
        self.fail_next_try_invoke.store(true, Ordering::SeqCst);
    }

    pub(crate) async fn string_object(&self, value: &str) -> Result<Value> {
        let class = self.class("java/lang/String").await?;
        let mut object = Object::new(class)?;
        let bytes: Vec<i8> = value.bytes().map(u8::cast_signed).collect();
        object.set_value(
            "value",
            Value::new_object(self.vm.garbage_collector(), Reference::from(bytes)),
        )?;
        object.set_value("hash", Value::Int(0))?;
        object.set_value("coder", Value::Int(0))?;
        object.set_value("hashIsZero", Value::Int(0))?;
        Ok(Value::from_object(self.vm.garbage_collector(), object))
    }
}

impl Thread for MockThread {
    type Vm = MockVm;
    type Frame = MockFrame;

    fn id(&self) -> u64 {
        self.id
    }

    fn vm(&self) -> Result<Arc<Self::Vm>> {
        Ok(self.vm.clone())
    }

    fn name(&self) -> crate::BoxFuture<'_, String> {
        Box::pin(async move { self.name.lock().expect("name lock").clone() })
    }

    fn set_name<'a>(&'a self, name: &'a str) -> crate::BoxFuture<'a, ()> {
        Box::pin(async move {
            *self.name.lock().expect("name lock") = name.to_string();
        })
    }

    fn java_object(&self) -> crate::BoxFuture<'_, Value> {
        Box::pin(async move { self.java_object.lock().expect("java object lock").clone() })
    }

    fn set_java_object(&self, value: Value) -> crate::BoxFuture<'_, ()> {
        Box::pin(async move {
            *self.java_object.lock().expect("java object lock") = value;
        })
    }

    fn frames(&self) -> crate::BoxFuture<'_, Result<Vec<Arc<Self::Frame>>>> {
        Box::pin(async move { Ok(self.frames.lock().expect("frames lock").clone()) })
    }

    fn interrupt(&self) {
        self.interrupted.store(true, Ordering::SeqCst);
    }

    fn is_interrupted(&self, clear_interrupt: bool) -> bool {
        if clear_interrupt {
            self.interrupted.swap(false, Ordering::SeqCst)
        } else {
            self.interrupted.load(Ordering::SeqCst)
        }
    }

    fn sleep(&self, _duration: Duration) -> crate::BoxFuture<'_, bool> {
        Box::pin(async move { self.is_interrupted(true) })
    }

    fn park(&self, _is_absolute: bool, _time: u64) -> crate::BoxFuture<'_, Result<()>> {
        Box::pin(async move { Ok(()) })
    }

    fn unpark(&self) {}

    fn class<'a>(&'a self, class_name: &'a str) -> crate::BoxFuture<'a, Result<Arc<Class>>> {
        Box::pin(async move { self.vm.get_or_create_class(class_name) })
    }

    fn class_java_str<'a>(
        &'a self,
        class_name: &'a ristretto_classfile::JavaStr,
    ) -> crate::BoxFuture<'a, Result<Arc<Class>>> {
        Box::pin(async move { self.class(&class_name.to_str_lossy()).await })
    }

    fn load_and_link_class<'a>(
        &'a self,
        class_name: &'a ristretto_classfile::JavaStr,
    ) -> crate::BoxFuture<'a, Result<Arc<Class>>> {
        Box::pin(async move { self.class_java_str(class_name).await })
    }

    fn register_class(&self, class: Arc<Class>) -> crate::BoxFuture<'_, Result<()>> {
        Box::pin(async move {
            self.vm
                .classes
                .lock()
                .expect("classes lock")
                .insert(class.name().to_string(), class);
            Ok(())
        })
    }

    fn invoke<'a>(
        &'a self,
        _class: &'a str,
        _method: &'a str,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Option<Value>>> {
        Box::pin(async move { Ok(parameters.first().cloned()) })
    }

    fn try_invoke<'a>(
        &'a self,
        _class: &'a str,
        method: &'a str,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Value>> {
        Box::pin(async move {
            if self.fail_next_try_invoke.swap(false, Ordering::SeqCst) {
                return Err(crate::Error::InternalError(
                    "mock try_invoke failure".to_string(),
                ));
            }
            if method == "getUnnamedModule()Ljava/lang/Module;"
                && let Some(class_loader_object) = parameters.first()
                && let Ok(object) = class_loader_object.as_object_ref()
            {
                return object
                    .value("unnamedModule")
                    .or_else(|_| Ok(Value::Object(None)));
            }
            Ok(parameters.first().cloned().unwrap_or(Value::Object(None)))
        })
    }

    fn execute<'a>(
        &'a self,
        _class: &'a Arc<Class>,
        _method: &'a Arc<Method>,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Option<Value>>> {
        Box::pin(async move { Ok(parameters.first().cloned()) })
    }

    fn try_execute<'a>(
        &'a self,
        class: &'a Arc<Class>,
        method: &'a Arc<Method>,
        parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Value>> {
        Box::pin(async move {
            self.execute(class, method, parameters)
                .await?
                .ok_or_else(|| crate::Error::InternalError("missing mock return value".to_string()))
        })
    }

    fn object<'a>(
        &'a self,
        class_name: &'a str,
        _descriptor: &'a str,
        _parameters: &'a [Value],
    ) -> crate::BoxFuture<'a, Result<Value>> {
        Box::pin(async move { self.vm.object_value(class_name) })
    }

    fn intern_string<'a>(&'a self, string: &'a str) -> crate::BoxFuture<'a, Result<Value>> {
        Box::pin(async move { self.string_object(string).await })
    }
}

#[cfg(not(target_family = "wasm"))]
pub(crate) fn raw_socket_type() -> SocketType {
    let socket = socket2::Socket::new(
        socket2::Domain::IPV4,
        socket2::Type::STREAM,
        Some(socket2::Protocol::TCP),
    )
    .expect("socket");
    SocketType::Raw(socket)
}

#[tokio::test]
#[expect(clippy::too_many_lines)]
async fn test_mock_vm_thread_and_module_system_methods() -> Result<()> {
    let vm = MockVm::new(JAVA_17);
    let thread = MockThread::new(vm.clone());
    let _default_module_system = MockModuleSystem::default();
    assert!(Arc::strong_count(MockVm::garbage_collector(vm.as_ref())) >= 1);
    assert_eq!(thread.id(), 7);
    assert_eq!(thread.vm()?.java_version(), "mock-java");
    thread.set_name("renamed").await;
    assert_eq!(thread.name().await, "renamed");
    thread.set_java_object(Value::Int(1)).await;
    assert_eq!(thread.java_object().await, Value::Int(1));
    let frame_class = class("FrameOwner", &[])?;
    let frame_method = method("run", "()V");
    thread.push_frame(Arc::new(MockFrame::new(
        frame_class.clone(),
        frame_method.clone(),
        12,
    )));
    assert_eq!(thread.frames().await?.len(), 1);
    thread.interrupt();
    assert!(thread.is_interrupted(false));
    assert!(thread.sleep(Duration::from_millis(0)).await);
    assert!(!thread.is_interrupted(false));
    thread.park(false, 0).await?;
    thread.unpark();
    assert_eq!(
        thread.class("java/lang/Object").await?.name(),
        "java/lang/Object"
    );
    let java_str = ristretto_classfile::JavaStr::try_from_str("java/lang/String")?;
    assert_eq!(
        thread.class_java_str(java_str).await?.name(),
        "java/lang/String"
    );
    assert_eq!(
        thread.load_and_link_class(java_str).await?.name(),
        "java/lang/String"
    );
    thread.register_class(frame_class.clone()).await?;
    assert_eq!(
        thread
            .invoke("Owner", "method", &[Value::Int(2)])
            .await?
            .expect("invoke value"),
        Value::Int(2)
    );
    assert_eq!(
        thread
            .try_invoke("Owner", "valueOf(I)Ljava/lang/Integer;", &[Value::Int(3)])
            .await?,
        Value::Int(3)
    );
    assert_eq!(
        thread
            .execute(&frame_class, &frame_method, &[Value::Int(4)])
            .await?
            .expect("execute value"),
        Value::Int(4)
    );
    assert_eq!(
        thread
            .try_execute(&frame_class, &frame_method, &[Value::Int(5)])
            .await?,
        Value::Int(5)
    );
    assert!(
        thread
            .object("java/lang/Object", "()V", &[])
            .await?
            .is_object()
    );
    assert!(thread.intern_string("value").await?.is_object());
    thread.fail_next_try_invoke();
    assert!(thread.try_invoke("Owner", "method", &[]).await.is_err());
    let object_without_module = vm.object_value("java/lang/Object")?;
    assert_eq!(
        thread
            .try_invoke(
                "java.lang.ClassLoader",
                "getUnnamedModule()Ljava/lang/Module;",
                &[object_without_module],
            )
            .await?,
        Value::Object(None)
    );
    assert!(
        thread
            .try_execute(&frame_class, &frame_method, &[])
            .await
            .is_err()
    );

    assert!(Arc::strong_count(vm.garbage_collector()) >= 1);
    assert_eq!(vm.java_home(), &PathBuf::from("/mock/java/home"));
    assert_eq!(vm.java_major_version(), JAVA_17.major());
    assert_eq!(vm.java_class_file_version(), &JAVA_17);
    assert!(vm.system_properties().is_empty());
    assert_eq!(vm.next_thread_id()?, 1);
    assert_eq!(vm.next_hidden_class_suffix()?, 1);
    assert_eq!(vm.next_nio_fd(), 3);
    assert!(vm.class("java/lang/Object").await?.name() == "java/lang/Object");
    assert_eq!(
        vm.invoke_main(&["arg"]).await?.expect("main result"),
        Value::Int(0)
    );
    assert!(vm.class_path().iter().next().is_none());
    assert_eq!(vm.verify_mode(), VerifyMode::Remote);
    assert!(!vm.preview_features());
    assert!(vm.stdin().try_lock().is_ok());
    assert!(vm.stdout().try_lock().is_ok());
    assert!(vm.stderr().try_lock().is_ok());
    assert!(!vm.native_memory().contains(1));
    assert!(format!("{:?}", vm.resource_manager()).contains("ResourceManager"));
    assert!(vm.file_handles().read().await.is_empty());
    #[cfg(not(target_family = "wasm"))]
    assert!(vm.socket_handles().read().await.is_empty());
    assert!(vm.thread_handles().read().await.is_empty());
    assert!(Arc::ptr_eq(
        &vm.monitor_registry().monitor(1),
        &vm.monitor_registry().monitor(1)
    ));
    assert_eq!(vm.class_loader().read().await.name(), "bootstrap");
    assert!(vm.intern_string(&thread, "interned").await?.is_object());
    assert!(vm.object("java/lang/Object", "()V", &[]).await?.is_object());
    assert_eq!(vm.create_thread(99)?.id(), 99);

    let bad_string_vm = MockVm::new(JAVA_17);
    let bad_string_thread = MockThread::new(bad_string_vm);
    bad_string_thread
        .register_class(class("java/lang/String", &[])?)
        .await?;
    assert!(bad_string_thread.string_object("bad").await.is_err());

    let module_system = vm.module_system();
    module_system.add_export("source", "pkg", Some("target"));
    module_system.add_export_to_all("source", "pkg");
    module_system.add_export_to_all_unnamed("source", "pkg");
    module_system.add_opens("source", "pkg", Some("target"));
    module_system.add_opens_to_all("source", "pkg");
    module_system.add_opens_to_all_unnamed("source", "pkg");
    module_system.add_read("source", "target");
    let mut module = DefinedModule::new("module.name".to_string(), true);
    module.packages = AHashSet::from_iter(["pkg".to_string()]);
    module_system.define_module(module);
    assert!(module_system.get_module("module.name").is_some());
    assert!(module_system.is_module_open("module.name"));
    assert_eq!(
        module_system.check_access(Some("a"), Some("b"), "b/C"),
        AccessCheckResult::Allowed
    );
    assert_eq!(
        module_system.check_reflection_access(Some("a"), Some("b"), "b/C"),
        AccessCheckResult::Allowed
    );
    module_system.require_reflection_access(Some("a"), Some("b"), "b/C")?;
    let module_value = vm.object_value("java/lang/Module")?;
    module_system.set_boot_unnamed_module(module_value.clone());
    assert!(module_system.boot_unnamed_module().is_some());
    module_system.set_module_for_package("pkg", module_value);
    assert!(module_system.get_module_for_package("pkg").is_some());
    assert!(module_system.resolved_configuration().is_empty());
    assert_eq!(
        module_system.all_defined_packages(),
        vec!["pkg".to_string()]
    );

    Ok(())
}

use ristretto_gc::canonical::Configuration;
use ristretto_vm_research::canonical::Limits;
use ristretto_vm_research::canonical_telemetry::{Binding, current_process_id};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    objects: usize,
    roots: usize,
    mutators: usize,
    helpers: usize,
    edges_per_object: usize,
}
impl Config {
    pub fn value(&self) -> Configuration {
        Configuration {
            objects: self.objects,
            roots: self.roots,
            mutators: self.mutators,
            helpers: self.helpers,
            edges_per_object: self.edges_per_object,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkLimits {
    pub instructions: usize,
    pub request_polls: usize,
    pub cleanup_polls: usize,
    pub frames: usize,
    pub native_slices: usize,
    pub array_bytes: usize,
}
impl WorkLimits {
    pub fn interpreted(&self) -> Limits {
        Limits {
            instructions: self.instructions,
            request_polls: self.request_polls,
            cleanup_polls: self.cleanup_polls,
            frames: self.frames,
            array_bytes: self.array_bytes,
        }
    }
    #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
    pub fn compiled(&self) -> ristretto_vm_research::canonical::CompiledLimits {
        ristretto_vm_research::canonical::CompiledLimits {
            request_polls: self.request_polls,
            cleanup_polls: self.cleanup_polls,
            frames: self.frames,
            native_slices: self.native_slices,
            array_bytes: self.array_bytes,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Admission {
    pub schema_version: u32,
    pub tier: String,
    pub class: String,
    pub method: String,
    pub argument: i32,
    pub configuration: Config,
    pub limits: WorkLimits,
    pub recorder_capacity: usize,
    pub expected_result: i32,
    pub require_compiled_backedges: bool,
    pub window_kind: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Build {
    schema_version: u32,
    source_archive_sha256: String,
    source_manifest_sha256: String,
    binary_sha256: String,
    binary_name: String,
    target: String,
    features: Vec<String>,
    command: Vec<String>,
    toolchain: String,
    source_unchanged: bool,
    qualification: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Launch {
    schema_version: u32,
    run_id: String,
    mode: String,
    host_process_id: u32,
    command: Vec<String>,
    host_executable_sha256: String,
    binary_sha256: String,
    launcher_sha256: String,
}

#[derive(Debug)]
pub struct Inputs {
    pub admission: Admission,
    pub class_bytes: Vec<Vec<u8>>,
    pub binding: Binding,
    pub identity: Value,
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut result = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        result.push(char::from(DIGITS[usize::from(byte >> 4)]));
        result.push(char::from(DIGITS[usize::from(byte & 15)]));
    }
    result
}

pub fn digest(path: &Path) -> std::io::Result<String> {
    let mut stream = std::fs::File::open(path)?;
    let mut buffer = [0_u8; 16 * 1024];
    let mut hash = Sha256::new();
    loop {
        let count = stream.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    Ok(hex(&hash.finalize()))
}

fn classfiles(root: &Path) -> Result<(Vec<Vec<u8>>, String)> {
    fn visit(root: &Path, names: &mut Vec<PathBuf>) -> std::io::Result<()> {
        for entry in std::fs::read_dir(root)? {
            let entry = entry?;
            let kind = entry.file_type()?;
            if kind.is_symlink() {
                return Err(std::io::Error::other("symlink workload input"));
            }
            if kind.is_dir() {
                visit(&entry.path(), names)?;
            } else if entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "class")
            {
                names.push(entry.path());
            }
        }
        Ok(())
    }
    let mut names = Vec::new();
    visit(root, &mut names)?;
    names.sort();
    if names.is_empty() {
        return Err("class manifest empty".into());
    }
    let mut hash = Sha256::new();
    let mut classes = Vec::new();
    for name in names {
        let relative = name
            .strip_prefix(root)?
            .to_str()
            .ok_or("non-UTF8 class name")?
            .replace('\\', "/");
        let bytes = std::fs::read(&name)?;
        hash.update(relative.as_bytes());
        hash.update([0]);
        hash.update(hex(&Sha256::digest(&bytes)).as_bytes());
        hash.update(b"\n");
        // The exact bytes hashed above are the bytes admitted, not a second read.
        classes.push(bytes);
    }
    Ok((classes, hex(&hash.finalize())))
}

impl Inputs {
    pub fn read(root: &Path) -> Result<Self> {
        let build: Build = serde_json::from_slice(&std::fs::read(root.join("build.json"))?)?;
        let launch: Launch = serde_json::from_slice(&std::fs::read(root.join("launch.json"))?)?;
        let admission_bytes = std::fs::read(root.join("admission.json"))?;
        let admission: Admission = serde_json::from_slice(&admission_bytes)?;
        if build.schema_version != 1
            || launch.schema_version != 1
            || admission.schema_version != 1
            || !build.source_unchanged
            || build.qualification
            || admission.window_kind != "entry_body_v1"
            || !matches!(admission.tier.as_str(), "interpreted" | "compiled")
            || admission.configuration.mutators != 1
            || admission.configuration.helpers != 1
            || admission.limits.instructions == 0
            || admission.limits.request_polls == 0
            || admission.limits.cleanup_polls == 0
            || admission.limits.frames == 0
            || admission.limits.native_slices == 0
            || admission.limits.array_bytes == 0
            || build.target.is_empty()
            || build.features.is_empty()
            || build.command.is_empty()
            || build.toolchain.is_empty()
            || launch.command.is_empty()
            || launch.launcher_sha256.len() != 64
        {
            return Err("unsupported build/launch/admission contract".into());
        }
        let guest_pid = current_process_id();
        let binary;
        #[cfg(not(target_family = "wasm"))]
        {
            if launch.mode != "native"
                || guest_pid != Some(launch.host_process_id)
                || build.binary_name != "program"
                || launch.host_executable_sha256 != build.binary_sha256
            {
                return Err("native executable/PID contract".into());
            }
            binary = std::env::current_exe()?;
        }
        #[cfg(target_family = "wasm")]
        {
            if launch.mode != "wasi"
                || guest_pid.is_some()
                || build.target != "wasm32-wasip2"
                || build.binary_name != "program.wasm"
                || admission.tier != "interpreted"
            {
                return Err("WASI module/PID contract".into());
            }
            binary = root.join("program.wasm");
        }
        let executable = digest(&binary)?;
        let archive = digest(&root.join("sources.tar.gz"))?;
        let manifest = digest(&root.join("source-manifest.json"))?;
        if executable != build.binary_sha256
            || executable != launch.binary_sha256
            || archive != build.source_archive_sha256
            || manifest != build.source_manifest_sha256
        {
            return Err("actual executable/source artifacts differ from build receipt".into());
        }
        let (classes, workload) = classfiles(&root.join("classes"))?;
        let configuration = hex(&Sha256::digest(&admission_bytes));
        let identity = json!({"run_id":launch.run_id,"mode":launch.mode,"tier":admission.tier,
            "host_process_id":launch.host_process_id,"guest_process_id":guest_pid,
            "host_executable_sha256":launch.host_executable_sha256,"binary_sha256":executable,
            "source_archive_sha256":archive,"source_manifest_sha256":manifest,
            "workload_sha256":workload,"admission_sha256":configuration,
            "build_receipt_sha256":digest(&root.join("build.json"))?,
            "launch_receipt_sha256":digest(&root.join("launch.json"))?});
        let binding = Binding {
            process_id: guest_pid,
            run_id: launch.run_id,
            source_sha256: archive,
            binary_sha256: executable,
            workload_sha256: workload,
            configuration_sha256: configuration,
        };
        Ok(Self {
            admission,
            class_bytes: classes,
            binding,
            identity,
        })
    }
}

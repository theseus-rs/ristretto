use crate::Result;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::Limits;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkLimits {
    instructions: usize,
    request_polls: usize,
    cleanup_polls: usize,
    frames: usize,
    native_slices: usize,
    array_bytes: usize,
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
    pub fn compiled(&self) -> ristretto_vm::canonical::CompiledLimits {
        ristretto_vm::canonical::CompiledLimits {
            request_polls: self.request_polls,
            cleanup_polls: self.cleanup_polls,
            frames: self.frames,
            native_slices: self.native_slices,
            array_bytes: self.array_bytes,
        }
    }
}

/// The service admission execution subset, with its two recorder-only fields
/// accepted for compatibility. This driver never creates a service recorder.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Admission {
    pub schema_version: u32,
    pub tier: String,
    pub class: String,
    pub method: String,
    pub argument: i32,
    pub configuration: Config,
    pub limits: WorkLimits,
    pub expected_result: i32,
    pub require_compiled_backedges: bool,
    #[serde(default)]
    recorder_capacity: Option<usize>,
    #[serde(default)]
    window_kind: Option<String>,
}

impl Admission {
    fn validate(&self) -> Result<()> {
        if self.schema_version != 1
            || !matches!(self.tier.as_str(), "interpreted" | "compiled")
            || self.class.is_empty()
            || self.method.is_empty()
            || self.configuration.mutators != 1
            || self.configuration.helpers != 1
            || self.limits.instructions == 0
            || self.limits.request_polls == 0
            || self.limits.cleanup_polls == 0
            || self.limits.frames == 0
            || self.limits.native_slices == 0
            || self.limits.array_bytes == 0
            || self
                .window_kind
                .as_deref()
                .is_some_and(|kind| !matches!(kind, "entry_body_v1" | "full_invocation_v1"))
        {
            return Err("invalid allocation admission contract".into());
        }
        self.configuration.value().charged_bytes()?;
        #[cfg(not(all(feature = "compiled", not(target_family = "wasm"))))]
        if self.tier == "compiled" {
            return Err("artifact does not support native canonical compiled execution".into());
        }
        Ok(())
    }
}

#[derive(Serialize)]
pub struct ClassIdentity {
    path: String,
    sha256: String,
}

pub struct Inputs {
    pub admission: Admission,
    pub admission_sha256: String,
    pub class_bytes: Vec<Vec<u8>>,
    pub classes: Vec<ClassIdentity>,
    pub class_manifest_sha256: String,
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut result = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        // Writing into a String cannot fail.
        let _ = write!(&mut result, "{byte:02x}");
    }
    result
}

fn visit(root: &Path, names: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(root)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        if kind.is_symlink() {
            return Err("symlink class input".into());
        }
        if kind.is_dir() {
            visit(&entry.path(), names)?;
        } else if entry
            .path()
            .extension()
            .is_some_and(|value| value == "class")
        {
            names.push(entry.path());
        }
    }
    Ok(())
}

impl Inputs {
    pub fn read(root: &Path) -> Result<Self> {
        let bytes = std::fs::read(root.join("admission.json"))?;
        let admission: Admission = serde_json::from_slice(&bytes)?;
        admission.validate()?;
        let admission_sha256 = hex(&Sha256::digest(&bytes));
        let classes_root = root.join("classes");
        let mut names = Vec::new();
        visit(&classes_root, &mut names)?;
        names.sort();
        if names.is_empty() {
            return Err("empty class input".into());
        }
        let mut classes = Vec::with_capacity(names.len());
        let mut class_bytes = Vec::with_capacity(names.len());
        let mut manifest = Sha256::new();
        for name in names {
            let path = name
                .strip_prefix(&classes_root)?
                .to_str()
                .ok_or("non-UTF8 class path")?
                .replace('\\', "/");
            let bytes = std::fs::read(name)?;
            let sha256 = hex(&Sha256::digest(&bytes));
            manifest.update(path.as_bytes());
            manifest.update([0]);
            manifest.update(sha256.as_bytes());
            manifest.update(b"\n");
            classes.push(ClassIdentity { path, sha256 });
            class_bytes.push(bytes);
        }
        Ok(Self {
            admission,
            admission_sha256,
            class_bytes,
            classes,
            class_manifest_sha256: hex(&manifest.finalize()),
        })
    }
}

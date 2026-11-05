//! `OpenJDK` resource exhaustion is isolated from the in-process compatibility suite.
//! Run explicitly with `cargo test -p ristretto_vm --test resource_errors
//! resource_error_tests -- --ignored --nocapture`.
#![cfg(not(target_family = "wasm"))]
#![expect(
    clippy::panic_in_result_fn,
    reason = "harness regression tests use assertions"
)]

mod common;

use common::{java, process};
use regex::Regex;
use ristretto_vm::Error::InternalError;
use ristretto_vm::Result;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

const VERSIONS: &[u8] = &[8, 11, 17, 21, 25];
const MEMORY_CASES: &[&str] = &[
    "heap-object",
    "heap-reference",
    "heap-multi-primitive",
    "heap-multi-reference",
    "heap-primitive-boolean",
    "heap-primitive-byte",
    "heap-primitive-char",
    "heap-primitive-short",
    "heap-primitive-int",
    "heap-primitive-long",
    "heap-primitive-float",
    "heap-primitive-double",
    "limit-boolean",
    "limit-byte",
    "limit-char",
    "limit-short",
    "limit-int",
    "limit-long",
    "limit-float",
    "limit-double",
    "limit-reference",
    "multi-outer-limit",
    "multi-inner-limit",
    "multi-reference-limit",
    "multi-limit-before-negative",
    "reflect-array",
    "reflect-multi",
    "clone",
    "string-growth",
    "direct-buffer",
    "native-allocate",
    "native-reallocate",
    "metaspace",
    "constant-pool",
    "compressed-class-space",
    "gc-overhead",
    "initialization",
    "initialization-wrapper",
    "native-thread",
];
const STACK_CASES: &[&str] = &[
    "static",
    "mutual",
    "virtual",
    "special",
    "interface",
    "default-method",
    "constructor",
    "initialization",
    "finally",
    "monitor",
    "synchronized",
    "reflection",
    "method-handle",
    "lambda",
    "invokedynamic",
    "large-frame",
    "operand-stack",
    "thread",
    "uncaught-thread",
    "jni",
    "virtual-thread",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    Memory,
    Stack,
    Contract,
}

impl Family {
    const ALL: [Self; 3] = [Self::Memory, Self::Stack, Self::Contract];

    fn directory(self) -> &'static str {
        match self {
            Self::Memory => "out_of_memory_error",
            Self::Stack => "stack_overflow_error",
            Self::Contract => "resource_error_contract",
        }
    }

    fn cases(self) -> &'static [&'static str] {
        match self {
            Self::Memory => MEMORY_CASES,
            Self::Stack => STACK_CASES,
            Self::Contract => &["controls"],
        }
    }

    fn suffix(self) -> &'static str {
        match self {
            Self::Memory => " java.lang.OutOfMemoryError",
            Self::Stack => " java.lang.StackOverflowError",
            Self::Contract => "",
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Mode {
    Interpreted,
    Compiled,
    Mixed,
}

impl Mode {
    fn name(self) -> &'static str {
        match self {
            Self::Interpreted => "int",
            Self::Compiled => "comp",
            Self::Mixed => "mixed",
        }
    }
}

#[derive(Debug)]
struct Options {
    root: PathBuf,
    output: PathBuf,
    versions: Vec<u8>,
    filter: Regex,
    timeout: Duration,
    skip_native: bool,
}

impl Options {
    fn from_env() -> Result<Self> {
        // Avoid Windows verbatim paths, which older JDKs reject.
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .ok_or_else(|| InternalError("crate directory has no parent".to_string()))?
            .to_path_buf();
        let output = std::env::var_os("RISTRETTO_RESOURCE_OUTPUT").map_or_else(
            || root.join("target/resource-errors"),
            |value| root.join(PathBuf::from(value)),
        );
        let versions = parse_versions(
            &std::env::var("RISTRETTO_RESOURCE_JAVA_VERSIONS")
                .unwrap_or_else(|_| "8,11,17,21,25".to_string()),
        )?;
        let filter = Regex::new(&std::env::var("TEST_FILTER").unwrap_or_else(|_| ".*".to_string()))
            .map_err(|error| InternalError(error.to_string()))?;
        let seconds = std::env::var("RISTRETTO_RESOURCE_TIMEOUT_SECS")
            .unwrap_or_else(|_| "45".to_string())
            .parse::<u64>()
            .map_err(|error| InternalError(error.to_string()))?;
        if seconds == 0 {
            return Err(InternalError(
                "RISTRETTO_RESOURCE_TIMEOUT_SECS must be positive".to_string(),
            ));
        }
        let skip_native = match std::env::var("RISTRETTO_RESOURCE_SKIP_NATIVE").as_deref() {
            Ok("1") => true,
            Err(std::env::VarError::NotPresent) | Ok("0") => false,
            _ => {
                return Err(InternalError(
                    "RISTRETTO_RESOURCE_SKIP_NATIVE must be 0 or 1".to_string(),
                ));
            }
        };
        Ok(Self {
            root,
            output,
            versions,
            filter,
            timeout: Duration::from_secs(seconds),
            skip_native,
        })
    }

    fn fixtures(&self) -> PathBuf {
        self.root.join("tests/exceptions")
    }
}

fn parse_versions(value: &str) -> Result<Vec<u8>> {
    let mut versions = Vec::new();
    for part in value.split(',') {
        let version = part
            .trim()
            .parse::<u8>()
            .map_err(|error| InternalError(error.to_string()))?;
        if !VERSIONS.contains(&version) || versions.contains(&version) {
            return Err(InternalError(
                "select distinct Java versions from 8,11,17,21,25".to_string(),
            ));
        }
        versions.push(version);
    }
    Ok(versions)
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum Status {
    Pass,
    Fail,
    Skip,
}

#[derive(Debug, Serialize)]
struct CaseResult {
    version: u8,
    test: String,
    status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
    #[serde(flatten)]
    output: Option<process::Output>,
}

#[derive(Debug, Default, Serialize)]
struct Summary {
    #[serde(rename = "PASS")]
    passed: usize,
    #[serde(rename = "FAIL")]
    failed: usize,
    #[serde(rename = "SKIP")]
    skipped: usize,
    errors: usize,
}

#[derive(Debug, Serialize)]
struct Report {
    platform: String,
    sources: BTreeMap<String, String>,
    jdks: BTreeMap<String, process::Output>,
    results: Vec<CaseResult>,
    summary: Summary,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[test]
#[ignore = "requires 64-bit OpenJDK 8/11/17/21/25 and a C compiler for Linux JNI fixtures"]
fn resource_error_tests() -> Result<()> {
    let options = Options::from_env()?;
    fs::create_dir_all(&options.output)?;
    let mut report = Report {
        platform: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
        sources: source_hashes(&options)?,
        jdks: BTreeMap::new(),
        results: Vec::new(),
        summary: Summary::default(),
        error: None,
    };
    if let Err(error) = run_matrix(&options, &mut report) {
        report.error = Some(error.to_string());
        report.summary.errors = 1;
    }
    for result in &report.results {
        match result.status {
            Status::Pass => report.summary.passed += 1,
            Status::Fail => report.summary.failed += 1,
            Status::Skip => report.summary.skipped += 1,
        }
    }
    let path = options.output.join("results.json");
    let json =
        serde_json::to_string_pretty(&report).map_err(|error| InternalError(error.to_string()))?;
    fs::write(&path, format!("{json}\n"))?;
    println!("{:?} -> {}", report.summary, path.display());
    if report.summary.failed != 0 || report.summary.errors != 0 {
        return Err(InternalError(format!(
            "resource-error tests failed: {:?}; see {}",
            report.error,
            path.display()
        )));
    }
    Ok(())
}

fn source_hashes(options: &Options) -> Result<BTreeMap<String, String>> {
    let mut sources = vec![
        options.root.join("ristretto_vm/Cargo.toml"),
        options.root.join("ristretto_vm/tests/resource_errors.rs"),
        options.root.join("ristretto_vm/tests/common/mod.rs"),
        options.root.join("ristretto_vm/tests/common/java.rs"),
        options.root.join("ristretto_vm/tests/common/process.rs"),
        options.fixtures().join("resource_errors/native.c"),
    ];
    for family in Family::ALL {
        for entry in fs::read_dir(options.fixtures().join(family.directory()))? {
            let path = entry?.path();
            if path
                .extension()
                .is_some_and(|extension| extension == "java")
            {
                sources.push(path);
            }
        }
    }
    let mut hashes = BTreeMap::new();
    for path in sources {
        let name = path
            .strip_prefix(&options.root)
            .map_err(|error| InternalError(error.to_string()))?
            .to_string_lossy()
            .replace('\\', "/");
        let mut digest = String::with_capacity(64);
        for byte in Sha256::digest(fs::read(path)?) {
            write!(digest, "{byte:02x}").map_err(|error| InternalError(error.to_string()))?;
        }
        hashes.insert(name, digest);
    }
    Ok(hashes)
}

fn run_matrix(options: &Options, report: &mut Report) -> Result<()> {
    for &version in &options.versions {
        let home = jdk_home(version)?;
        let mut java = java::command(&home, "java");
        java.arg("-version").current_dir(&options.output);
        let identity = process::run(&mut java, options.timeout)?;
        identity.require_success()?;
        let banner = format!("{}{}", identity.stdout, identity.stderr);
        report.jdks.insert(version.to_string(), identity);
        validate_jdk(version, &banner)?;
        println!(
            "JDK {version}: {}",
            banner.lines().next().unwrap_or_default()
        );
        let build = options.output.join(version.to_string());
        fs::create_dir_all(&build)?;
        let native = build.join("libresource_errors.so");
        if !options.skip_native {
            compile_native(options, &home, &native)?;
        }
        for family in Family::ALL {
            run_family(options, report, version, &home, &native, family)?;
        }
    }
    if report.results.is_empty() {
        return Err(InternalError(
            "TEST_FILTER selected no resource-error cases".to_string(),
        ));
    }
    Ok(())
}

fn jdk_home(version: u8) -> Result<PathBuf> {
    for variable in [
        format!("JAVA{version}_HOME"),
        format!("JAVA_HOME_{version}_X64"),
    ] {
        if let Some(value) = std::env::var_os(&variable) {
            let home = std::path::absolute(value)?;
            for tool in ["java", "javac"] {
                let executable = home
                    .join("bin")
                    .join(format!("{tool}{}", std::env::consts::EXE_SUFFIX));
                if !executable.is_file() {
                    return Err(InternalError(format!(
                        "{variable}: missing {}",
                        executable.display()
                    )));
                }
            }
            return Ok(home);
        }
    }
    java::home(&version.to_string())
}

fn validate_jdk(version: u8, banner: &str) -> Result<()> {
    let pattern = Regex::new(r#"version "(?:1\.)?(\d+)"#)
        .map_err(|error| InternalError(error.to_string()))?;
    let major = pattern
        .captures(banner)
        .and_then(|captures| captures.get(1))
        .and_then(|major| major.as_str().parse::<u8>().ok());
    if major != Some(version)
        || !banner.contains("64-Bit")
        || !banner.contains("OpenJDK")
        || !banner.contains("Server VM")
    {
        return Err(InternalError(format!(
            "Expected 64-bit OpenJDK HotSpot {version}: {banner}"
        )));
    }
    Ok(())
}

fn compile_native(options: &Options, home: &Path, library: &Path) -> Result<()> {
    if !cfg!(target_os = "linux") {
        return Err(InternalError(
            "JNI fixtures require Linux; set RISTRETTO_RESOURCE_SKIP_NATIVE=1 for explicit skips"
                .to_string(),
        ));
    }
    let compiler = std::env::var_os("CC").unwrap_or_else(|| "cc".into());
    let mut command = Command::new(compiler);
    command
        .args([
            "-shared", "-fPIC", "-O0", "-Wall", "-Wextra", "-Werror", "-I",
        ])
        .arg(home.join("include"))
        .arg("-I")
        .arg(home.join("include/linux"))
        .arg(options.fixtures().join("resource_errors/native.c"))
        .arg("-o")
        .arg(library)
        .current_dir(&options.output);
    process::run(&mut command, options.timeout)?.require_success()
}

fn run_family(
    options: &Options,
    report: &mut Report,
    version: u8,
    home: &Path,
    native: &Path,
    family: Family,
) -> Result<()> {
    let source = options.fixtures().join(family.directory());
    let properties = java::runtime_properties(&source.join("runtime.properties"))?;
    let versions = properties
        .get("java.versions")
        .ok_or_else(|| InternalError("missing java.versions".to_string()))?;
    if !parse_versions(versions)?.contains(&version) {
        return Err(InternalError(format!(
            "{} does not declare Java {version}",
            source.display()
        )));
    }
    let classes = options
        .output
        .join(version.to_string())
        .join(family.directory());
    java::compile(home, &source, &classes, None, true, options.timeout)?;
    for &case in family.cases() {
        let modes: &[Mode] = if family == Family::Stack {
            &[Mode::Interpreted, Mode::Compiled]
        } else if case == "initialization-wrapper" {
            &[Mode::Interpreted]
        } else {
            &[Mode::Mixed]
        };
        for &mode in modes {
            let label = format!("{}/{case}/{}", family.directory(), mode.name());
            if !options.filter.is_match(&label) {
                continue;
            }
            let reason = skip_reason(version, case, options.skip_native);
            let mut result = CaseResult {
                version,
                test: label,
                status: Status::Skip,
                reason: reason.map(str::to_string),
                output: None,
            };
            if reason.is_none() {
                let mut java = java::command(home, "java");
                java.args(jvm_flags(family, case, mode));
                if version >= 25 {
                    java.args([
                        "--enable-native-access=ALL-UNNAMED",
                        "--sun-misc-unsafe-memory-access=allow",
                    ]);
                }
                java.arg(format!("-Dresource.library={}", native.display()))
                    .arg("-cp")
                    .arg(&classes)
                    .args(["Test", case])
                    .current_dir(&options.output);
                let output = process::run(&mut java, options.timeout)?;
                let expected = format!("PASS {case}{}", family.suffix());
                result.status = if passed(&output, &expected) {
                    Status::Pass
                } else {
                    Status::Fail
                };
                if result.status == Status::Fail {
                    eprintln!("{:?}\n{}{}", output.command, output.stdout, output.stderr);
                }
                result.output = Some(output);
            }
            println!("{:?} java{version} {}", result.status, result.test);
            report.results.push(result);
        }
    }
    Ok(())
}

fn skip_reason(version: u8, case: &str, skip_native: bool) -> Option<&'static str> {
    if case == "virtual-thread" && version < 21 {
        Some("virtual threads require Java 21+")
    } else if skip_native && matches!(case, "jni" | "native-thread") {
        Some("explicit RISTRETTO_RESOURCE_SKIP_NATIVE=1")
    } else {
        None
    }
}

fn passed(output: &process::Output, expected: &str) -> bool {
    let mut passes = output
        .stdout
        .lines()
        .filter(|line| line.starts_with("PASS "));
    output.success() && passes.next() == Some(expected) && passes.next().is_none()
}

fn jvm_flags(family: Family, case: &str, mode: Mode) -> Vec<String> {
    let heap = if case == "compressed-class-space" {
        "128m"
    } else if family == Family::Stack || matches!(case, "metaspace" | "constant-pool") {
        "64m"
    } else {
        "16m"
    };
    let mut flags = vec![
        "-Xms16m".to_string(),
        format!("-Xmx{heap}"),
        "-Xss256k".to_string(),
        "-Xverify:all".to_string(),
        "-XX:-HeapDumpOnOutOfMemoryError".to_string(),
    ];
    let collector: &[&str] = if case == "gc-overhead" {
        &[
            "-XX:+UseParallelGC",
            "-XX:ParallelGCThreads=2",
            "-XX:+UseGCOverheadLimit",
            "-XX:GCTimeLimit=1",
            "-XX:GCHeapFreeLimit=100",
        ]
    } else {
        &["-XX:+UseSerialGC", "-XX:-UseGCOverheadLimit"]
    };
    flags.extend(collector.iter().map(ToString::to_string));
    let limits: &[&str] = match case {
        "metaspace" | "constant-pool" => &[
            "-XX:MaxMetaspaceSize=16m",
            "-XX:-UseCompressedClassPointers",
        ],
        "compressed-class-space" => &[
            "-XX:+UseCompressedClassPointers",
            "-XX:CompressedClassSpaceSize=1m",
            "-XX:MaxMetaspaceSize=128m",
        ],
        "direct-buffer" => &["-XX:MaxDirectMemorySize=1m"],
        // Avoid TLAB refill failure leaving enough memory for the VM's error wrapper.
        "initialization-wrapper" => &["-XX:-UseTLAB"],
        _ => &[],
    };
    flags.extend(limits.iter().map(ToString::to_string));
    match mode {
        Mode::Interpreted => flags.push("-Xint".to_string()),
        Mode::Compiled => flags.extend(
            [
                "-Xcomp",
                "-XX:CompileCommand=compileonly,Test*::*",
                "-XX:CompileCommand=compileonly,Generated::*",
            ]
            .map(str::to_string),
        ),
        Mode::Mixed => {}
    }
    flags
}

#[test]
fn rejects_false_passes() {
    let mut output = process::Output {
        command: Vec::new(),
        exit_code: Some(0),
        timed_out: false,
        stdout: "PASS example\n".to_string(),
        stderr: String::new(),
        seconds: 0.0,
    };
    assert!(passed(&output, "PASS example"));
    assert!(!passed(&output, "PASS another"));
    output.stdout.push_str("PASS example\n");
    assert!(!passed(&output, "PASS example"));
    output.stdout = "PASS example\n".to_string();
    output.exit_code = Some(1);
    assert!(!passed(&output, "PASS example"));
    output.exit_code = Some(0);
    output.timed_out = true;
    assert!(!passed(&output, "PASS example"));
}

#[test]
fn validates_versions_and_jvm_identity() -> Result<()> {
    assert_eq!(parse_versions("8,11,17,21,25")?, VERSIONS);
    for invalid in ["", "8,", "7", "8,8", "invalid"] {
        assert!(parse_versions(invalid).is_err());
    }
    let java8 = "openjdk version \"1.8.0_502\"\nOpenJDK 64-Bit Server VM";
    assert!(validate_jdk(8, java8).is_ok());
    assert!(validate_jdk(11, java8).is_err());
    assert!(validate_jdk(8, "openjdk version \"1.8.0_502\"\nOpenJ9 VM").is_err());
    assert!(skip_reason(17, "virtual-thread", false).is_some());
    assert!(skip_reason(21, "virtual-thread", false).is_none());
    assert!(skip_reason(25, "jni", true).is_some());
    Ok(())
}

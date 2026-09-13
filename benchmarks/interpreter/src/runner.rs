//! Compile once, validate and calibrate, then measure independent JVM processes.
#![expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Calibration converts bounded positive batch counts and elapsed times"
)]

use crate::process::{self, Environment};
use crate::report::{empty_directory, write_json, write_report};
use crate::statistics::{RNG_NAME, Random, put, sample_median};
use crate::workloads::{WORKLOADS, Workload};
use anyhow::{Context, Result, ensure};
use clap::Args;
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[derive(Args, Debug, Serialize)]
pub struct Options {
    /// `OpenJDK` installation used by both runtimes.
    #[arg(long)]
    jdk_home: PathBuf,
    #[arg(long)]
    ristretto: Option<PathBuf>,
    #[arg(long)]
    output: Option<PathBuf>,
    #[arg(long, default_value_t = 5)]
    forks: usize,
    #[arg(long, default_value_t = 5)]
    samples: usize,
    #[arg(long, default_value_t = 3)]
    warmups: usize,
    #[arg(long, default_value_t = 20.0)]
    target_ms: f64,
    #[arg(long, default_value_t = 120.0)]
    timeout: f64,
    #[arg(long, default_value_t = 15)]
    startup_runs: usize,
    #[arg(long, default_value_t = 1729)]
    seed: u32,
    #[arg(long, default_value_t = 20_260_912)]
    order_seed: u64,
    /// Comma-separated workload names; always includes the timing control.
    #[arg(long)]
    benchmarks: Option<String>,
    /// Reuse exact work counts from a previous run with identical classes, JDK library and seed.
    #[arg(long)]
    calibration_from: Option<PathBuf>,
    /// Use an already built release Ristretto binary.
    #[arg(long)]
    skip_build: bool,
    /// Compatibility smoke test: one fork/sample/warmup, 1 ms, three startup runs.
    #[arg(long)]
    quick: bool,
}

fn now() -> String {
    jiff::Timestamp::now().to_string()
}

pub fn sha256(path: &Path) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut digest = Sha256::new();
    let mut buffer = vec![0; 64 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        digest.update(buffer.get(..count).context("Invalid read size")?);
    }
    let mut hex = String::with_capacity(64);
    for byte in digest.finalize() {
        use std::fmt::Write as _;
        write!(hex, "{byte:02x}")?;
    }
    Ok(hex)
}

fn args(parts: &[&str]) -> Vec<String> {
    parts.iter().map(ToString::to_string).collect()
}
fn path_text(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn environment(jdk: &Path) -> (Environment, Vec<String>) {
    let mut env: Environment = std::env::vars().collect();
    let mut removed = Vec::new();
    env.retain(|key, _| {
        let remove = key.starts_with("RISTRETTO_JDK")
            || [
                "JAVA_TOOL_OPTIONS",
                "JDK_JAVA_OPTIONS",
                "_JAVA_OPTIONS",
                "JAVA_VERSION",
                "CLASSPATH",
                "RISTRETTO_LOG",
                "RUST_LOG",
                "TOKIO_WORKER_THREADS",
            ]
            .contains(&key.as_str());
        if remove {
            removed.push(key.clone());
        }
        !remove
    });
    for key in ["JAVA_HOME", "RISTRETTO_JDK_HOME"] {
        env.insert(key.into(), path_text(jdk));
    }
    env.insert("LC_ALL".into(), "C".into());
    (env, removed)
}

#[derive(Debug)]
struct Suite {
    root: PathBuf,
    output: PathBuf,
    env: Environment,
    commands: BTreeMap<String, Vec<String>>,
    limit: Duration,
    records: Vec<Value>,
    calibration: Option<Vec<Value>>,
}

#[derive(Clone, Copy, Debug)]
struct Batch {
    units: u32,
    warmups: usize,
    samples: usize,
    seed: u32,
    fork: Option<usize>,
    expected: Option<i64>,
}

impl Suite {
    fn save(&mut self, record: Value) -> Result<()> {
        let mut raw = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.output.join("raw.jsonl"))?;
        serde_json::to_writer(&mut raw, &record)?;
        writeln!(raw)?;
        self.records.push(record);
        Ok(())
    }

    async fn execute(
        &mut self,
        phase: &str,
        runtime: &str,
        work: Workload,
        batch: Batch,
    ) -> Result<(Value, i64)> {
        let mut command = self
            .commands
            .get(runtime)
            .context("Unknown runtime")?
            .clone();
        command.extend([
            "InterpreterBench".into(),
            work.id.to_string(),
            batch.units.to_string(),
            batch.warmups.to_string(),
            batch.samples.to_string(),
            batch.seed.to_string(),
        ]);
        let mut record = json!({"phase":phase,"runtime":runtime,"benchmark":work.name,"units":batch.units,"warmups":batch.warmups,"seed":batch.seed,"fork":batch.fork,"command":command,"started_utc":now()});
        let result = async {
            let result =
                process::run_fork(&command, &self.env, &self.root, self.limit, batch.samples)
                    .await?;
            let value = serde_json::to_value(&result)?;
            record
                .as_object_mut()
                .context("Invalid record")?
                .extend(value.as_object().context("Invalid fork")?.clone());
            process::verify(&result.samples, batch.expected)
        }
        .await;
        match result {
            Ok(checksum) => {
                put(&mut record, "status", json!("ok"))?;
                self.save(record.clone())?;
                Ok((record, checksum))
            }
            Err(error) => {
                put(&mut record, "status", json!("error"))?;
                put(&mut record, "error", json!(format!("{error:#}")))?;
                self.save(record)?;
                Err(error)
            }
        }
    }

    async fn calibrate(&mut self, work: Workload, options: &Options) -> Result<(u32, i64)> {
        for seed in [options.seed, options.seed ^ 0x005a_5a5a] {
            let batch = Batch {
                units: 1,
                warmups: 1,
                samples: 2,
                seed,
                fork: None,
                expected: None,
            };
            let (_, checksum) = self.execute("validation", "openjdk", work, batch).await?;
            self.execute(
                "validation",
                "ristretto",
                work,
                Batch {
                    expected: Some(checksum),
                    ..batch
                },
            )
            .await?;
        }
        let fixed_units = self
            .calibration
            .as_ref()
            .map(|rows| saved_units(rows, work.name))
            .transpose()?;
        let mut units = fixed_units.unwrap_or(1);
        let mut calibrated = (Value::Null, 0);
        for attempt in 0..=5 {
            let batch = Batch {
                units,
                warmups: 1,
                samples: 3,
                seed: options.seed,
                fork: None,
                expected: None,
            };
            calibrated = self.execute("calibration", "openjdk", work, batch).await?;
            let elapsed = sample_median(&calibrated.0)?;
            if fixed_units.is_some()
                || work.id == 0
                || elapsed >= options.target_ms * 1e6 * 0.9
                || attempt == 5
            {
                break;
            }
            units = (f64::from(units) * options.target_ms * 1e6 / elapsed * 1.1)
                .ceil()
                .clamp(
                    f64::from(units.saturating_add(1).min(10_000_000)),
                    10_000_000.0,
                ) as u32;
        }
        let pilot_batch = Batch {
            units,
            warmups: 1,
            samples: 1,
            seed: options.seed,
            fork: None,
            expected: Some(calibrated.1),
        };
        let (pilot, _) = self
            .execute("pilot", "ristretto", work, pilot_batch)
            .await?;
        println!(
            "  {}: {units} units; OpenJDK {:.2} ms; Ristretto pilot {:.2} ms",
            work.name,
            sample_median(&calibrated.0)? / 1e6,
            sample_median(&pilot)? / 1e6
        );
        Ok((units, calibrated.1))
    }

    async fn startup(&mut self, options: &Options, rng: &mut Random) -> Result<()> {
        let mut no_cds = self
            .commands
            .get("openjdk")
            .context("Missing OpenJDK")?
            .clone();
        no_cds.insert(2, "-Xshare:off".into());
        self.commands.insert("openjdk_no_cds".into(), no_cds);
        for iteration in 0..=options.startup_runs {
            let mut order = ["StartupEmpty", "StartupHello"]
                .into_iter()
                .flat_map(|program| {
                    ["openjdk", "openjdk_no_cds", "ristretto"].map(|runtime| (program, runtime))
                })
                .collect::<Vec<_>>();
            rng.shuffle(&mut order)?;
            for (program, runtime) in order {
                let mut command = self
                    .commands
                    .get(runtime)
                    .context("Missing runtime")?
                    .clone();
                command.push(program.into());
                let phase = if iteration == 0 {
                    "startup_warmup"
                } else {
                    "startup"
                };
                let mut record = json!({"phase":phase,"program":program,"runtime":runtime,"iteration":iteration,"command":command});
                let begin = Instant::now();
                let result = process::capture(&command, &self.env, &self.root, self.limit).await;
                put(
                    &mut record,
                    "elapsed_ns",
                    json!(process::elapsed_ns(begin)?),
                )?;
                let result = result.and_then(|(status, stdout, stderr)| {
                    let expected = if program == "StartupEmpty" {
                        ""
                    } else {
                        "Hello, interpreter!\n"
                    };
                    ensure!(
                        status.success() && stdout == expected,
                        "Bad startup result: {status}; stdout: {stdout}; stderr: {stderr}"
                    );
                    Ok(stderr)
                });
                match result {
                    Ok(stderr) => {
                        put(&mut record, "status", json!("ok"))?;
                        put(&mut record, "stderr", json!(stderr))?;
                    }
                    Err(error) => {
                        put(&mut record, "status", json!("error"))?;
                        put(&mut record, "error", json!(format!("{error:#}")))?;
                    }
                }
                self.save(record)?;
            }
        }
        Ok(())
    }
}

fn source_hashes(here: &Path) -> Result<BTreeMap<String, String>> {
    let mut files = Vec::new();
    for directory in [here.to_path_buf(), here.join("src")] {
        for entry in std::fs::read_dir(directory)? {
            let path = entry?.path();
            if path
                .extension()
                .is_some_and(|ext| ext == "rs" || ext == "java" || ext == "toml")
            {
                files.push(path);
            }
        }
    }
    files
        .into_iter()
        .map(|path| Ok((path_text(path.strip_prefix(here)?), sha256(&path)?)))
        .collect()
}

fn vm_source_hashes(root: &Path) -> Result<BTreeMap<String, String>> {
    let mut directories = vec![root.join("ristretto_vm/src")];
    let mut hashes = BTreeMap::new();
    while let Some(directory) = directories.pop() {
        for entry in std::fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                directories.push(path);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                hashes.insert(path_text(path.strip_prefix(root)?), sha256(&path)?);
            }
        }
    }
    Ok(hashes)
}

fn saved_units(rows: &[Value], name: &str) -> Result<u32> {
    let row = rows
        .iter()
        .find(|r| r["benchmark"] == name)
        .with_context(|| format!("Missing calibration for {name}"))?;
    ensure!(row["complete"] == true, "Incomplete calibration for {name}");
    let units = row["units"].as_u64().context("Invalid saved unit count")?;
    ensure!(
        (1..=10_000_000).contains(&units),
        "Saved unit count out of range"
    );
    Ok(u32::try_from(units)?)
}

#[expect(
    clippy::too_many_lines,
    reason = "Sequential setup and measurement phases share immutable runtime configuration"
)]
pub async fn run(mut options: Options) -> Result<()> {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = here.join("../..").canonicalize()?;
    if options.quick {
        options.forks = 1;
        options.samples = 1;
        options.warmups = 1;
        options.target_ms = 1.0;
        options.startup_runs = 3;
    }
    ensure!(
        options.forks > 0 && options.samples > 0 && options.startup_runs > 0,
        "Counts must be positive"
    );
    ensure!(
        options.target_ms.is_finite() && options.target_ms > 0.0,
        "Target must be positive and finite"
    );
    ensure!(
        options.timeout.is_finite() && options.timeout > 0.0,
        "Timeout must be positive and finite"
    );
    ensure!(
        i32::try_from(options.seed).is_ok(),
        "Seed must fit a nonnegative Java int"
    );
    options.jdk_home = options.jdk_home.canonicalize()?;
    let output = options.output.clone().unwrap_or_else(|| {
        root.join("target/interpreter-bench")
            .join(now().replace(':', "-"))
    });
    empty_directory(&output)?;
    let output = output.canonicalize()?;
    let (env, removed) = environment(&options.jdk_home);
    if !options.skip_build {
        println!("Building release Ristretto...");
        let status = std::process::Command::new("cargo")
            .args(["build", "--release", "--locked", "-p", "ristretto_java"])
            .current_dir(&root)
            .status()?;
        ensure!(status.success(), "Release build failed");
    }
    let ristretto = options
        .ristretto
        .clone()
        .unwrap_or_else(|| root.join("target/release/java"))
        .canonicalize()?;
    options.ristretto = Some(ristretto.clone());
    options.output = Some(output.clone());
    let selected = if let Some(names) = &options.benchmarks {
        let names = names.split(',').chain(["control"]).collect::<BTreeSet<_>>();
        ensure!(
            names.iter().all(|n| WORKLOADS.iter().any(|w| &w.name == n)),
            "Unknown benchmark name"
        );
        WORKLOADS
            .iter()
            .copied()
            .filter(|w| names.contains(w.name))
            .collect::<Vec<_>>()
    } else {
        WORKLOADS.to_vec()
    };
    let classes_parent = root.join("target/interpreter-bench/classes");
    std::fs::create_dir_all(&classes_parent)?;
    let classes = tempfile::Builder::new()
        .prefix("rust-")
        .tempdir_in(classes_parent)?
        .keep();
    let java = options.jdk_home.join("bin/java");
    let javac = options.jdk_home.join("bin/javac");
    let disassembler = options.jdk_home.join("bin/javap");
    let mut compile = vec![
        path_text(&javac),
        "--release".into(),
        "8".into(),
        "-Xlint:-options".into(),
        "-d".into(),
        path_text(&classes),
    ];
    for source in [
        "InterpreterBench.java",
        "StartupEmpty.java",
        "StartupHello.java",
    ] {
        compile.push(path_text(&here.join(source)));
    }
    process::output(&compile, &env, &root).await?;
    let bytecode = vec![
        path_text(&disassembler),
        "-c".into(),
        "-p".into(),
        "-classpath".into(),
        path_text(&classes),
        "InterpreterBench".into(),
    ];
    std::fs::write(
        output.join("bytecode.txt"),
        process::output(&bytecode, &env, &root).await?,
    )?;
    let mut commands = BTreeMap::new();
    for (runtime, path) in [("openjdk", &java), ("ristretto", &ristretto)] {
        commands.insert(
            runtime.to_owned(),
            vec![
                path_text(path),
                "-Xint".into(),
                "-cp".into(),
                path_text(&classes),
            ],
        );
    }
    let (mut versions, mut runtime_info) = (BTreeMap::new(), BTreeMap::new());
    for (runtime, command) in &commands {
        let mut version_command = command.get(..2).context("Incomplete command")?.to_vec();
        version_command.push("-version".into());
        versions.insert(
            runtime.clone(),
            process::output(&version_command, &env, &root).await?,
        );
        let mut info_command = command.clone();
        info_command.extend(args(&["InterpreterBench", "info"]));
        let info = process::output(&info_command, &env, &root).await?;
        let home = info
            .lines()
            .find_map(|line| line.strip_prefix("java.home="))
            .context("Missing java.home")?;
        ensure!(
            Path::new(home).canonicalize()? == options.jdk_home,
            "{runtime} uses the wrong JDK library"
        );
        runtime_info.insert(runtime.clone(), info);
    }
    let reference = versions.get("openjdk").context("Missing OpenJDK version")?;
    ensure!(
        reference.contains("OpenJDK") && reference.contains("interpreted mode"),
        "Reference must be OpenJDK in interpreted mode"
    );
    versions.insert(
        "rustc".into(),
        process::output(&args(&["rustc", "--version"]), &env, &root).await?,
    );
    versions.insert(
        "javac".into(),
        process::output(&[path_text(&javac), "-version".into()], &env, &root).await?,
    );
    let cpu_command = if cfg!(target_os = "macos") {
        args(&["sysctl", "-n", "machdep.cpu.brand_string"])
    } else {
        args(&["lscpu"])
    };
    let cpu = process::output(&cpu_command, &env, &root).await.ok();
    let memory = if cfg!(target_os = "macos") {
        process::output(&args(&["sysctl", "-n", "hw.memsize"]), &env, &root)
            .await?
            .parse::<u64>()?
    } else {
        let info = std::fs::read_to_string("/proc/meminfo")?;
        info.lines()
            .find_map(|line| line.strip_prefix("MemTotal:"))
            .and_then(|s| s.split_whitespace().next())
            .context("Missing total memory")?
            .parse::<u64>()?
            * 1024
    };
    let power = if cfg!(target_os = "macos") {
        process::output(&args(&["pmset", "-g", "batt"]), &env, &root)
            .await
            .ok()
    } else {
        None
    };
    let class_hashes = std::fs::read_dir(&classes)?
        .map(|entry| {
            let path = entry?.path();
            Ok((
                path.file_name()
                    .context("Missing class filename")?
                    .to_string_lossy()
                    .into_owned(),
                sha256(&path)?,
            ))
        })
        .collect::<Result<BTreeMap<_, _>>>()?;
    let executable = std::env::current_exe()?;
    let mut metadata = json!({"started_utc":now(),"platform":process::output(&args(&["uname","-a"]),&env,&root).await?,"cpu":cpu,"memory_bytes":memory,
        "logical_cpus":std::thread::available_parallelism()?.get(),"power":power,"options":options,
        "git_commit":process::output(&args(&["git","rev-parse","HEAD"]),&env,&root).await?,
        "git_status":process::output(&args(&["git","status","--short"]),&env,&root).await?,
        "invocation":std::env::args().collect::<Vec<_>>(),"versions":versions,"runtime_info":runtime_info,"commands":commands,"compile_command":compile,"bytecode_command":bytecode,
        "environment_overrides":{"JAVA_HOME":path_text(&options.jdk_home),"RISTRETTO_JDK_HOME":path_text(&options.jdk_home),"LC_ALL":"C"},"removed_environment_keys":removed,
        "harness":{"language":"Rust","timer":"std::time::Instant","rng":RNG_NAME,"resources":"/usr/bin/time -p -l (macOS), -v (Linux); separate output file; outside kernel timing"},
        "sha256":{"ristretto":sha256(&ristretto)?,"openjdk_java":sha256(&java)?,"jdk_modules":sha256(&options.jdk_home.join("lib/modules"))?,"cargo_lock":sha256(&root.join("Cargo.lock"))?,"harness_binary":sha256(&executable)?,"sources":source_hashes(&here)?,"vm_sources":vm_source_hashes(&root)?,"classes":class_hashes}});
    let calibration = if let Some(directory) = &options.calibration_from {
        let saved: Value =
            serde_json::from_slice(&std::fs::read(directory.join("metadata.json"))?)?;
        for field in ["/sha256/classes", "/sha256/jdk_modules", "/options/seed"] {
            ensure!(
                saved.pointer(field) == metadata.pointer(field),
                "Calibration mismatch: {field}"
            );
        }
        let rows: Vec<Value> =
            serde_json::from_slice(&std::fs::read(directory.join("summary.json"))?)?;
        for work in &selected {
            saved_units(&rows, work.name)?;
        }
        put(
            &mut metadata,
            "calibration_source",
            json!({
                "directory": directory.canonicalize()?,
                "metadata_sha256": sha256(&directory.join("metadata.json"))?,
                "summary_sha256": sha256(&directory.join("summary.json"))?,
            }),
        )?;
        Some(rows)
    } else {
        None
    };
    write_json(&output.join("metadata.json"), &metadata)?;
    println!("Results: {}", output.display());
    let mut suite = Suite {
        root,
        output: output.clone(),
        env,
        commands,
        limit: Duration::try_from_secs_f64(options.timeout)?,
        records: Vec::new(),
        calibration,
    };
    let mut calibrated = BTreeMap::new();
    for work in &selected {
        match suite.calibrate(*work, &options).await {
            Ok(batch) => {
                calibrated.insert(work.name, batch);
            }
            Err(error) => eprintln!("  FAILED {}: {error:#}", work.name),
        }
    }
    let mut rng = Random::new(options.order_seed);
    for fork in 0..options.forks {
        let mut order = selected
            .iter()
            .copied()
            .filter(|w| calibrated.contains_key(w.name))
            .collect::<Vec<_>>();
        rng.shuffle(&mut order)?;
        for work in order {
            let (units, checksum) = *calibrated.get(work.name).context("Missing calibration")?;
            let mut runtimes = ["openjdk", "ristretto"];
            rng.shuffle(&mut runtimes)?;
            for runtime in runtimes {
                let batch = Batch {
                    units,
                    warmups: options.warmups,
                    samples: options.samples,
                    seed: options.seed,
                    fork: Some(fork),
                    expected: Some(checksum),
                };
                match suite.execute("measurement", runtime, work, batch).await {
                    Ok((record, _)) => println!(
                        "  fork {}/{} {} {runtime}: {:.2} ms",
                        fork + 1,
                        options.forks,
                        work.name,
                        sample_median(&record)? / 1e6
                    ),
                    Err(error) => eprintln!("  FAILED {} {runtime}: {error:#}", work.name),
                }
            }
        }
        write_report(&output, &metadata, &suite.records, &selected)?;
    }
    suite.startup(&options, &mut rng).await?;
    put(&mut metadata, "finished_utc", json!(now()))?;
    let unchanged = sha256(&ristretto)?
        == metadata
            .pointer("/sha256/ristretto")
            .and_then(Value::as_str)
            .context("Missing binary hash")?;
    put(&mut metadata, "binary_unchanged", json!(unchanged))?;
    write_json(&output.join("metadata.json"), &metadata)?;
    let rows = write_report(&output, &metadata, &suite.records, &selected)?;
    println!("Report: {}", output.join("report.md").display());
    ensure!(
        unchanged
            && rows.len() == selected.len()
            && rows.iter().all(|r| r["complete"] == true)
            && suite.records.iter().all(|r| r["status"] == "ok"),
        "Benchmark run failed or is incomplete; see raw.jsonl and report.md"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "Test assertions accompany fallible fixture construction"
    )]
    fn calibration_requires_complete_and_bounded_work_counts() -> Result<()> {
        let mut row = json!({"benchmark":"control","complete":true,"units":17});
        assert_eq!(saved_units(&[row.clone()], "control")?, 17);
        assert!(saved_units(&[row.clone()], "missing").is_err());
        for invalid in [
            json!(0),
            json!(-1),
            json!(10_000_001),
            json!(1.5),
            Value::Null,
        ] {
            put(&mut row, "units", invalid)?;
            assert!(saved_units(&[row.clone()], "control").is_err());
        }
        put(&mut row, "units", json!(17))?;
        put(&mut row, "complete", json!(false))?;
        assert!(saved_units(&[row], "control").is_err());
        Ok(())
    }
}

//! Exercise the host CLI with deterministic stand-ins for the external Java tools.
#![expect(
    clippy::panic_in_result_fn,
    reason = "Tests assert observable artifacts and propagate fixture setup errors"
)]

use super::*;
use anyhow::Context;
use serde_json::{Value, json};
use std::ffi::OsString;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

struct Fixture {
    directory: tempfile::TempDir,
}

impl Fixture {
    fn new() -> Result<Self> {
        let directory = tempfile::tempdir()?;
        let root = directory.path();
        std::fs::create_dir_all(root.join("bin"))?;
        std::fs::create_dir(root.join("lib"))?;
        std::fs::write(root.join("lib/modules"), b"fixture JDK library")?;
        let script = r#"#!/bin/sh
mode=
if test -f "$JAVA_HOME/mode"; then read -r mode < "$JAVA_HOME/mode"; fi
case "$0" in
  */javac)
    if test "$1" = -version; then echo 'javac fixture'; exit; fi
    while test "$1" != -d; do shift; done
    printf 'fixture bytecode' > "$2/InterpreterBench.class"
    exit ;;
  */javap) echo 'fixture disassembly'; exit ;;
esac
while test "$#" -gt 0; do
  case "$1" in
    -version) echo 'OpenJDK fixture (interpreted mode)' >&2; exit ;;
    -cp) shift 2 ;;
    -X*) shift ;;
    *) break ;;
  esac
done
case "$1" in
  StartupEmpty) exit ;;
  StartupHello)
    if test "$mode" = startup-error; then echo 'startup failure' >&2; exit 1; fi
    echo 'Hello, interpreter!'; exit ;;
  InterpreterBench) shift ;;
  *) echo 'unexpected command' >&2; exit 1 ;;
esac
if test "$1" = info; then
  if test "$mode" = wrong-library; then echo 'java.home=/'; else echo "java.home=$JAVA_HOME"; fi
  echo 'java.version=fixture'
  exit
fi
checksum=$(($1 * $2 + $5))
case "$0" in
  */ristretto)
    if test "$mode" = bad-checksum; then checksum=$((checksum + 1)); fi ;;
esac
printf 'H\n'
read -r start || exit 1
test "$start" = G || exit 1
count=0
while test "$count" -lt "$4"; do
  printf 'R\n'
  read -r start || exit 1
  test "$start" = G || exit 1
  printf 'D\n%s\n' "$checksum"
  count=$((count + 1))
done
"#;
        for name in ["java", "javac", "javap", "ristretto"] {
            let path = root.join("bin").join(name);
            std::fs::write(&path, script)?;
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))?;
        }
        Ok(Self { directory })
    }

    fn path(&self, name: &str) -> PathBuf {
        self.directory.path().join(name)
    }

    fn options(&self, name: &str) -> Vec<OsString> {
        vec![
            "interpreter-bench".into(),
            "run".into(),
            "--jdk-home".into(),
            self.directory.path().into(),
            "--ristretto".into(),
            self.path("bin/ristretto").into(),
            "--output".into(),
            self.path(name).into(),
            "--skip-build".into(),
            "--quick".into(),
            "--benchmarks".into(),
            "int_arithmetic".into(),
        ]
    }

    async fn run(&self, name: &str) -> Result<()> {
        Cli::try_parse_from(self.options(name))?.execute().await
    }
}

fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&std::fs::read(path)?)?)
}

#[tokio::test]
async fn run_replay_plot_and_reuse_calibration() -> Result<()> {
    let fixture = Fixture::new()?;
    fixture.run("first").await?;
    let first = fixture.path("first");
    let metadata = read_json(&first.join("metadata.json"))?;
    assert_eq!(metadata.get("binary_unchanged"), Some(&json!(true)));
    assert_eq!(metadata.pointer("/options/forks"), Some(&json!(1)));
    assert_eq!(metadata.pointer("/options/startup_runs"), Some(&json!(3)));
    assert_eq!(
        metadata.pointer("/runtime_info/ristretto"),
        metadata.pointer("/runtime_info/openjdk")
    );
    assert_eq!(
        std::fs::read_to_string(first.join("bytecode.txt"))?,
        "fixture disassembly"
    );
    let summary = read_json(&first.join("summary.json"))?;
    let rows = summary.as_array().context("Missing summary rows")?;
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|row| row["complete"] == true));
    assert!(rows.iter().any(|row| row["benchmark"] == "control"));
    let raw = std::fs::read_to_string(first.join("raw.jsonl"))?;
    let records = raw
        .lines()
        .map(serde_json::from_str::<Value>)
        .collect::<std::result::Result<Vec<_>, _>>()?;
    assert!(records.iter().all(|record| record["status"] == "ok"));
    assert_eq!(
        records
            .iter()
            .filter(|record| record["phase"] == "startup")
            .count(),
        18
    );
    assert!(
        records
            .iter()
            .any(|record| record["phase"] == "measurement")
    );

    Cli::try_parse_from([
        OsString::from("bench"),
        "report".into(),
        first.clone().into(),
        "--output".into(),
        fixture.path("replay").into(),
    ])?
    .execute()
    .await?;
    for file in ["summary.json", "summary.csv", "report.md"] {
        assert_eq!(
            std::fs::read(first.join(file))?,
            std::fs::read(fixture.path("replay").join(file))?
        );
    }
    Cli::try_parse_from([OsString::from("bench"), "plot".into(), first.clone().into()])?
        .execute()
        .await?;
    assert!(std::fs::read_to_string(first.join("ratios.svg"))?.contains("int arithmetic"));
    assert!(std::fs::read(first.join("ratios.png"))?.starts_with(b"\x89PNG\r\n\x1a\n"));

    let mut arguments = fixture.options("reused");
    arguments.extend([OsString::from("--calibration-from"), first.into()]);
    Cli::try_parse_from(arguments)?.execute().await?;
    let reused = read_json(&fixture.path("reused/summary.json"))?;
    for (before, after) in rows
        .iter()
        .zip(reused.as_array().context("Missing reused rows")?)
    {
        assert_eq!(before["units"], after["units"]);
    }
    let report = std::fs::read_to_string(fixture.path("reused/report.md"))?;
    assert!(report.contains("Work counts are reused exactly"));
    assert!(report.contains("Recorded failures: 0"));

    std::fs::write(fixture.path("lib/modules"), b"different JDK library")?;
    let mut arguments = fixture.options("mismatch");
    arguments.extend([
        OsString::from("--calibration-from"),
        fixture.path("first").into(),
    ]);
    let error = Cli::try_parse_from(arguments)?
        .execute()
        .await
        .expect_err("Changed JDK must reject calibration");
    assert!(error.to_string().contains("Calibration mismatch"));
    Ok(())
}

#[tokio::test]
async fn failed_runs_preserve_diagnostics_and_reject_wrong_library() -> Result<()> {
    let fixture = Fixture::new()?;
    for mode in ["bad-checksum", "startup-error"] {
        std::fs::write(fixture.path("mode"), format!("{mode}\n"))?;
        let error = fixture
            .run(mode)
            .await
            .expect_err("Failed VMs must fail the run");
        assert!(
            error
                .to_string()
                .contains("Benchmark run failed or is incomplete")
        );
        let raw = std::fs::read_to_string(fixture.path(mode).join("raw.jsonl"))?;
        let expected = if mode == "bad-checksum" {
            "Checksum mismatch"
        } else {
            "startup failure"
        };
        assert!(raw.contains(expected));
        assert!(std::fs::read_to_string(fixture.path(mode).join("report.md"))?.contains(expected));
    }
    std::fs::write(fixture.path("mode"), b"wrong-library\n")?;
    let error = fixture
        .run("wrong-library")
        .await
        .expect_err("Wrong JDK must be rejected");
    assert!(error.to_string().contains("uses the wrong JDK library"));
    Ok(())
}

#[tokio::test]
async fn rejects_invalid_run_options() -> Result<()> {
    let fixture = Fixture::new()?;
    for (flag, value, expected) in [
        ("--forks", "0", "Counts must be positive"),
        ("--samples", "0", "Counts must be positive"),
        ("--startup-runs", "0", "Counts must be positive"),
        ("--target-ms", "0", "Target must be positive and finite"),
        ("--timeout", "NaN", "Timeout must be positive and finite"),
        ("--seed", "2147483648", "Seed must fit"),
    ] {
        let mut arguments = fixture.options("invalid");
        arguments.retain(|argument| argument != "--quick");
        arguments.extend([flag.into(), value.into()]);
        let error = Cli::try_parse_from(arguments)?
            .execute()
            .await
            .expect_err("Invalid options must be rejected");
        assert!(error.to_string().contains(expected), "{error:#}");
    }
    let mut arguments = fixture.options("unknown-workload");
    arguments.pop();
    arguments.push("nonexistent".into());
    let error = Cli::try_parse_from(arguments)?
        .execute()
        .await
        .expect_err("Unknown workload must be rejected");
    assert!(error.to_string().contains("Unknown benchmark name"));
    Ok(())
}

#[tokio::test]
async fn charts_filter_incomplete_results_and_validate_ratios() -> Result<()> {
    let fixture = Fixture::new()?;
    let input = fixture.path("charts");
    std::fs::create_dir(&input)?;
    report::write_json(&input.join("metadata.json"), &json!({}))?;
    let mut rows = json!([
        {"benchmark":"exception_fresh","complete":true,"ratio":4.0,"ratio_ci_low":3.0,"ratio_ci_high":5.0},
        {"benchmark":"int_arithmetic","complete":true,"ratio":0.5},
        {"benchmark":"control","complete":true,"ratio":1.0},
        {"benchmark":"incomplete","complete":false,"ratio":2.0}
    ]);
    report::write_json(&input.join("summary.json"), &rows)?;
    Cli::try_parse_from([
        OsString::from("bench"),
        "plot".into(),
        input.clone().into(),
        "--output".into(),
        fixture.path("images").into(),
    ])?
    .execute()
    .await?;
    let svg = std::fs::read_to_string(fixture.path("images/ratios.svg"))?;
    assert!(svg.contains("exception fresh*"));
    assert!(svg.contains("Fresh exceptions: finite-run result"));
    assert!(svg.contains("int arithmetic"));
    assert!(!svg.contains("incomplete"));
    assert!(!svg.contains(">control<"));
    *rows.pointer_mut("/0/ratio").context("Missing ratio")? = json!(0);
    report::write_json(&input.join("summary.json"), &rows)?;
    assert!(chart::plot(&input, &fixture.path("invalid-chart")).is_err());
    report::write_json(&input.join("summary.json"), &json!([]))?;
    assert!(chart::plot(&input, &fixture.path("empty-chart")).is_err());
    Ok(())
}

#[test]
fn reports_distinguish_complete_and_partial_measurements() -> Result<()> {
    let fixture = Fixture::new()?;
    let output = fixture.path("report");
    report::empty_directory(&output)?;
    let metadata = json!({
        "options":{"forks":3,"samples":2,"warmups":1,"target_ms":20},
        "started_utc":"2026-09-13T00:00:00Z", "git_commit":"fixture", "git_status":"",
        "versions":{"openjdk":"OpenJDK", "ristretto":"Ristretto", "rustc":"rustc"}
    });
    let mut records = Vec::new();
    for (name, forks) in [
        ("control", 3),
        ("int_arithmetic", 3),
        ("long_arithmetic", 1),
    ] {
        for (runtime, factor) in [("openjdk", 1), ("ristretto", 2)] {
            for fork in 0..forks {
                records.push(json!({
                    "phase":"measurement", "status":"ok", "benchmark":name,
                    "runtime":runtime, "units":1, "peak_rss_bytes":4096,
                    "samples":[{"elapsed_ns":(fork + 1) * factor * 1000}]
                }));
            }
        }
    }
    report::write_json(&output.join("metadata.json"), &metadata)?;
    let rows = report::write_report(&output, &metadata, &records, workloads::WORKLOADS)?;
    assert_eq!(rows.len(), 3);
    let complete = rows
        .iter()
        .find(|row| row["benchmark"] == "int_arithmetic")
        .context("Missing complete row")?;
    assert_eq!(complete["ratio"], 2.0);
    assert!(complete["ratio_ci_low"].is_number());
    let partial = rows
        .iter()
        .find(|row| row["benchmark"] == "long_arithmetic")
        .context("Missing partial row")?;
    assert_eq!(partial["complete"], false);
    assert!(partial["ratio_ci_low"].is_null());
    let text = std::fs::read_to_string(output.join("report.md"))?;
    assert!(text.contains("long_arithmetic (incomplete)"));
    assert!(text.contains("geometric mean across 1 complete workloads"));
    assert!(!text.contains("working tree has local changes"));
    assert!(report::empty_directory(&output).is_err());
    std::fs::write(output.join("raw.jsonl"), "{\"benchmark\":\"unknown\"}\n")?;
    assert!(report::replay(&output, &fixture.path("unknown")).is_err());
    Ok(())
}

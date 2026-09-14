//! Human-readable reports and machine-readable summaries; old raw files remain readable.
#![expect(
    clippy::cast_precision_loss,
    reason = "Workload counts are small and descriptive means use floating point"
)]

use crate::statistics::{RNG_NAME, median, number, string, summarize};
use crate::workloads::{WORKLOADS, Workload};
use anyhow::{Context, Result, ensure};
use serde_json::Value;
use std::fmt::Write;
use std::path::Path;

pub fn write_json(path: &Path, value: &impl serde::Serialize) -> Result<()> {
    let mut text = serde_json::to_string_pretty(value)?;
    text.push('\n');
    std::fs::write(path, text)?;
    Ok(())
}

pub fn empty_directory(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)?;
    ensure!(
        std::fs::read_dir(path)?.next().is_none(),
        "Output directory must be empty: {}",
        path.display()
    );
    Ok(())
}

fn csv_field(value: &Value) -> String {
    if value.is_null() {
        return String::new();
    }
    let text = value
        .as_str()
        .map_or_else(|| value.to_string(), str::to_owned);
    if text.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", text.replace('"', "\"\""))
    } else {
        text
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "The report follows the same order as the output document"
)]
pub fn write_report(
    output: &Path,
    metadata: &Value,
    records: &[Value],
    selected: &[Workload],
) -> Result<Vec<Value>> {
    let forks = usize::try_from(
        metadata
            .pointer("/options/forks")
            .unwrap_or(&Value::Null)
            .as_u64()
            .context("Missing fork count")?,
    )?;
    let rows = summarize(records, selected, forks)?;
    write_json(&output.join("summary.json"), &rows)?;
    if let Some(first) = rows.first() {
        let fields = first
            .as_object()
            .context("Invalid summary")?
            .keys()
            .collect::<Vec<_>>();
        let mut csv = fields
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(",");
        csv.push('\n');
        for row in &rows {
            writeln!(
                csv,
                "{}",
                fields
                    .iter()
                    .map(|key| csv_field(&row[*key]))
                    .collect::<Vec<_>>()
                    .join(",")
            )?;
        }
        std::fs::write(output.join("summary.csv"), csv)?;
    }
    let mut text = String::from("# Ristretto / OpenJDK interpreter benchmark results\n\n");
    writeln!(
        text,
        "Run: {}; checkout based on commit `{}`.\n",
        string(metadata, "started_utc")?,
        string(metadata, "git_commit")?
    )?;
    if metadata["git_status"]
        .as_str()
        .is_some_and(|s| !s.is_empty())
    {
        text.push_str("The measured working tree has local changes. See metadata.json for checkout status, executable digests and source hashes.\n\n");
    }
    text.push_str("Both execute the same class files and JDK library with `-Xint`. Ratios are Ristretto time / OpenJDK time; higher means Ristretto is slower.\n\nEach result is the median of per-process medians. A host monotonic clock times the pipe handshake and kernel; no overhead is subtracted. Approximate 95% intervals bootstrap independent process medians (10,000 resamples). Few forks limit precision.\n\n");
    writeln!(
        text,
        "Configuration: {forks} processes/runtime/workload; {} untimed batches and {} measured batches/process; OpenJDK target {} ms.\n",
        metadata.pointer("/options/warmups").unwrap_or(&Value::Null),
        metadata.pointer("/options/samples").unwrap_or(&Value::Null),
        metadata
            .pointer("/options/target_ms")
            .unwrap_or(&Value::Null)
    )?;
    if metadata.get("calibration_source").is_some() {
        text.push_str("Work counts are reused exactly from the prior run recorded in `calibration_source` in metadata.json; this run does not rescale them to the target duration.\n\n");
    }
    writeln!(
        text,
        "Host: {}; CPU: {}; memory: {} bytes.\n",
        metadata["platform"], metadata["cpu"], metadata["memory_bytes"]
    )?;
    writeln!(
        text,
        "```text\n{}\n{}\n{}\n```\n",
        string(&metadata["versions"], "openjdk")?,
        string(&metadata["versions"], "ristretto")?,
        string(&metadata["versions"], "rustc")?
    )?;
    text.push_str("Ristretto's version banner includes its compiled-in default JDK version. `runtime_info` in metadata identifies the library actually used by both VMs.\n\n## Kernel results\n\nOne unit is the fixed work described in the suite README, not one bytecode.\n\n| Workload | Units/batch | OpenJDK µs/unit | Ristretto µs/unit | Ratio | 95% interval |\n|---|---:|---:|---:|---:|---:|\n");
    for row in &rows {
        let interval = match (row["ratio_ci_low"].as_f64(), row["ratio_ci_high"].as_f64()) {
            (Some(low), Some(high)) => format!("{low:.2}–{high:.2}"),
            _ => "—".into(),
        };
        let suffix = if row["complete"] == true {
            ""
        } else {
            " (incomplete)"
        };
        writeln!(
            text,
            "| {}{suffix} | {} | {:.3} | {:.3} | {:.2}× | {interval} |",
            string(row, "benchmark")?,
            row["units"],
            number(row, "openjdk_median_ns_per_unit")? / 1000.0,
            number(row, "ristretto_median_ns_per_unit")? / 1000.0,
            number(row, "ratio")?
        )?;
    }
    let ratios = rows
        .iter()
        .filter(|r| r["benchmark"] != "control" && r["complete"] == true)
        .map(|r| number(r, "ratio"))
        .collect::<Result<Vec<_>>>()?;
    if !ratios.is_empty() {
        let mean = (ratios.iter().map(|r| r.ln()).sum::<f64>() / ratios.len() as f64).exp();
        writeln!(
            text,
            "\nEqual-weight geometric mean across {} complete workloads: **{mean:.2}×**. This describes this synthetic suite, not arbitrary Java applications.\n",
            ratios.len()
        )?;
    }
    if let Some(control) = rows.iter().find(|r| r["benchmark"] == "control") {
        writeln!(
            text,
            "Empty-kernel control: OpenJDK {:.1} µs; Ristretto {:.1} µs. Control percentages, fork ranges, variability and whole-process peak RSS are in `summary.csv`.\n",
            number(control, "openjdk_batch_median_ms")? * 1000.0,
            number(control, "ristretto_batch_median_ms")? * 1000.0
        )?;
    }
    text.push_str("## Process startup\n\nNew process to exit, with filesystem caches warmed by preflight. OpenJDK keeps default CDS; a separate `-Xshare:off` run measures sensitivity.\n\n| Program | Runtime | Runs | Median ms | Min–max ms |\n|---|---|---:|---:|---:|\n");
    for program in ["StartupEmpty", "StartupHello"] {
        for runtime in ["openjdk", "openjdk_no_cds", "ristretto"] {
            let values = records
                .iter()
                .filter(|r| {
                    r["phase"] == "startup"
                        && r["status"] == "ok"
                        && r["program"] == program
                        && r["runtime"] == runtime
                })
                .map(|r| Ok(number(r, "elapsed_ns")? / 1e6))
                .collect::<Result<Vec<_>>>()?;
            if !values.is_empty() {
                writeln!(
                    text,
                    "| {program} | {runtime} | {} | {:.2} | {:.2}–{:.2} |",
                    values.len(),
                    median(&values)?,
                    values.iter().copied().fold(f64::INFINITY, f64::min),
                    values.iter().copied().fold(0.0, f64::max)
                )?;
            }
        }
    }
    let failures = records
        .iter()
        .filter(|r| r["status"] == "error")
        .collect::<Vec<_>>();
    writeln!(
        text,
        "\n## Validation and limitations\n\nRecorded failures: {}. Measured checksums must match OpenJDK; preflight compares two input seeds.\n",
        failures.len()
    )?;
    text.push_str("Single-threaded kernels and uncontended monitors only. Library operations can use native intrinsics. Allocation includes naturally occurring GC; default heap policies differ. RSS covers the entire process, including warmup, and is not retained Java heap size.\n\nFresh-exception results can slow across samples and depend on run length. Inspect raw sample trends before interpreting any finite-run ratio as a stable rate.\n\nNo CPU affinity, clock lock, cache flush or OS isolation. Small arrays do not measure main-memory bandwidth. The external timer avoids Ristretto's current wall-clock-based `System.nanoTime()` implementation.\n\nRaw observations and commands: `raw.jsonl`; runtime properties, environment and hashes: `metadata.json`.\n");
    writeln!(
        text,
        "\nReport bootstrap generator: {RNG_NAME}. Reprocessing historical Python observations preserves medians, but bootstrap interval endpoints can differ because the generator changed."
    )?;
    for failure in failures {
        writeln!(
            text,
            "\n- {}/{}/{}: {}",
            failure["phase"],
            failure
                .get("benchmark")
                .or_else(|| failure.get("program"))
                .unwrap_or(&Value::Null),
            failure["runtime"],
            failure["error"]
        )?;
    }
    std::fs::write(output.join("report.md"), text)?;
    Ok(rows)
}

pub fn replay(input: &Path, output: &Path) -> Result<()> {
    let metadata: Value = serde_json::from_slice(&std::fs::read(input.join("metadata.json"))?)?;
    let records = std::fs::read_to_string(input.join("raw.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<Vec<Value>, _>>()?;
    let selected = WORKLOADS
        .iter()
        .copied()
        .filter(|w| records.iter().any(|r| r["benchmark"] == w.name))
        .collect::<Vec<_>>();
    ensure!(!selected.is_empty(), "No known workloads in input");
    empty_directory(output)?;
    write_report(output, &metadata, &records, &selected)?;
    println!("Report: {}", output.join("report.md").display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_escapes_fields() {
        assert_eq!(csv_field(&Value::from("a,\"b\"\n")), "\"a,\"\"b\"\"\n\"");
        assert_eq!(csv_field(&Value::Null), "");
    }
}

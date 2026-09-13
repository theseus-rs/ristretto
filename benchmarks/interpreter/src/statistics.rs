//! Deterministic randomization and aggregation of independent process medians.
#![expect(
    clippy::cast_precision_loss,
    reason = "Sample counts and elapsed nanoseconds fit the precision needed for descriptive statistics"
)]

use crate::workloads::Workload;
use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};

pub const RNG_NAME: &str = "SplitMix64 with rejection sampling and Fisher-Yates shuffle";

pub fn put(value: &mut Value, key: impl Into<String>, entry: Value) -> Result<()> {
    value
        .as_object_mut()
        .context("Expected JSON object")?
        .insert(key.into(), entry);
    Ok(())
}

#[derive(Debug)]
pub struct Random(u64);

impl Random {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.0;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }

    pub fn index(&mut self, length: usize) -> Result<usize> {
        let bound = u64::try_from(length)?;
        ensure!(bound > 0, "Cannot sample an empty collection");
        let ceiling = u64::MAX - u64::MAX % bound;
        loop {
            let value = self.next();
            if value < ceiling {
                return Ok(usize::try_from(value % bound)?);
            }
        }
    }

    pub fn shuffle<T>(&mut self, values: &mut [T]) -> Result<()> {
        for index in (1..values.len()).rev() {
            let other = self.index(index + 1)?;
            values.swap(index, other);
        }
        Ok(())
    }
}

pub fn number(value: &Value, key: &str) -> Result<f64> {
    let result = value
        .get(key)
        .and_then(Value::as_f64)
        .with_context(|| format!("Missing numeric field {key}"))?;
    ensure!(result.is_finite(), "Non-finite field {key}");
    Ok(result)
}

pub fn string<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    value
        .get(key)
        .and_then(Value::as_str)
        .with_context(|| format!("Missing string field {key}"))
}

pub fn median(values: &[f64]) -> Result<f64> {
    ensure!(
        !values.is_empty() && values.iter().all(|x| x.is_finite()),
        "Median requires finite, nonempty samples"
    );
    let mut ordered = values.to_vec();
    ordered.sort_by(f64::total_cmp);
    let middle = *ordered.get(ordered.len() / 2).context("Missing median")?;
    if ordered.len().is_multiple_of(2) {
        Ok((middle
            + ordered
                .get(ordered.len() / 2 - 1)
                .context("Missing lower median")?)
            / 2.0)
    } else {
        Ok(middle)
    }
}

pub fn sample_median(record: &Value) -> Result<f64> {
    let samples = record
        .get("samples")
        .and_then(Value::as_array)
        .context("Missing samples")?;
    median(
        &samples
            .iter()
            .map(|s| number(s, "elapsed_ns"))
            .collect::<Result<Vec<_>>>()?,
    )
}

pub fn bootstrap_ratio(left: &[f64], right: &[f64]) -> Result<(Option<f64>, Option<f64>)> {
    if left.len().min(right.len()) < 3 {
        return Ok((None, None));
    }
    ensure!(
        left.iter().chain(right).all(|x| x.is_finite() && *x > 0.0),
        "Ratios require positive finite samples"
    );
    let mut rng = Random::new(42);
    let mut ratios = Vec::with_capacity(10_000);
    for _ in 0..10_000 {
        let mut resample = |values: &[f64]| -> Result<f64> {
            let draws = (0..values.len())
                .map(|_| {
                    values
                        .get(rng.index(values.len())?)
                        .copied()
                        .context("Invalid random index")
                })
                .collect::<Result<Vec<_>>>()?;
            median(&draws)
        };
        ratios.push(resample(left)? / resample(right)?);
    }
    ratios.sort_by(f64::total_cmp);
    Ok((ratios.get(249).copied(), ratios.get(9749).copied()))
}

pub fn summarize(records: &[Value], selected: &[Workload], forks: usize) -> Result<Vec<Value>> {
    let mut rows = Vec::new();
    for workload in selected {
        let mut row = json!({"benchmark":workload.name,"category":workload.category,"work_per_unit":workload.work});
        let mut centers = Vec::new();
        let mut complete = true;
        let mut all_medians = Vec::new();
        let mut common_units = None;
        for runtime in ["openjdk", "ristretto"] {
            let group = records
                .iter()
                .filter(|r| {
                    r["phase"] == "measurement"
                        && r["status"] == "ok"
                        && r["benchmark"] == workload.name
                        && r["runtime"] == runtime
                })
                .collect::<Vec<_>>();
            if group.is_empty() {
                break;
            }
            let units = group
                .first()
                .and_then(|r| r["units"].as_u64())
                .context("Missing units")?;
            ensure!(
                units > 0 && group.iter().all(|r| r["units"].as_u64() == Some(units)),
                "Inconsistent batch sizes"
            );
            ensure!(
                common_units.is_none_or(|n| n == units),
                "Runtimes measured different unit counts"
            );
            common_units = Some(units);
            put(&mut row, "units", json!(units))?;
            let medians = group
                .iter()
                .map(|r| sample_median(r))
                .collect::<Result<Vec<_>>>()?;
            let center = median(&medians)?;
            ensure!(center > 0.0, "Zero elapsed time");
            centers.push(center);
            let mean = medians.iter().sum::<f64>() / medians.len() as f64;
            let cv = if medians.len() > 1 {
                Some(
                    (medians.iter().map(|n| (n - mean).powi(2)).sum::<f64>()
                        / (medians.len() - 1) as f64)
                        .sqrt()
                        / mean
                        * 100.0,
                )
            } else {
                None
            };
            let rss = group
                .iter()
                .map(|r| number(r, "peak_rss_bytes"))
                .collect::<Result<Vec<_>>>()?;
            for (key, value) in [
                ("forks", json!(group.len())),
                ("median_ns_per_unit", json!(center / units as f64)),
                ("batch_median_ms", json!(center / 1e6)),
                (
                    "fork_min_ms",
                    json!(medians.iter().copied().fold(f64::INFINITY, f64::min) / 1e6),
                ),
                (
                    "fork_max_ms",
                    json!(medians.iter().copied().fold(0.0, f64::max) / 1e6),
                ),
                ("fork_cv_pct", json!(cv)),
                ("median_peak_rss_mib", json!(median(&rss)? / 1_048_576.0)),
            ] {
                put(&mut row, format!("{runtime}_{key}"), value)?;
            }
            complete &= group.len() == forks;
            all_medians.push(medians);
        }
        if let [openjdk, ristretto] = centers.as_slice() {
            put(&mut row, "ratio", json!(ristretto / openjdk))?;
            if let [left, right] = all_medians.as_slice() {
                let (low, high) = bootstrap_ratio(right, left)?;
                put(&mut row, "ratio_ci_low", json!(low))?;
                put(&mut row, "ratio_ci_high", json!(high))?;
            }
            put(&mut row, "complete", json!(complete))?;
            rows.push(row);
        }
    }
    let control = rows.iter().find(|r| r["benchmark"] == "control").cloned();
    for row in &mut rows {
        for runtime in ["openjdk", "ristretto"] {
            let key = format!("{runtime}_batch_median_ms");
            let percentage = match &control {
                Some(control) => json!(100.0 * number(control, &key)? / number(row, &key)?),
                None => Value::Null,
            };
            put(row, format!("{runtime}_control_pct"), percentage)?;
        }
    }
    Ok(rows)
}

#[cfg(test)]
#[expect(
    clippy::panic_in_result_fn,
    reason = "Tests use assertions and propagate setup errors"
)]
mod tests {
    use super::*;

    #[test]
    fn process_medians_and_failures() -> Result<()> {
        let mut records = Vec::new();
        for (runtime, factor) in [("openjdk", 1), ("ristretto", 2)] {
            for values in [[10, 10, 1000], [20, 20, 20], [30, 30, 30]] {
                let samples = values
                    .into_iter()
                    .map(|n| json!({"elapsed_ns":n * factor}))
                    .collect::<Vec<_>>();
                records.push(json!({"phase":"measurement","status":"ok","benchmark":"example","runtime":runtime,"units":2,"peak_rss_bytes":1_048_576,"samples":samples}));
            }
        }
        records.push(json!({"phase":"measurement","status":"error","benchmark":"example","runtime":"ristretto","samples":[{"elapsed_ns":1}]}));
        let workloads = [Workload {
            id: 1,
            name: "example",
            category: "test",
            work: "two operations",
        }];
        let rows = summarize(&records, &workloads, 3)?;
        let row = rows.first().context("No summary")?;
        assert_eq!(row["ratio"], 2.0);
        assert_eq!(row["openjdk_median_ns_per_unit"], 10.0);
        assert_eq!(row["ristretto_forks"], 3);
        assert_eq!(bootstrap_ratio(&[20.0], &[10.0])?, (None, None));
        records.first_mut().context("No record")?["units"] = json!(3);
        assert!(summarize(&records, &workloads, 3).is_err());
        Ok(())
    }

    #[test]
    fn deterministic_shuffle_and_medians() -> Result<()> {
        let (mut a, mut b) = ([0, 1, 2, 3, 4], [0, 1, 2, 3, 4]);
        Random::new(42).shuffle(&mut a)?;
        Random::new(42).shuffle(&mut b)?;
        assert_eq!(a, b);
        a.sort_unstable();
        assert_eq!(a, [0, 1, 2, 3, 4]);
        assert!(median(&[]).is_err());
        assert!(median(&[f64::NAN]).is_err());
        assert!((median(&[4.0, 1.0, 2.0, 3.0])? - 2.5).abs() < f64::EPSILON);
        Ok(())
    }
}

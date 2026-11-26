//! Shared cyclic retained-ring fixture: checks correctness, never timing.
#![cfg(feature = "experimental-canonical-runtime")]
#![expect(clippy::panic_in_result_fn, reason = "integration assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm_research::canonical::{Limits, Program, Report};

fn program() -> Result<Program, Box<dyn std::error::Error>> {
    Ok(Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalRingWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalRingWorkload$Node.class"
        ))?,
    ])?)
}

fn configuration(method: &str) -> Result<(Configuration, usize), Box<dyn std::error::Error>> {
    let retained = match method {
        "small" => 8,
        "medium" => 32,
        "large" => 128,
        _ => return Err("unknown ring workload".into()),
    };
    Ok((
        Configuration {
            objects: retained * 4 + 8,
            roots: 128,
            mutators: 1,
            helpers: 1,
            edges_per_object: 3,
        },
        retained,
    ))
}

struct OracleCase {
    method: &'static str,
    steps: i32,
    expected: i32,
}

fn oracle() -> Result<Vec<OracleCase>, Box<dyn std::error::Error>> {
    include_str!("../../ristretto_vm/tests/fixtures/canonical/ring-oracle.tsv")
        .lines()
        .map(|line| {
            let mut columns = line.split('\t');
            let method = columns.next().ok_or("missing method")?;
            let steps = columns.next().ok_or("missing steps")?.parse()?;
            let expected = columns.next().ok_or("missing expected result")?.parse()?;
            if columns.next().is_some() {
                return Err("extra oracle column".into());
            }
            Ok(OracleCase {
                method,
                steps,
                expected,
            })
        })
        .collect()
}

fn check(
    report: &Report,
    steps: i32,
    retained: usize,
    expected: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let arrays = if steps < 0 {
        0
    } else {
        retained + usize::try_from(steps)?
    };
    assert_eq!(report.result, expected);
    assert_eq!(report.array_allocations, arrays);
    assert_eq!(report.allocations, arrays * 2);
    assert_eq!(report.released_objects, arrays * 2);
    assert_eq!(report.payload_bytes_after_cleanup, 0);
    assert_eq!(report.catalog_bytes_after_drop, 0);
    assert_eq!(report.array_writes, arrays * 2);
    if steps >= 0 {
        assert!(report.reference_reads > 0);
        assert!(report.reference_writes > 0);
    }
    Ok(())
}

#[tokio::test]
async fn interpreted_ring_matches_hotspot_and_reclaims_all_cycles()
-> Result<(), Box<dyn std::error::Error>> {
    let program = program()?;
    let cases = oracle()?;
    assert_eq!(cases.len(), 21);
    for OracleCase {
        method,
        steps,
        expected,
    } in cases
    {
        let (config, retained) = configuration(method)?;
        let report = program
            .run(
                "CanonicalRingWorkload",
                method,
                steps,
                config,
                Limits {
                    instructions: 4_000_000,
                    request_polls: 1_000_000,
                    cleanup_polls: 4_000_000,
                    ..Limits::default()
                },
            )
            .await?;
        println!("interpreted {method}({steps}) {report:?}");
        check(&report, steps, retained, expected)?;
    }
    Ok(())
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn compiled_ring_matches_hotspot_and_reclaims_all_cycles()
-> Result<(), Box<dyn std::error::Error>> {
    use ristretto_vm_research::canonical::CompiledLimits;
    let program = program()?;
    let compiled = program.compile()?;
    let cases = oracle()?;
    assert_eq!(cases.len(), 21);
    for OracleCase {
        method,
        steps,
        expected,
    } in cases
    {
        let (config, retained) = configuration(method)?;
        let report = compiled
            .run(
                "CanonicalRingWorkload",
                method,
                steps,
                config,
                CompiledLimits {
                    native_slices: 4_000_000,
                    request_polls: 1_000_000,
                    cleanup_polls: 4_000_000,
                    ..CompiledLimits::default()
                },
            )
            .await?;
        println!("compiled {method}({steps}) {report:?}");
        check(&report, steps, retained, expected)?;
        assert!(report.compiled_entries > 0);
    }
    Ok(())
}

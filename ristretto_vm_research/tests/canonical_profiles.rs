//! Method-level generated-code evidence, distinct from performance qualification.
#![cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#![expect(clippy::panic_in_result_fn, reason = "integration assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm_research::canonical::{CompiledLimits, CompiledMethodProfile, Error, Program};
use std::future::Future;
use std::task::{Context, Waker};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn graph() -> Result<Program> {
    Ok(Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalGraphWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalGraphWorkload$Node.class"
        ))?,
    ])?)
}

fn configuration() -> Configuration {
    Configuration {
        objects: 4,
        roots: 12,
        mutators: 1,
        helpers: 1,
        edges_per_object: 2,
    }
}

fn identity<'a>(row: &CompiledMethodProfile<'a>) -> (&'a str, &'a str, &'a str) {
    (row.class, row.name, row.descriptor)
}

#[tokio::test]
async fn linked_method_census_keeps_overloads_and_declaring_classes_distinct() -> Result {
    let program = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalLinkedWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalLinkedWorkload$Node.class"
        ))?,
    ])?;
    let compiled = program.compile()?;
    let before = compiled.method_profiles()?;
    assert_eq!(
        before.iter().map(identity).collect::<Vec<_>>(),
        [
            ("CanonicalLinkedWorkload", "<init>", "()V"),
            ("CanonicalLinkedWorkload", "helper", "(I)I"),
            ("CanonicalLinkedWorkload", "helper", "(J)I"),
            ("CanonicalLinkedWorkload", "run", "(I)I"),
            ("CanonicalLinkedWorkload$Node", "<init>", "(I)V"),
            ("CanonicalLinkedWorkload$Node", "<init>", "(J)V"),
            ("CanonicalLinkedWorkload$Node", "helper", "(I)I"),
        ]
    );
    assert!(before.iter().all(|row| row.entries == 0));
    let report = compiled
        .run(
            "CanonicalLinkedWorkload",
            "run",
            3,
            configuration(),
            CompiledLimits::default(),
        )
        .await?;
    assert_eq!(report.result, 91);
    assert_eq!(report.compiled_entries, 6);
    assert_eq!(report.calls, 6);
    let after = compiled.method_profiles()?;
    for (before, after) in before.iter().zip(&after) {
        assert_eq!(identity(before), identity(after));
        assert_eq!(
            after.entries,
            u64::from(identity(after) != ("CanonicalLinkedWorkload", "<init>", "()V"))
        );
    }
    assert_eq!(report.payload_bytes_after_cleanup, 0);
    assert_eq!(report.catalog_bytes_after_drop, 0);
    Ok(())
}

#[tokio::test]
async fn complete_method_census_and_deltas_match_actual_graph_execution() -> Result {
    let program = graph()?;
    let compiled = program.compile()?;
    let initial = compiled.method_profiles()?;
    assert_eq!(
        initial.iter().map(identity).collect::<Vec<_>>(),
        [
            ("CanonicalGraphWorkload", "<init>", "()V"),
            (
                "CanonicalGraphWorkload",
                "checksum",
                "(LCanonicalGraphWorkload$Node;LCanonicalGraphWorkload$Node;)I"
            ),
            ("CanonicalGraphWorkload", "run", "(I)I"),
            ("CanonicalGraphWorkload$Node", "<init>", "(I)V"),
        ]
    );
    assert!(
        initial
            .iter()
            .all(|row| row.entries == 0 && row.resumes == 0 && row.backedges == 0)
    );
    for rounds in [0, 1, 8] {
        let before = compiled.method_profiles()?;
        let report = compiled
            .run(
                "CanonicalGraphWorkload",
                "run",
                rounds,
                configuration(),
                CompiledLimits::default(),
            )
            .await?;
        let after = compiled.method_profiles()?;
        assert_eq!(report.result, rounds * rounds);
        let rounds = u64::try_from(rounds)?;
        let mut totals = (0_u64, 0_u64, 0_u64);
        for (before, after) in before.iter().zip(&after) {
            assert_eq!(identity(before), identity(after));
            let entries = after
                .entries
                .checked_sub(before.entries)
                .ok_or("entries regressed")?;
            let resumes = after
                .resumes
                .checked_sub(before.resumes)
                .ok_or("resumes regressed")?;
            let backedges = after
                .backedges
                .checked_sub(before.backedges)
                .ok_or("backedges regressed")?;
            let (expected_entries, expected_backedges) = match (after.class, after.name) {
                ("CanonicalGraphWorkload", "run") => (1, rounds),
                ("CanonicalGraphWorkload", "checksum") => (rounds, 0),
                ("CanonicalGraphWorkload$Node", "<init>") => (2 * rounds, 0),
                ("CanonicalGraphWorkload", "<init>") => (0, 0),
                _ => return Err("unexpected compiled method".into()),
            };
            assert_eq!((entries, backedges), (expected_entries, expected_backedges));
            totals.0 += entries;
            totals.1 += resumes;
            totals.2 += backedges;
        }
        assert_eq!(totals.0, u64::try_from(report.compiled_entries)?);
        assert_eq!(totals.1, u64::try_from(report.compiled_resumes)?);
        assert_eq!(totals.2, report.compiled_backedges);
        assert_eq!(report.payload_bytes_after_cleanup, 0);
        assert_eq!(report.catalog_bytes_after_drop, 0);
    }
    Ok(())
}

#[tokio::test]
async fn active_invocation_rejects_snapshot_and_cancellation_restores_access() -> Result {
    let program = graph()?;
    let compiled = program.compile()?;
    let independent = program.compile()?;
    let mut future = Box::pin(compiled.run(
        "CanonicalGraphWorkload",
        "run",
        100,
        configuration(),
        CompiledLimits::default(),
    ));
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    assert!(matches!(
        compiled.method_profiles(),
        Err(Error::Boundary("method profile during active invocation"))
    ));
    assert!(
        independent
            .method_profiles()?
            .iter()
            .all(|row| row.entries == 0)
    );
    drop(future);
    assert!(
        compiled
            .method_profiles()?
            .iter()
            .any(|row| row.entries > 0)
    );
    let report = compiled
        .run(
            "CanonicalGraphWorkload",
            "run",
            3,
            configuration(),
            CompiledLimits::default(),
        )
        .await?;
    assert_eq!(report.result, 9);
    assert!(compiled.method_profiles().is_ok());
    assert!(
        independent
            .method_profiles()?
            .iter()
            .all(|row| row.entries == 0)
    );
    Ok(())
}

#[tokio::test]
async fn ring_profile_identifies_hot_and_unexecuted_methods() -> Result {
    let program = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalRingWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalRingWorkload$Node.class"
        ))?,
    ])?;
    let compiled = program.compile()?;
    let report = compiled
        .run(
            "CanonicalRingWorkload",
            "small",
            17,
            Configuration {
                objects: 40,
                roots: 128,
                mutators: 1,
                helpers: 1,
                edges_per_object: 3,
            },
            CompiledLimits::default(),
        )
        .await?;
    assert_eq!(report.result, 652_228_386);
    let profiles = compiled.method_profiles()?;
    assert_eq!(profiles.len(), 7);
    for row in profiles {
        let (entries, backedges) = match (row.class, row.name) {
            ("CanonicalRingWorkload", "small") => (1, 0),
            ("CanonicalRingWorkload", "execute") => (1, 32),
            ("CanonicalRingWorkload", "inspect") | ("CanonicalRingWorkload$Node", "<init>") => {
                (25, 0)
            }
            ("CanonicalRingWorkload", "medium" | "large" | "<init>") => (0, 0),
            _ => return Err("unexpected ring method".into()),
        };
        assert_eq!(
            (row.entries, row.backedges),
            (entries, backedges),
            "{row:?}"
        );
    }
    assert_eq!(report.compiled_entries, 52);
    assert_eq!(report.compiled_backedges, 32);
    Ok(())
}

#[tokio::test]
async fn shared_native_profile_boundary_tracks_each_active_invocation() -> Result {
    let program = graph()?;
    let compiled = program.compile()?;
    let config = Configuration {
        objects: 8,
        roots: 24,
        mutators: 2,
        helpers: 2,
        edges_per_object: 2,
    };
    program
        .with_heap(config, 1024 * 1024, 1_000_000, async |heap| {
            let mut first = compiled.attach(heap, CompiledLimits::default())?;
            let mut second = compiled.attach(heap, CompiledLimits::default())?;
            assert!(compiled.method_profiles().is_ok());
            let mut a = Box::pin(first.invoke("CanonicalGraphWorkload", "run", 100));
            let mut b = Box::pin(second.invoke("CanonicalGraphWorkload", "run", 100));
            let mut context = Context::from_waker(Waker::noop());
            assert!(a.as_mut().poll(&mut context).is_pending());
            assert!(b.as_mut().poll(&mut context).is_pending());
            assert!(matches!(
                compiled.method_profiles(),
                Err(Error::Boundary("method profile during active invocation"))
            ));
            drop(a);
            assert!(matches!(
                compiled.method_profiles(),
                Err(Error::Boundary("method profile during active invocation"))
            ));
            drop(b);
            assert!(compiled.method_profiles().is_ok());
            assert_eq!(
                first
                    .invoke("CanonicalGraphWorkload", "run", 3)
                    .await?
                    .result,
                9
            );
            assert_eq!(
                second
                    .invoke("CanonicalGraphWorkload", "run", 4)
                    .await?
                    .result,
                16
            );
            Ok(())
        })
        .await?;
    Ok(())
}

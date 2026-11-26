//! Untimed, explicit diagnostic on the checked persistent Java ring.
use super::*;

#[tokio::test]
#[ignore = "explicit finite mechanism diagnostic; no timing or throughput claim"]
async fn persistent_ring_publication_census() -> Result<(), Error> {
    let program = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../../ristretto_vm/tests/fixtures/canonical/CanonicalRingWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../../ristretto_vm/tests/fixtures/canonical/CanonicalRingWorkload$Node.class"
        ))?,
    ])?;
    let compiled = program.compile()?;
    for (method, retained, expected) in [
        ("small", 8, -539_718_046),
        ("medium", 32, -855_157_214),
        ("large", 128, -678_999_774),
    ] {
        let config = Configuration {
            objects: retained * 4 + 8,
            roots: 128,
            mutators: 1,
            helpers: 1,
            edges_per_object: 3,
        };
        let completion = program.with_heap(config, 64 * 1024 * 1024, 1_000_000, async |heap| {
            let mut thread = compiled.attach(heap, CompiledLimits::default())?;
            let warmup = thread.invoke("CanonicalRingWorkload", method, 257).await?;
            assert_eq!(warmup.result, expected);
            let mut rows = Vec::new();
            for sample in 0..3 {
                let before = thread.publication_census()?;
                let profiles_before = compiled.method_profiles()?;
                let invocation = thread.invoke("CanonicalRingWorkload", method, 257).await?;
                let after = thread.publication_census()?;
                let profiles_after = compiled.method_profiles()?;
                assert_eq!(invocation.result, expected);
                assert_eq!(invocation.allocations, (retained + 257) * 2);
                let mut entries = 0;
                let mut backedges = 0;
                let mut resumes = 0;
                assert_eq!(profiles_before.len(), profiles_after.len());
                for (before, after) in profiles_before.iter().zip(&profiles_after) {
                    assert_eq!((before.class, before.name, before.descriptor), (after.class, after.name, after.descriptor));
                    entries += after.entries - before.entries;
                    resumes += after.resumes - before.resumes;
                    backedges += after.backedges - before.backedges;
                }
                assert!(entries > 0 && backedges > 0);
                rows.push(serde_json::json!({
                    "method": method, "argument": 257, "sample": sample,
                    "result": invocation.result, "allocations": invocation.allocations,
                    "helper_polls": invocation.helper_polls, "native_slices": invocation.native_slices,
                    "released_objects": invocation.released_objects,
                    "entries": entries, "resumes": resumes, "backedges": backedges,
                    "publications": after.calls - before.calls,
                    "unchanged_snapshots": after.unchanged - before.unchanged,
                    "nonnull_inputs": after.nonnull_inputs - before.nonnull_inputs,
                    "owners_rebound": after.rebound - before.rebound,
                    "owner_entry_probes": after.owner_probes - before.owner_probes,
                        "snapshot_cells_checked": after.snapshot_cells_checked - before.snapshot_cells_checked,
                }));
            }
            Ok(rows)
        }).await?;
        // Successful with_heap completion checks zero payload and refunded catalog charge.
        for row in completion.value {
            println!("CANONICAL_PUBLICATION_CENSUS {row}");
        }
        println!(
            "CANONICAL_PUBLICATION_CLEANUP {}",
            serde_json::json!({"method":method,"cleanup_polls":completion.cleanup_polls,"released_during_cleanup":completion.released_during_cleanup})
        );
    }
    Ok(())
}

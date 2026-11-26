# Co-locating ordinary small payloads with descriptors

The ordinary host allocator already shares separate slabs between payloads
or between descriptors of the same class. We tested three single-backing
layouts for payloads of up to 64 bytes. An atomic bit per slot lets a payload
be reused while a stale descriptor is retained. Every variant passed 578
native GC unit tests, 332 WASI tests, strict native GC Clippy and the focused
Miri all-headers-retained test. The ordinary allocator remains the default.

| Layout | Mutator allocation time ratio | Ephemeral host allocation count ratio | Ephemeral allocated bytes ratio |
|---|---:|---:|---:|
| [Interleaved rows](rows.csv) | 1.245 [1.219, 1.265] | 1.739 | 3.173 |
| [Separate columns, 32 rows](columns.csv) | 0.959 [0.929, 0.984] | 1.100 | 1.111 |
| [Separate columns, up to 256 rows](dense.csv) | 1.005 [0.987, 1.026] | 0.990 | 1.010 |

Ratios are candidate divided by the ordinary allocator. The first column times
the mutator section of the Rust `mechanisms allocation` fixture. Brackets
are a paired process-bootstrap 95% interval. Each layout used 24 shuffled
process pairs per fixture, one native CPU, baseline publication policy and
disabled diagnostics. Each timing process performed 20,000 operations and
each allocator-counting process performed 3,000. All 1,008 processes checked
their checksum and zero live objects at teardown. The CSVs also report leaf,
owning and managed-cell allocation, wall time through collection, batch
construction, weak retention, and host byte counts.

## Physical reclamation

A separate probe allocated 2,048 ordinary `u64` objects, retained one metadata
descriptor every 256 allocations, dropped every root and completed three
collections. Both allocators reported zero charged managed bytes. The eight
stale descriptors retained **126,976 bytes** of slab backing with separate
slabs and **524,288 bytes** with the larger combined slab. Both started with
524,288 bytes and released all backing after the descriptors dropped. Every
one of 12 fresh process pairs produced the same counts. These are physical
slab budget credits. The probe does not measure whole-process RSS.
[Sparse retention trials](sparse.json)

## Reproduction

Use `60e3b9f9` as a checkout with the parked-descriptor test. Its allocator
matches the timed parent `2e9795b8`. In separate checkouts, apply one
decompressed [row](rows.patch.gz), [column](columns.patch.gz) or
[dense column](dense.patch.gz) patch with `git apply`. Build each checkout
with its own empty Cargo target using `cargo build --release --locked -p
ristretto_gc --example mechanisms --example allocation_counts`.
Run the respective release binaries pinned to the same CPU with
`mechanisms allocation baseline 20000 false` or
`allocation_counts baseline 3000 ephemeral`. The raw records identify the
remaining fixtures and repeat numbers. Pair each before and after process by
fixture and repeat before computing an interval.

Apply the [sparse probe](sparse-probe.patch.gz) to both the parent and dense
checkouts containing the parked-descriptor test. Run `cargo test --locked -p
ristretto_gc --lib sampled_stale_headers_report_physical_backing_after_payload_reclamation
-- --nocapture` in each. The three counts are charged slab bytes with all
objects live, with only stale descriptors, and after releasing those descriptors.

[Binary hashes](binaries.json) · [Row processes](rows.jsonl.gz) · [Column processes](columns.jsonl.gz) · [Dense column processes](dense.jsonl.gz)

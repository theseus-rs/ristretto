# A suspended context can stall full collection

On `c02357ca`, an explicit context flush claimed one clean edge and suspended before publishing its mark work. With zero background workers, a different thread requested a fresh full collection and attempted an eight-byte managed allocation. Across 2,048 bounded polls and collector steps, neither completed under either publication policy. The object shard was available throughout. A separate unreachable allocation provided the funding.

| Publication policy | Pending edge records | Fresh full ticket completed | Capped allocation completed |
|---|---:|---|---|
| Legacy | 1 | No | No |
| Counted | 1 | No | No |

After the flusher resumed, the test checked the original weak identity, collected the now-unreachable graph and verified that the managed charge returned to zero. The registered traced-edge case now completes with the flusher suspended on both publication policies. The alpha still needs progress evidence for dirty owners, raw handles and other phases. [Corrected collector, forced schedules and release measurements](qualification/README.md). [Regression patch](control.patch.gz) · [Observed assertion and toolchain output](control.log.gz).

Run from a clean checkout of the measured revision. The final assertion is expected to fail on that revision.

```sh
gzip -dc /path/to/control.patch.gz | git apply -
cargo test --locked -p ristretto_gc --lib fresh_full_and_capped_allocation_finish_while_clean_context_flusher_is_suspended
```

## A parked dirty-owner flush still blocks peers

At `9d9edb63` a write through the managed guard appended a child to an existing
vector. The probe drained the clean edge records, verified that the remaining
dirty record named the exact parent, then parked its explicit flusher after
claim. Under both publication policies the single record remained outstanding.
Neither the fresh full ticket nor the independently funded allocation finished
in 2,048 polls and steps. Once the flusher resumed, both exact child witnesses
survived, then cleared when the parent died. All 100 cases across 50 fresh
native processes had the same outcome.
[Single-collector reproducer](dirty-owner/control.patch.gz) ·
[Assertion](dirty-owner/control.log.gz) · [Process results](dirty-owner/processes.json.gz).

An experimental change published remembered membership immediately during an
old/full-only epoch. It passed the new parked-owner probe. Five of 653 native
library tests failed, including a young-start schedule and a busy-index
reclamation schedule. Several failed while establishing their expected pending
record. Those failures leave the affected schedules untested for this
alternative. The change is withheld. Any replacement must preserve exact owner
identity and mutation version across a concurrent young start and prove
progress when the remembered shard is busy.
[Experimental patch](dirty-owner/direct-enrollment.patch.gz) ·
[Parked-owner result](dirty-owner/direct-enrollment-probe.log.gz) ·
[Full test outcomes](dirty-owner/direct-enrollment-suite.log.gz).

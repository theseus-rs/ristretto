# Parked native callback claims

The shipping revision `5ba3b207` shares a receiver mutex across native callback claimants. The candidate `5fc3a4d6` publishes sweep batches with independently claimable slots and groups retirements on registered native workers. Cooperative sweeping keeps its existing work budget. Both the implementation and the parent scheduling hooks can be applied to `5ba3b207` with the patches below.

| Forced native schedule | Shipping collector with test hooks | Candidate |
|---|---:|---:|
| Claimant parked immediately before taking a job | 0 of 128 peer callbacks reclaimed | 128 of 128 |
| Claimant parked immediately after taking one job | 0 of 128 | 127 of 128 |
| Heap-cap allocation, before claim | 0 of 100 funded | 100 of 100 |
| Heap-cap allocation, after claim | 0 of 100 funded | 100 of 100 |

Each row used 100 fresh parent/candidate pairs. An older arbitrary destructor stayed parked in all four schedules. After resumption the fixture required one destructor invocation per peer and zero managed bytes. The integrated candidate passed 969 native tests with one ignored, strict Clippy, and four separate full unit-suite runs on CPUs 12–23.

**A pressure-service schedule still fails the release requirement.** On the worker-batching development revision `8db9a39d`, both heap-cap tests failed in one parallel suite run. A separate candidate probe stopped the dedicated pressure worker immediately after receiving the request and before its first claim. The real allocation returned out-of-memory with all 128 peer callbacks still charged. Once allowed to help, a peer drained all 128. This repeated in 100 fresh processes. The probe establishes one way to exhaust the allocation budget. Its cause in the earlier parallel failures was not instrumented.

The complete-collection candidate exceeded 10× in all 36 native/WASI conditions against `f1daebeb`. Minimum individual 95% lower speedup bounds were 10.84× native and 10.28× WASI. Against the immediate shipping parent, 1,024 cheap finalizers took 14–15% longer, and 64 finalizers with two workers took 26% longer. The native queue stays outside the shipping collector while pressure-service progress and cheap-callback cost remain open.

The next validation must cover a demand already visible to a runnable pressure worker, an assistant running an arbitrary parked destructor, shutdown, cancellation, all storage still charged, and an independent physical refund becoming available immediately before terminal allocation denial. An async allocation poll must remain bounded even when the user destructor blocks.

Reproduce with `gzip -dc candidate.patch.gz | git apply` in a clean checkout of `5ba3b207`. Use the [parent hook patch](control.patch.gz) on a separate checkout of the same base, and apply the [pressure-service probe](pressure-cut.patch.gz) on the candidate. Run `cargo test --locked -p ristretto_gc --lib parked_callback_claim`, `cargo test --locked -p ristretto_gc --lib independent_callbacks_remain_available`, and the candidate-only `terminal_allocation_can_close_before_the_runnable_pressure_assistant_claims` test.

[Candidate patch](candidate.patch.gz), [paired claim samples](claim-processes.jsonl.gz), [pressure-service runs](pressure-service-processes.jsonl.gz), [callback fixture](callback_cost.rs), [callback costs](callback-cost.csv), [callback samples](callback-processes.jsonl.gz), [build identities](claim-provenance.json), [all complete-collection conditions](cycles/complete-cycles.csv).

## Follow-up with an acknowledged dispatch

Revision `3ebce7d6` added a pressure-worker acknowledgment to the independently claimable queue. The delayed-worker regression passed, as did all 400 forced before/after-claim schedules. With 64 cheap finalizers and two workers, median full-cycle time was 1.320× the ordinary parent [1.288, 1.368]. With 1,024 finalizers it was 1.123× with one worker and 1.143× with two. The shipping collector takes the acknowledgment independently. The per-slot queue remains experimental pending a cheaper way to let peers claim work around a parked participant.

Apply the [incremental patch](acknowledged-queue.patch.gz) after the original candidate patch above. [Forced claim runs](acknowledged-claims.jsonl.gz), [callback intervals](acknowledged-cost.csv), [raw timings](acknowledged-cost.jsonl.gz), [shipping implementation and controls](../pressure-service-turn/README.md).

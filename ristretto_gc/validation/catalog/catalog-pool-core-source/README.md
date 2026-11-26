# Fixed backing for immutable owning values

This experimental pool is used by the adjacent numeric catalog prototype, not by
the garbage collector. Initialization allocates a fixed array of slots. Subsequent
slot reservation does not call the system allocator. `try_allocate` tries one
cursor-selected slot and returns the original value on failure; failure does not
mean every slot is occupied.

Each occupied slot retains one aggregate strong owner of its pool. Cloning a slot
owner changes only that slot's reference count. The final decrement acquires prior
owners' releases, destroys the value, then publishes the slot as free and releases
the aggregate pool owner. A separate occupied bit prevents reuse between the final
decrement and completed destruction. An unwind guard preserves this ordering when
destruction panics. There is no weak slot-owner API.

`Owned<T>` supplies ArcSwap's owning-pointer contract. An expected owning reference
keeps the exact slot value alive, preventing address reuse while a delayed CAS can
still compare it. Raw conversions preserve the real slot pointer and transfer one
reference count. Send/Sync require `T: Send + Sync`.

Six native fixtures, five Miri fixtures (including concurrent owning loads), five
Tree Borrows fixtures, three WASI fixtures, and ASan/TSan checks pass. Four models
import the exact `slot_state.rs` atomics. Publishing reuse early or omitting the
final acquire causes the corresponding model to fail. Pointer provenance and the
aggregate pool owner are outside those atomic models; native/Miri checks cover
their exercised schedules. The corrected-checker runner is:

```sh
python3 benchmarks/gc/catalog_model_check.py --output target/catalog-model-check
```

The checker patch and its limitations are recorded in the parent README and each
run's `result.json`. Stock Loom results are also retained.

This is not a heap-budget implementation. Pool initialization, teardown, arbitrary
`T` destructors and ArcSwap's internal work are not bounded by slot reservation.
A stopped destructor leaves its slot unavailable. A production reserve policy must
bound retained participant state and preserve independent progress capacity before
admitting participants; a finite pool alone does not establish that guarantee.

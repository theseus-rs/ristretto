# Restricted Java heaps and contexts

The opt-in `Program::with_heap` API runs multiple entries on one private collector
and canonical catalog. `JavaHeap::attach` reserves one mutator and one helper for
an interpreter context. Registration failure releases any partially claimed
participant. Dropping a context returns its participant quotas.

`JavaThread::invoke` resets call-local limits and counters. Frame roots remain
owned across every cooperative suspension; dropping an invocation future releases
those roots, and the same context can run again. Other attached contexts may run
and reclaim unrelated objects while one invocation is parked. Native contexts can
be created on separate OS threads, each with its own local executor. The
interpreter future itself is local and does not establish migration of a suspended
future between threads. WASI exercises independent cooperative contexts.

The collector's payload byte cap and the catalog's checked reservation are
separate limits. A successful outer return requires every context to detach,
managed bytes to reach zero within the cleanup poll cap, and all catalog credit
to be refunded. Error and cancellation paths still drop the catalog and its owned
payloads. A released-object count describes physical slots and can include work
from another invocation; it is neither an invocation allocation count nor a GC
cycle count. `primary_cycles` reports the collector's conservative completed
primary-epoch bounds only.

This slice preserves `Program::run`, `run_scalar`, and `run_primitive`. It does not
admit additional Java bytecodes, layouts, static storage, inheritance, handlers,
or ordinary-VM interoperability. It changes no default backend and makes no
latency, throughput, endurance, or pause qualification claim.

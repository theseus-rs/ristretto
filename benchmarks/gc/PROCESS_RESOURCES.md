# Child lifetime resource receipts

New ring runs use schema-2 `run.json` and a separately hashed
`process-resources.json`. The run binds the exact child PID, exit status,
unique run ID, requested watchdog, frozen launch plan, controller and observer source. Schema-1 evidence remains readable, with no
retrospective RSS claim. Missing, altered or mismatched schema-2 receipts fail
validation; a watchdog expiration fails even if exit raced the kill signal. The
checker compares the duration with the hashed launch plan, and matrix execution
and plotting additionally compare it with the frozen matrix watchdog. Rebinding
a stale receipt hash does not repair a mismatched run identity or duration.

On Linux, the parent opens a pidfd for its exact child, blocks on descriptor
readiness until exit or the watchdog deadline, and reaps that PID once through
`wait4`. The watchdog signals the pidfd, never a potentially reused numeric PID.
There is no periodic parent polling or RSS sampling thread. The Popen object is
given the reaped return code to prevent a second wait. Platforms without these
APIs retain normal watchdog handling and report metrics as unavailable, never
zero. Control/reaping failures produce no valid observation.

The receipt retains raw `ru_maxrss` in KiB and bytes, CPU times, page faults,
context switches and the raw wait status. These values cover the child's complete
lifetime, including the Python launcher, exec, startup, admission, compilation,
warmup, execution and teardown. For WASI they describe the Wasmtime host process,
not a guest PID or guest-only allocation. [Python's process APIs](https://docs.python.org/3/library/os.html#os.wait4)
and the [Linux resource-usage contract](https://man7.org/linux/man-pages/man2/getrusage.2.html)
define the PID-specific result and RSS units.

Linux `wait4` also includes accounting for descendants the child reaped. Its RSS
is the maximum of the process and those descendants, **not their simultaneous
total**. The receipt therefore explicitly uses
`linux_wait4_child_lifetime_including_reaped_descendants`; it does not assert
that descendants were excluded. This follows the kernel's
[wait accounting](https://github.com/torvalds/linux/blob/master/kernel/exit.c)
and [RUSAGE_BOTH implementation](https://github.com/torvalds/linux/blob/master/kernel/sys.c).
The closed ring programs do not request subprocesses, but that source property
does not justify silently relabeling the generic observer as process-tree RSS.

These are observations, not RSS enforcement or measurement-window peaks. Cgroup
`memory.peak` and `memory.events` remain separate. A future RSS gate must declare
its lifetime/process-tree scope and ceiling, require available observations, and
assess observer effects. The already frozen baseline and optimized comparison
retain their original controller and provide no RSS measurement.

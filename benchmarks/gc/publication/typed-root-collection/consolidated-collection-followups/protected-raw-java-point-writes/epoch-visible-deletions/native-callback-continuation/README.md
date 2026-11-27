# Foreground access to a native callback continuation

A native callback returned `Continue(7)` on a real swept object. Its primary thread stopped immediately after publishing the continuation. With no marking workers or allocation pressure, the test allowed a foreground caller 2,048 calls to public `gc.step()` to release the object's charge. The shipping heap queue allowed the refund in 24/24 fresh processes. A dedicated callback lane failed in 24/24. Resuming the primary finished cleanup in both arms. The two executable hashes differ, and the test compiled from a separate Cargo target directory for each arm.
[Control patch](control.patch.gz) · [Experimental patch](lane.patch.gz) · [Control output](control.log.gz) · [Failure](lane.log.gz) · [48 executions](schedules.json.gz)

The candidate let the primary publish directly to a lane visible to callback helpers and the pressure assistant. An ordinary foreground step could not reach that lane while the primary was stopped. The read-only audit identified the same failure. The experiment was withheld before throughput qualification. A faster handoff must preserve public access through foreground steps, including when there is no allocation pressure.
[Auditor response](audit.json)

Commit `93b97724` integrates the regression on the shipping path and checks the callback offset, exactly one finalizer, exactly one drop, physical refund and both outstanding-work counters. All 658 native GC library tests and strict Clippy passed. The verified Cargo archive contained the unified collector.
[Native tests](native-tests.log.gz) · [Clippy](native-clippy.log.gz) · [Cargo verification](package.log.gz) · [Packaged source identity](package-identity.json)

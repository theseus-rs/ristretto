# Marking progress on the shipping collector

On shipping revision `4ae63553`, a native collector was suspended immediately after marking a promoted child and before placing its traversal in the private claim. The child contained one managed edge to a grandchild. A separate detached graph controlled for accidental retention. The test enabled independent marking and old-only collections, capped the heap at 64 KiB and exercised gated and counted publication.

| Peer while the executor was parked | Fresh native processes | Outcomes across both policies |
| --- | ---: | --- |
| Old-cycle steps only | 8 | All 16 retained the child’s mark while its grandchild was still white |
| Fresh full collection | 8 | All 16 tickets exceeded 2,048 peer turns |
| Funded allocation | 8 | All 16 requests exceeded 2,048 peer turns |

Every parked observation found the coordinator and domain cycle locks available, the marking writer excluded and zero in-flight traversal reservations. After the executor resumed, each requested full ticket completed and each allocation returned the expected value. The old-only arm retained the detached treatment graph for its required epoch, reclaimed the disjoint control and reclaimed the treatment after a subsequent full collection. The ordinary uninstrumented schedule also passed.

The probe modifies only native test code and keeps the expected stall out of the shipping regression suite. The next progress gate requires peers to discover the traversal for the exact allocation and epoch through the interval between its physical mark and work publication. These results concern the optional old-only independent marking path.

[Patch against the shipping tree](probe.patch.gz) · [Build identity and per-process outcomes](schedules.json.gz)

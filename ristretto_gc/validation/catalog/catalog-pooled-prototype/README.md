# Catalog with preallocated operation and membership states

The publication protocol is the same numeric experiment described in the parent
README. Control versions and membership values now use the adjacent fixed pool.
A new cell still uses one system allocation. Each private preparation makes at most
eight slot attempts per pooled value and returns failure without changing canonical
membership when it cannot prepare the complete operation.

The experimental default reserves 256 control slots and rounds the catalog capacity plus 256 membership slots to the next power of two. These are design-screen settings,
not an enforced participant limit or production progress reserve. Deliberately
undersized configurations fail closed in the tests. They are not qualified to make
progress with stalled participants.

Thirteen native fixtures and eight protocol models pass. Native stopped-helper
fixtures complete 999 subsequent descriptor lifecycles at each of four stop points.
The retained old operation leaves five control slots and three membership slots
occupied, including the current state; those counts fall to one and zero after
its owners disappear. Preparation failure and cancellation return private slots.
Ten fixtures pass on actual WASI, Miri and Tree Borrows; ASan/TSan pass all 13 native
fixtures. Models abstract owning links and allocation; the pool has separate
atomic-state models and pointer checks.

The pooled version lowers the synthetic lifecycle from 18 system allocation
requests to one (56 native / 32 WASI bytes). At 16,384 resident descriptors it retains
2,644,200 native / 1,917,044 WASI requested bytes after setup, versus 1,704,168 /
1,114,244 for the unpooled descriptor version. These are requested bytes, not RSS.
The 90-process screen reproduces all earlier tree/unpooled counts exactly. See
the parent README for the figure, raw evidence and reproduction commands.

Actual metadata, initial roots, payload leases, byte credits, reference floors and
every existing admission caller remain unintegrated. Counter exhaustion refuses
to wrap; epoch rebasing is not implemented. The collector and defaults are unchanged.

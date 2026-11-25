use super::*;

#[test]
fn a_parked_tree_read_view_allows_unrelated_payload_reclamation() {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        );
        let live = Gc::try_new(&gc, 11_u64).unwrap();
        let live_bytes = gc.managed_bytes();
        drop(Gc::try_new(&gc, 22_u64).unwrap());
        assert!(gc.managed_bytes() > live_bytes);
        let view = gc.roots.read_view();
        gc.collect();
        for _ in 0..4096 {
            gc.step();
            if gc.managed_bytes() == live_bytes {
                break;
            }
        }
        let retained = gc.managed_bytes();
        drop(view);
        assert_eq!(
            retained, live_bytes,
            "parked root view prevented unrelated reclamation"
        );
        assert_eq!(**live, 11);
        drop(live);
        gc.collect_full().unwrap();
        gc.stop().unwrap();
        assert_eq!(gc.managed_bytes(), 0);
    }
}

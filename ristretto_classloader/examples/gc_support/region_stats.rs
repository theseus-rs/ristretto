//! Physical backing and relocation observations shared by the workload drivers.
use ristretto_gc::region::RegionStatistics;

pub fn snapshot(value: RegionStatistics) -> serde_json::Value {
    serde_json::json!({
        "committed_bytes": value.committed_bytes,
        "occupied_bytes": value.occupied_bytes,
        "regions": value.regions,
        "registry_shards": value.registry_shards,
        "bytes_copied": value.bytes_copied,
        "bytes_staged": value.bytes_staged,
        "bytes_recopied": value.bytes_recopied,
        "destination_failures": value.destination_failures,
        "relocations": value.relocations,
        "regions_released": value.regions_released,
        "shared_committed_bytes": value.shared_committed_bytes,
        "peak_shared_committed_bytes": value.peak_shared_committed_bytes,
        "relocation_reserve_bytes": value.relocation_reserve_bytes,
        "relocation_reserve_used_bytes": value.relocation_reserve_used_bytes,
        "peak_relocation_reserve_used_bytes": value.peak_relocation_reserve_used_bytes,
        "regions_selected": value.regions_selected,
        "selected_occupied_bytes": value.selected_occupied_bytes,
        "selection_skips": value.selection_skips,
        "selection_capacity_bytes": value.selection_capacity_bytes,
        "pinned_region_skips": value.pinned_region_skips,
        "hot_region_skips": value.hot_region_skips,
        "evacuation_regions_released": value.evacuation_regions_released,
        "evacuation_bytes_released": value.evacuation_bytes_released,
    })
}

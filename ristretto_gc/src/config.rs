//! Configuration and statistics for garbage collection.

/// Configuration for the garbage collector
#[derive(Debug, Clone)]
pub struct Configuration {
    /// Target allocation rate threshold before triggering collection (bytes)
    pub allocation_threshold: usize,
    /// Maximum pause time per incremental step (microseconds)
    pub max_pause_time_us: u64,
    /// Number of objects to process per incremental marking step
    pub incremental_step_size: usize,
}

impl Default for Configuration {
    /// Creates a default configuration for the garbage collector.
    /// This sets:
    /// - `allocation_threshold` to 8MB
    /// - `max_pause_time_us` to 100 microseconds
    /// - `incremental_step_size` to 1000 objects
    fn default() -> Self {
        Self {
            allocation_threshold: 8 * 1024 * 1024,
            max_pause_time_us: 100,
            incremental_step_size: 1000,
        }
    }
}

/// Statistics about garbage collection performance
#[derive(Clone, Debug, Default)]
pub struct Statistics {
    pub collections_started: usize,
    pub collections_completed: usize,
    pub objects_marked: usize,
    pub objects_swept: usize,
    pub bytes_allocated: usize,
    pub bytes_freed: usize,
    pub total_pause_time_us: u64,
    pub concurrent_time_us: u64,
    pub last_collection_start: Option<std::time::Instant>,
    pub last_collection_duration: Option<std::time::Duration>,
    pub total_collection_time: std::time::Duration,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration() {
        let config = Configuration::default();
        assert_eq!(config.allocation_threshold, 8 * 1024 * 1024);
        assert_eq!(config.max_pause_time_us, 100);
        assert_eq!(config.incremental_step_size, 1000);
    }

    #[test]
    fn default_statistics() {
        let stats = Statistics::default();
        assert_eq!(stats.collections_started, 0);
        assert_eq!(stats.collections_completed, 0);
        assert_eq!(stats.objects_marked, 0);
        assert_eq!(stats.objects_swept, 0);
        assert_eq!(stats.bytes_allocated, 0);
        assert_eq!(stats.bytes_freed, 0);
        assert_eq!(stats.total_pause_time_us, 0);
        assert_eq!(stats.concurrent_time_us, 0);
        assert!(stats.last_collection_start.is_none());
        assert!(stats.last_collection_duration.is_none());
        assert_eq!(stats.total_collection_time, std::time::Duration::new(0, 0));
    }
}

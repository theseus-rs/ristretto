//! Configuration and statistics for garbage collection.

/// Configuration for the garbage collector
#[derive(Debug, Clone)]
pub struct Configuration {
    /// Number of threads to use for garbage collection. A value of `0` means the collector will use
    /// 50% of available CPU cores.
    pub threads: usize,
    /// Target allocation rate threshold before triggering collection (bytes)
    pub allocation_threshold: usize,
    /// Maximum pause time per incremental step (microseconds)
    pub max_pause_time_us: u64,
    /// Number of objects to process per incremental marking step
    pub incremental_step_size: usize,
    /// Threshold for parallel collection (objects)
    pub parallel_threshold: usize,
}

impl Default for Configuration {
    /// Creates a default configuration for the garbage collector.
    /// This sets:
    /// - `threads` to `0` (use 50% of available CPU cores)
    /// - `allocation_threshold` to 8MB
    /// - `max_pause_time_us` to 100 microseconds
    /// - `incremental_step_size` to 1000 objects
    fn default() -> Self {
        Self {
            threads: 0,
            allocation_threshold: 8 * 1024 * 1024,
            max_pause_time_us: 100,
            incremental_step_size: 1000,
            parallel_threshold: 1_000_000,
        }
    }
}

impl Configuration {
    /// Creates a new `ConfigurationBuilder` with default values.
    #[must_use]
    pub fn builder() -> ConfigurationBuilder {
        ConfigurationBuilder::new()
    }
}

/// Builder for creating a `Configuration` with a fluent interface.
///
/// # Example
///
/// ```rust
/// use ristretto_gc::ConfigurationBuilder;
///
/// let config = ConfigurationBuilder::new()
///     .threads(4)
///     .allocation_threshold(16 * 1024 * 1024)
///     .max_pause_time_us(50)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct ConfigurationBuilder {
    threads: usize,
    allocation_threshold: usize,
    max_pause_time_us: u64,
    incremental_step_size: usize,
    parallel_threshold: usize,
}

impl Default for ConfigurationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigurationBuilder {
    /// Creates a new `ConfigurationBuilder` with default values.
    ///
    /// Default values:
    /// - `threads`: `0` (use 50% of available CPU cores)
    /// - `allocation_threshold`: 8MB
    /// - `max_pause_time_us`: 100 microseconds
    /// - `incremental_step_size`: 1000 objects
    /// - `parallel_threshold`: 1,000,000 objects
    #[must_use]
    pub fn new() -> Self {
        let defaults = Configuration::default();
        Self {
            threads: defaults.threads,
            allocation_threshold: defaults.allocation_threshold,
            max_pause_time_us: defaults.max_pause_time_us,
            incremental_step_size: defaults.incremental_step_size,
            parallel_threshold: defaults.parallel_threshold,
        }
    }

    /// Sets the number of threads for garbage collection.
    ///
    /// A value of `0` means the collector will use 50% of available CPU cores.
    #[must_use]
    pub fn threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    /// Sets the allocation threshold in bytes before triggering collection.
    #[must_use]
    pub fn allocation_threshold(mut self, threshold: usize) -> Self {
        self.allocation_threshold = threshold;
        self
    }

    /// Sets the maximum pause time per incremental step in microseconds.
    #[must_use]
    pub fn max_pause_time_us(mut self, pause_time: u64) -> Self {
        self.max_pause_time_us = pause_time;
        self
    }

    /// Sets the number of objects to process per incremental marking step.
    #[must_use]
    pub fn incremental_step_size(mut self, step_size: usize) -> Self {
        self.incremental_step_size = step_size;
        self
    }

    /// Sets the threshold for parallel collection (number of objects).
    #[must_use]
    pub fn parallel_threshold(mut self, threshold: usize) -> Self {
        self.parallel_threshold = threshold;
        self
    }

    /// Builds the `Configuration` from this builder.
    #[must_use]
    pub fn build(self) -> Configuration {
        Configuration {
            threads: self.threads,
            allocation_threshold: self.allocation_threshold,
            max_pause_time_us: self.max_pause_time_us,
            incremental_step_size: self.incremental_step_size,
            parallel_threshold: self.parallel_threshold,
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
        assert_eq!(config.threads, 0);
        assert_eq!(config.allocation_threshold, 8 * 1024 * 1024);
        assert_eq!(config.max_pause_time_us, 100);
        assert_eq!(config.incremental_step_size, 1000);
        assert_eq!(config.parallel_threshold, 1_000_000);
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

    #[test]
    fn configuration_builder_defaults() {
        let config = ConfigurationBuilder::new().build();
        assert_eq!(config.threads, 0);
        assert_eq!(config.allocation_threshold, 8 * 1024 * 1024);
        assert_eq!(config.max_pause_time_us, 100);
        assert_eq!(config.incremental_step_size, 1000);
        assert_eq!(config.parallel_threshold, 1_000_000);
    }

    #[test]
    fn configuration_builder_threads() {
        let config = ConfigurationBuilder::new().threads(4).build();
        assert_eq!(config.threads, 4);
    }

    #[test]
    fn configuration_builder_allocation_threshold() {
        let config = ConfigurationBuilder::new()
            .allocation_threshold(16 * 1024 * 1024)
            .build();
        assert_eq!(config.allocation_threshold, 16 * 1024 * 1024);
    }

    #[test]
    fn configuration_builder_max_pause_time_us() {
        let config = ConfigurationBuilder::new().max_pause_time_us(50).build();
        assert_eq!(config.max_pause_time_us, 50);
    }

    #[test]
    fn configuration_builder_incremental_step_size() {
        let config = ConfigurationBuilder::new()
            .incremental_step_size(2000)
            .build();
        assert_eq!(config.incremental_step_size, 2000);
    }

    #[test]
    fn configuration_builder_parallel_threshold() {
        let config = ConfigurationBuilder::new()
            .parallel_threshold(500_000)
            .build();
        assert_eq!(config.parallel_threshold, 500_000);
    }

    #[test]
    fn configuration_builder_fluent_chain() {
        let config = ConfigurationBuilder::new()
            .threads(2)
            .allocation_threshold(4 * 1024 * 1024)
            .max_pause_time_us(200)
            .incremental_step_size(500)
            .parallel_threshold(100_000)
            .build();

        assert_eq!(config.threads, 2);
        assert_eq!(config.allocation_threshold, 4 * 1024 * 1024);
        assert_eq!(config.max_pause_time_us, 200);
        assert_eq!(config.incremental_step_size, 500);
        assert_eq!(config.parallel_threshold, 100_000);
    }

    #[test]
    fn configuration_builder_from_configuration() {
        let config = Configuration::builder().threads(1).build();
        assert_eq!(config.threads, 1);
    }

    #[test]
    fn configuration_builder_default_impl() {
        let builder = ConfigurationBuilder::default();
        let config = builder.build();
        assert_eq!(config.threads, 0);
        assert_eq!(config.allocation_threshold, 8 * 1024 * 1024);
    }
}

//! Fixed-memory latency recording shared by native GC workload drivers.
use std::collections::BTreeMap;
use std::time::Duration;

pub(crate) struct Histogram {
    buckets: Vec<u64>,
    pub(crate) count: u64,
    pub(crate) max_ns: u64,
}
impl Default for Histogram {
    fn default() -> Self {
        Self {
            buckets: vec![0; 4096],
            count: 0,
            max_ns: 0,
        }
    }
}
impl Histogram {
    pub(crate) fn record(&mut self, elapsed: Duration) {
        let ns = u64::try_from(elapsed.as_nanos()).unwrap_or(u64::MAX);
        let us = ns.div_ceil(1000);
        // Exact microseconds through 1024 us, then 64 subdivisions per doubling.
        // Fixed storage keeps backlog from adding allocations to the measured load.
        let index = if us <= 1024 {
            usize::try_from(us).unwrap_or(1024)
        } else {
            let exponent = u64::BITS - 1 - us.leading_zeros();
            let width = 1_u64 << (exponent - 6);
            let sub = (us - (1_u64 << exponent)).div_ceil(width);
            1024 + (exponent as usize - 10) * 64 + usize::try_from(sub).unwrap_or(64)
        };
        // Clamping ns above bounds even Duration::MAX within these 4096 bins.
        if let Some(bucket) = self.buckets.get_mut(index) {
            *bucket += 1;
        }
        self.count += 1;
        self.max_ns = self.max_ns.max(ns);
    }
    pub(crate) fn percentile_us(&self, numerator: u64, denominator: u64) -> u64 {
        let rank = self.count.saturating_mul(numerator).div_ceil(denominator);
        let mut cumulative = 0;
        for (index, count) in self.buckets.iter().enumerate() {
            cumulative += count;
            if cumulative >= rank {
                return Self::upper_us(index);
            }
        }
        0
    }

    fn upper_us(index: usize) -> u64 {
        if index <= 1024 {
            return index as u64;
        }
        let relative = index - 1024;
        let exponent = 10 + relative / 64;
        (1_u64 << exponent) + (relative % 64) as u64 * (1_u64 << (exponent - 6))
    }

    pub(crate) fn nonempty_buckets(&self) -> BTreeMap<u64, u64> {
        self.buckets
            .iter()
            .enumerate()
            .filter_map(|(index, count)| (*count != 0).then_some((Self::upper_us(index), *count)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn histogram_preserves_gate_precision_and_bounds_large_values() {
        for us in [
            0, 1, 99, 100, 999, 1000, 1001, 1024, 1025, 2047, 2048, 2049, 1_000_000,
        ] {
            let mut histogram = Histogram::default();
            histogram.record(Duration::from_micros(us));
            let upper = histogram.percentile_us(1, 1);
            assert!(upper >= us);
            if us <= 1024 {
                assert_eq!(upper, us);
            } else {
                assert!(upper <= us.saturating_mul(65).div_ceil(64));
            }
            assert_eq!(histogram.buckets.iter().sum::<u64>(), 1);
        }
        let mut histogram = Histogram::default();
        let capacity = histogram.buckets.capacity();
        for us in (0..10_000_000).step_by(137) {
            histogram.record(Duration::from_micros(us));
        }
        histogram.record(Duration::MAX);
        assert_eq!(histogram.buckets.capacity(), capacity);
        assert_eq!(histogram.buckets.iter().sum::<u64>(), histogram.count);
        assert!(histogram.percentile_us(1, 1) >= u64::MAX.div_ceil(1000));
    }
}

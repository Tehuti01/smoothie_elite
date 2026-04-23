/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x2ec96d08 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-net/src/jitter_buffer.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Adaptive jitter buffer for smoothing network audio arrivals.

use core::cmp::min;

const DEFAULT_BUFFER_DEPTH: usize = 128;
const MIN_JITTER_SAMPLES: usize = 32;
const MAX_JITTER_SAMPLES: usize = 512;

#[repr(align(64))]
/// Technical implementation of the JitterBufferConfig structure.
pub struct JitterBufferConfig {
    pub min_latency_samples: usize,
    pub max_latency_samples: usize,
    pub target_latency_samples: usize,
    pub adaptive_mode: bool,
    pub spike_threshold_ms: f64,
    pub settling_time_ms: f64,
}

impl Default for JitterBufferConfig {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            min_latency_samples: 128,
            max_latency_samples: 512,
            target_latency_samples: 256,
            adaptive_mode: true,
            spike_threshold_ms: 20.0,
            settling_time_ms: 500.0,
        }
    }
}

#[repr(align(64))]
/// Technical implementation of the JitterBuffer structure.
pub struct JitterBuffer<T, const N: usize> {
    buffer: [Option<T>; N],
    write_index: usize,
    read_index: usize,
    valid_count: usize,
    config: JitterBufferConfig,
    current_latency: usize,
    spike_count: u32,
    settled: bool,
    settle_start_ms: f64,
}

impl<T, const N: usize> JitterBuffer<T, N> {
    /// Initializes a new instance of the associated type.
    pub fn new(config: JitterBufferConfig) -> Self {
        let depth = config.max_latency_samples.min(N);
        Self {
            buffer: [None; N],
            write_index: 0,
            read_index: 0,
            valid_count: 0,
            config,
            current_latency: config.target_latency_samples,
            spike_count: 0,
            settled: false,
            settle_start_ms: 0.0,
        }
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.write_index = 0;
        self.read_index = 0;
        self.valid_count = 0;
        self.current_latency = self.config.target_latency_samples;
        self.spike_count = 0;
        self.settled = false;
    }

    #[inline(always)]
    /// Technical implementation of the push logic.
    pub fn push(&mut self, value: T) -> bool {
        if self.valid_count >= N {
            return false;
        }

        self.buffer[self.write_index] = Some(value);
        self.write_index = (self.write_index + 1) & (N - 1);
        self.valid_count += 1;
        true
    }

    #[inline(always)]
    /// Technical implementation of the push_batch logic.
    pub fn push_batch(&mut self, values: &[T]) -> usize {
        let mut pushed = 0;
        for v in values {
            if self.push((*v).clone()) {
                pushed += 1;
            } else {
                break;
            }
        }
        pushed
    }

    #[inline(always)]
    /// Technical implementation of the pop logic.
    pub fn pop(&mut self) -> Option<T> {
        if self.valid_count == 0 {
            return None;
        }

        let value = self.buffer[self.read_index].take();
        self.read_index = (self.read_index + 1) & (N - 1);
        self.valid_count -= 1;
        value
    }

    #[inline(always)]
    /// Technical implementation of the peek logic.
    pub fn peek(&self) -> Option<&T> {
        if self.valid_count == 0 {
            return None;
        }
        self.buffer[self.read_index].as_ref()
    }

    #[inline(always)]
    /// Technical implementation of the len logic.
    pub fn len(&self) -> usize {
        self.valid_count
    }

    #[inline(always)]
    /// Technical implementation of the is_empty logic.
    pub fn is_empty(&self) -> bool {
        self.valid_count == 0
    }

    #[inline(always)]
    /// Technical implementation of the is_full logic.
    pub fn is_full(&self) -> bool {
        self.valid_count >= N
    }

    /// Technical implementation of the latency_samples logic.
    pub fn latency_samples(&self) -> usize {
        self.valid_count
    }

    /// Technical implementation of the current_latency_ms logic.
    pub fn current_latency_ms(&self, sample_rate: u32) -> f64 {
        (self.valid_count as f64) / (sample_rate as f64 / 1000.0)
    }

    /// Technical implementation of the adaptive_update logic.
    pub fn adaptive_update(&mut self, arrival_delta_ms: f64, sample_rate: u32) {
        if !self.config.adaptive_mode {
            return;
        }

        let threshold = self.config.spike_threshold_ms;

        if arrival_delta_ms > threshold {
            self.spike_count += 1;
        } else {
            self.spike_count = self.spike_count.saturating_sub(1);
        }

        if self.spike_count > 10 {
            let target = self.config.target_latency_samples;
            let new_latency = min(target + 64, self.config.max_latency_samples);

            if new_latency != self.current_latency {
                self.current_latency = new_latency;
                self.spike_count = 0;
            }
        } else if self.spike_count == 0 && self.current_latency > self.config.min_latency_samples {
            let new_latency = self.current_latency.saturating_sub(1);
            if new_latency >= self.config.min_latency_samples {
                self.current_latency = new_latency;
            }
        }
    }

    /// Technical implementation of the underrun_count logic.
    pub fn underrun_count(&self) -> usize {
        0
    }

    /// Technical implementation of the set_target_latency logic.
    pub fn set_target_latency(&mut self, samples: usize) {
        self.current_latency = samples.clamp(
            self.config.min_latency_samples,
            self.config.max_latency_samples,
        );
    }
}

impl<T, const N: usize> Default for JitterBuffer<T, N> {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(JitterBufferConfig::default())
    }
}

#[repr(align(64))]
/// Technical implementation of the PacketWindow structure.
pub struct PacketWindow {
    arrivals: [u64; 64],
    timestamps: [u64; 64],
    count: usize,
    index: usize,
    last_arrival: u64,
}

impl PacketWindow {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            arrivals: [0u64; 64],
            timestamps: [0u64; 64],
            count: 0,
            index: 0,
            last_arrival: 0,
        }
    }

    /// Technical implementation of the record logic.
    pub fn record(&mut self, packet_sequence: u64, arrival_timestamp: u64) {
        let idx = self.index & 63;
        self.arrivals[idx] = packet_sequence;
        self.timestamps[idx] = arrival_timestamp;
        self.index = self.index.wrapping_add(1);
        if self.count < 64 {
            self.count += 1;
        }
        self.last_arrival = arrival_timestamp;
    }

    /// Technical implementation of the inter_arrival_mean logic.
    pub fn inter_arrival_mean(&self) -> u64 {
        if self.count < 2 {
            return 0;
        }

        let mut sum = 0u64;
        let mut last_ts = 0u64;

        for i in 0..self.count {
            let idx = (self.index - self.count + i) & 63;
            if last_ts != 0 {
                sum += self.timestamps[idx].saturating_sub(last_ts);
            }
            last_ts = self.timestamps[idx];
        }

        sum / (self.count - 1) as u64
    }

    /// Technical implementation of the jitter_variance logic.
    pub fn jitter_variance(&self) -> u64 {
        if self.count < 3 {
            return 0;
        }

        let mean = self.inter_arrival_mean();
        let mut sum_sq = 0u64;
        let mut last_ts = 0u64;
        let mut n = 0;

        for i in 0..self.count {
            let idx = (self.index - self.count + i) & 63;
            if last_ts != 0 {
                let diff = self.timestamps[idx].saturating_sub(last_ts);
                let diff_mean = diff.saturating_sub(mean);
                sum_sq = sum_sq.saturating_add(diff_mean * diff_mean);
                n += 1;
            }
            last_ts = self.timestamps[idx];
        }

        if n > 0 {
            sum_sq / n as u64
        } else {
            0
        }
    }
}

impl Default for PacketWindow {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_jitter_buffer_basic logic.
    fn test_jitter_buffer_basic() {
        let config = JitterBufferConfig::default();
        let mut buffer: JitterBuffer<i32, 128> = JitterBuffer::new(config);

        assert!(buffer.push(42));
        assert_eq!(buffer.pop(), Some(42));
        assert!(buffer.is_empty());
    }

    #[test]
    /// Technical implementation of the test_jitter_buffer_full logic.
    fn test_jitter_buffer_full() {
        let config = JitterBufferConfig::default();
        let mut buffer: JitterBuffer<i32, 4> = JitterBuffer::new(config);

        assert!(buffer.push(1));
        assert!(buffer.push(2));
        assert!(buffer.push(3));
        assert!(buffer.push(4));
        assert!(buffer.is_full());
        assert!(!buffer.push(5));
    }
}

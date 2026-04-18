use std::collections::VecDeque;

pub struct Delay {
    sample_rate: f64,
    buffer: VecDeque<f32>,
    max_delay_samples: usize,
    delay_samples: usize,
    feedback: f32,
    mix: f32,
    wet: f32,
    dry: f32,
}

impl Delay {
    pub fn new(sample_rate: f64, max_delay_ms: f64) -> Self {
        let max_delay_samples = ((sample_rate * max_delay_ms / 1000.0) as usize).max(1);
        Self {
            sample_rate,
            buffer: VecDeque::with_capacity(max_delay_samples + 1),
            max_delay_samples,
            delay_samples: (sample_rate * 0.3 / 1000.0) as usize,
            feedback: 0.3,
            mix: 0.5,
            wet: 0.5,
            dry: 0.5,
        }
    }

    pub fn set_delay_ms(&mut self, ms: f64) {
        self.delay_samples = ((self.sample_rate * ms / 1000.0) as usize)
            .min(self.max_delay_samples)
            .max(1);
    }

    pub fn set_delay_samples(&mut self, samples: usize) {
        self.delay_samples = samples.min(self.max_delay_samples).max(1);
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback.clamp(0.0, 0.95);
    }

    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
        self.wet = self.mix;
        self.dry = 1.0 - self.mix;
    }

    pub fn process(&mut self, input: f32) -> f32 {
        if self.buffer.is_empty() {
             for _ in 0..self.max_delay_samples {
                 self.buffer.push_back(0.0);
             }
        }

        let delayed = if self.delay_samples > 0 && self.delay_samples < self.buffer.len() {
            // We want the sample from 'delay_samples' ago.
            // Since we push to back and pop from front, the front is the oldest.
            // If the buffer size is max_delay_samples, the sample from D ago is at len - D?
            // Actually, if we keep size constant at max_delay_samples:
            // The sample from 1 ago was pushed last (back).
            // The sample from delay_samples ago is at index (len - 1) - delay_samples? No.
            // Let's use it as a simple queue: push to back, pop from front.
            // If the size is EXACTLY delay_samples, the front is the sample from delay_samples ago.
            
            // To support variable delay with a fixed max buffer:
            // We can just look 'delay_samples' back from the current end.
            let idx = self.buffer.len().saturating_sub(self.delay_samples).max(0);
            self.buffer.get(idx).copied().unwrap_or(0.0)
        } else {
            0.0
        };

        let output = input * self.dry + delayed * self.wet;
        let feedback_sample = input + delayed * self.feedback;

        self.buffer.push_back(feedback_sample);
        if self.buffer.len() > self.max_delay_samples {
            self.buffer.pop_front();
        }

        output
    }

    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        (self.process(left), self.process(right))
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl Default for Delay {
    fn default() -> Self {
        Self::new(44100.0, 1000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay_processing() {
        let mut delay = Delay::new(44100.0, 100.0);
        delay.set_delay_samples(10);
        delay.set_mix(1.0); // 100% wet
        delay.set_feedback(0.0);

        // Input an impulse
        let _ = delay.process(1.0);
        
        // Next 9 samples should be 0 (if 100% wet and D=10)
        for _ in 0..9 {
            assert_eq!(delay.process(0.0), 0.0);
        }
        
        // 10th sample should be the impulse
        assert_eq!(delay.process(0.0), 1.0);
    }
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------

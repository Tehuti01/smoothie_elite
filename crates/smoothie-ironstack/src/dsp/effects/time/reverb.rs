use std::f32::consts::PI;

pub struct Reverb {
    sample_rate: f64,
    comb_delays: [usize; 8],
    comb_buffers: Vec<Vec<f32>>,
    comb_indices: Vec<usize>,
    allpass_delays: [usize; 4],
    allpass_buffers: Vec<Vec<f32>>,
    allpass_indices: Vec<usize>,
    comb_feedback: f32,
    damping: f32,
    damp_state: Vec<f32>,
    wet: f32,
    dry: f32,
    mix: f32,
}

impl Reverb {
    pub fn new(sample_rate: f64) -> Self {
        let comb_delay_times_ms = [29.7, 37.1, 41.1, 43.7, 31.7, 35.3, 36.9, 38.3];
        let allpass_delay_times_ms = [5.0, 1.7, 3.3, 2.0];

        let comb_delays: Vec<usize> = comb_delay_times_ms
            .iter()
            .map(|t| (*t * sample_rate / 1000.0) as usize)
            .collect();

        let allpass_delays: Vec<usize> = allpass_delay_times_ms
            .iter()
            .map(|t| (*t * sample_rate / 1000.0) as usize)
            .collect();

        let max_comb = *comb_delays.iter().max().unwrap_or(&1);
        let max_allpass = *allpass_delays.iter().max().unwrap_or(&1);

        let mut comb_buffers: Vec<Vec<f32>> = comb_delays.iter().map(|&d| vec![0.0; d]).collect();

        let allpass_buffers: Vec<Vec<f32>> = allpass_delays.iter().map(|&d| vec![0.0; d]).collect();

        Self {
            sample_rate,
            comb_delays: [29, 37, 41, 43, 31, 35, 36, 38],
            comb_buffers,
            comb_indices: vec![0; 8],
            allpass_delays: [5, 1, 3, 2],
            allpass_buffers,
            allpass_indices: vec![0; 4],
            comb_feedback: 0.84,
            damping: 0.2,
            damp_state: vec![0.0; 8],
            wet: 0.33,
            dry: 0.67,
            mix: 0.33,
        }
    }

    pub fn set_room_size(&mut self, size: f32) {
        self.comb_feedback = 0.7 + size * 0.24;
    }

    pub fn set_damping(&mut self, damp: f32) {
        self.damping = damp * 0.4;
    }

    pub fn set_wet(&mut self, wet: f32) {
        self.mix = wet.clamp(0.0, 1.0);
        self.wet = self.mix * 2.0;
        self.dry = 1.0 - self.mix;
    }

    pub fn set_mix(&mut self, mix: f32) {
        self.set_wet(mix);
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let mut output = 0.0;

        for i in 0..self.comb_buffers.len() {
            let buf = &mut self.comb_buffers[i];
            let idx = self.comb_indices[i];

            let delayed = buf[idx];
            let filtered = self.damp_state[i];
            let new_sample = input + delayed * self.comb_feedback;

            buf[idx] = new_sample;
            self.damp_state[i] = delayed * (1.0 - self.damping) + filtered * self.damping;

            self.comb_indices[i] = (idx + 1) % buf.len();
            output += delayed;
        }

        output /= self.comb_buffers.len() as f32;

        for i in 0..self.allpass_buffers.len() {
            let buf = &mut self.allpass_buffers[i];
            let idx = self.allpass_indices[i];

            let delayed = buf[idx];
            let new_sample = output + delayed * 0.5;
            buf[idx] = new_sample;
            output = delayed - new_sample * 0.5;

            self.allpass_indices[i] = (idx + 1) % buf.len();
        }

        output * self.wet + input * self.dry
    }

    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        (self.process(left), self.process(right))
    }

    pub fn clear(&mut self) {
        for buf in &mut self.comb_buffers {
            buf.fill(0.0);
        }
        for buf in &mut self.allpass_buffers {
            buf.fill(0.0);
        }
        self.damp_state.fill(0.0);
    }
}

impl Default for Reverb {
    fn default() -> Self {
        Self::new(44100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverb_processing() {
        let mut reverb = Reverb::new(44100.0);
        reverb.set_mix(1.0); // 100% wet
        
        // Input an impulse
        let initial_output = reverb.process(1.0);
        
        // Reverb should have long tail, so output should be non-zero for a while
        let mut has_tail = false;
        // Wait for impulse to propagate (longest delay is approx 43ms ~ 2000 samples)
        // We check for up to 10,000 samples
        for _ in 0..10000 {
            if reverb.process(0.0).abs() > 0.00001 {
                has_tail = true;
                break;
            }
        }
        assert!(has_tail);
    }

    #[test]
    fn test_reverb_clear() {
        let mut reverb = Reverb::new(44100.0);
        let _ = reverb.process(1.0);
        reverb.clear();
        assert_eq!(reverb.process(0.0), 0.0);
    }
}

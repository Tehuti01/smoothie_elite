//! 'Elite' modulation engines, including organic Neural LFOs 
//! and chaotic Gesture generators.

/// A high-performance, organic LFO where shapes are generated 
/// using a non-linear chaotic system (Neural-style feedback).
pub struct NeuralLfo {
    phase: f64,
    phase_inc: f64,
    sample_rate: f64,
    
    // Chaotic state
    segments: Vec<f64>,
    num_segments: usize,
    chaos_amount: f64,
    seed: f64,
}

impl NeuralLfo {
    pub fn new(sample_rate: f64, num_segments: usize) -> Self {
        let mut lfo = Self {
            phase: 0.0,
            phase_inc: 0.0,
            sample_rate,
            segments: vec![0.0; num_segments],
            num_segments,
            chaos_amount: 0.5,
            seed: 0.123,
        };
        lfo.regenerate();
        lfo
    }

    pub fn set_frequency(&mut self, hz: f64) {
        self.phase_inc = hz / self.sample_rate;
    }

    pub fn set_chaos(&mut self, amount: f64) {
        self.chaos_amount = amount.clamp(0.0, 1.0);
        self.regenerate();
    }

    /// Regenerate the neural shapes using a chaotic Logistic Map.
    pub fn regenerate(&mut self) {
        let r = 3.5 + self.chaos_amount * 0.45; // Logistic map parameter for chaos
        let mut x = self.seed;
        
        for i in 0..self.num_segments {
            // x_next = r * x * (1 - x)
            x = r * x * (1.0 - x);
            self.segments[i] = x * 2.0 - 1.0;
        }
    }

    /// Advance one sample and return output in [-1, 1].
    /// Uses cubic interpolation across segments for 'Elite' smoothness.
    pub fn next_sample(&mut self) -> f64 {
        let total_pos = self.phase * self.num_segments as f64;
        let idx_prev = (total_pos as usize) % self.num_segments;
        let idx_next = (idx_prev + 1) % self.num_segments;
        let idx_next2 = (idx_prev + 2) % self.num_segments;
        let idx_prev2 = (idx_prev + self.num_segments - 1) % self.num_segments;
        
        let t = total_pos - total_pos.floor();
        
        // Cubic Hermite Spline interpolation
        let p0 = self.segments[idx_prev2];
        let p1 = self.segments[idx_prev];
        let p2 = self.segments[idx_next];
        let p3 = self.segments[idx_next2];

        let out = 0.5 * (
            (2.0 * p1) +
            (-p0 + p2) * t +
            (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t * t +
            (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t * t * t
        );

        self.phase = (self.phase + self.phase_inc) % 1.0;
        out
    }
}

/// A high-performance modulation matrix for 'Elite' patch design.
pub struct ModulationMatrix {
    sources: Vec<f64>,
    offsets: Vec<f64>,
    amounts: Vec<f64>,
    destinations: Vec<f64>,
}

impl ModulationMatrix {
    pub fn new(num_sources: usize, num_destinations: usize) -> Self {
        Self {
            sources: vec![0.0; num_sources],
            offsets: vec![0.0; num_destinations],
            amounts: vec![0.0; num_sources * num_destinations],
            destinations: vec![0.0; num_destinations],
        }
    }

    pub fn set_source(&mut self, idx: usize, val: f64) {
        if idx < self.sources.len() { self.sources[idx] = val; }
    }

    pub fn set_amount(&mut self, src_idx: usize, dest_idx: usize, amount: f64) {
        let idx = src_idx * self.destinations.len() + dest_idx;
        if idx < self.amounts.len() { self.amounts[idx] = amount; }
    }

    pub fn set_offset(&mut self, dest_idx: usize, offset: f64) {
        if dest_idx < self.destinations.len() { self.offsets[dest_idx] = offset; }
    }

    /// Update the modulation matrix outputs.
    pub fn process(&mut self) {
        let num_dests = self.destinations.len();
        for d in 0..num_dests {
            let mut val = self.offsets[d];
            for s in 0..self.sources.len() {
                val += self.sources[s] * self.amounts[s * num_dests + d];
            }
            self.destinations[d] = val;
        }
    }

    pub fn get_destination(&self, idx: usize) -> f64 {
        self.destinations.get(idx).copied().unwrap_or(0.0)
    }
}

/// A non-periodic modulator aligned with the divine Phi ratio.
pub struct PhiLfo {
    phase: f64,
    phi_accumulator: f64,
    sample_rate: f64,
    frequency: f64,
}

impl PhiLfo {
    pub fn new(sample_rate: f64) -> Self {
        Self {
            phase: 0.0,
            phi_accumulator: 0.0,
            sample_rate,
            frequency: 1.0,
        }
    }

    pub fn set_frequency(&mut self, hz: f64) { self.frequency = hz; }

    pub fn next_value(&mut self) -> f64 {
        const PHI: f64 = 1.618033988749895;
        const PI: f64 = std::f64::consts::PI;
        let dt = self.frequency / self.sample_rate;
        self.phase = (self.phase + dt) % 1.0;
        self.phi_accumulator = (self.phi_accumulator + dt * PHI) % 1.0;
        (2.0 * PI * (self.phase + self.phi_accumulator)).sin() * 0.5 + 0.5
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

//! FDN (Feedback Delay Network) reverb with Hadamard mixing matrix.
//! 8-line FDN using prime-number delay lengths for maximum diffusion.

use smoothie_math::matrix::hadamard_in_place;

const NUM_LINES: usize = 8;

/// Delay line backed by a pre-allocated ring buffer.
struct DelayLine {
    buf:       Vec<f32>,
    write_ptr: usize,
    len:       usize,
}

impl DelayLine {
    fn new(len: usize) -> Self {
        Self { buf: vec![0.0; len], write_ptr: 0, len }
    }

    #[inline]
    fn write(&mut self, x: f32) {
        self.buf[self.write_ptr] = x;
        self.write_ptr = (self.write_ptr + 1) % self.len;
    }

    #[inline]
    fn read(&self) -> f32 {
        self.buf[self.write_ptr]
    }

    fn clear(&mut self) { self.buf.fill(0.0); }
}

/// All-pass diffuser for pre-diffusion.
struct AllpassDiffuser {
    buf:   Vec<f32>,
    ptr:   usize,
    len:   usize,
    coeff: f32,
}

impl AllpassDiffuser {
    fn new(len: usize, coeff: f32) -> Self {
        Self { buf: vec![0.0; len], ptr: 0, len, coeff }
    }

    #[inline]
    fn process(&mut self, x: f32) -> f32 {
        let delayed = self.buf[self.ptr];
        let v = x - self.coeff * delayed;
        self.buf[self.ptr] = v;
        self.ptr = (self.ptr + 1) % self.len;
        delayed + self.coeff * v
    }
}

/// 8-line FDN reverb with configurable room size, decay, and damping.
pub struct FdnReverb {
    delays:    [DelayLine; NUM_LINES],
    diffusers: [AllpassDiffuser; 4],
    fb_buf:    [f32; NUM_LINES],
    lp_state:  [f32; NUM_LINES],
    /// Feedback coefficient (controls decay time).
    pub decay:   f32,
    /// Lowpass damping coefficient [0, 1].
    pub damping: f32,
    /// Dry/wet mix [0, 1].
    pub mix:     f32,
}

// Prime delay lengths in samples (at 44100 Hz, scale by sample rate).
const DELAY_PRIMES: [usize; NUM_LINES] = [1117, 1481, 1949, 2377, 2819, 3163, 3581, 4049];

impl FdnReverb {
    pub fn new(sample_rate: f32) -> Self {
        let scale = sample_rate / 44100.0;
        let delays = std::array::from_fn(|i| {
            DelayLine::new(((DELAY_PRIMES[i] as f32 * scale) as usize).max(16))
        });
        let diffusers = [
            AllpassDiffuser::new(347, 0.6),
            AllpassDiffuser::new(113, 0.6),
            AllpassDiffuser::new(37,  0.6),
            AllpassDiffuser::new(67,  0.6),
        ];
        Self {
            delays,
            diffusers,
            fb_buf:   [0.0; NUM_LINES],
            lp_state: [0.0; NUM_LINES],
            decay:    0.85,
            damping:  0.3,
            mix:      0.3,
        }
    }

    /// Process a stereo pair in-place. `left`/`right` are single samples.
    #[inline]
    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        // Pre-diffuse the mono input
        let mono = (left + right) * 0.5;
        let d0 = self.diffusers[0].process(mono);
        let d1 = self.diffusers[1].process(d0);
        let d2 = self.diffusers[2].process(d1);
        let d3 = self.diffusers[3].process(d2);

        // Read from delay lines into mixing buffer
        let mut mix_buf = [0.0f32; NUM_LINES];
        for i in 0..NUM_LINES {
            mix_buf[i] = self.delays[i].read();
        }

        // Hadamard mixing
        hadamard_in_place(&mut mix_buf);

        // Apply damping (one-pole lowpass) and feedback, then write back
        let wet_l = mix_buf[0] + mix_buf[2];
        let wet_r = mix_buf[1] + mix_buf[3];

        let inp = if NUM_LINES > 4 { d3 } else { mono };
        for i in 0..NUM_LINES {
            let lp = self.lp_state[i] + (1.0 - self.damping) * (mix_buf[i] - self.lp_state[i]);
            self.lp_state[i] = lp;
            let fb = self.decay * lp + if i < 2 { inp * 0.5 } else { 0.0 };
            self.delays[i].write(fb);
        }

        let out_l = left  * (1.0 - self.mix) + wet_l * self.mix;
        let out_r = right * (1.0 - self.mix) + wet_r * self.mix;
        (out_l, out_r)
    }

    /// Process a block of interleaved stereo samples (L,R,L,R,...).
    pub fn process_block(&mut self, buf: &mut [f32]) {
        for chunk in buf.chunks_exact_mut(2) {
            let (l, r) = self.process_stereo(chunk[0], chunk[1]);
            chunk[0] = l;
            chunk[1] = r;
        }
    }

    pub fn reset(&mut self) {
        for d in self.delays.iter_mut() { d.clear(); }
        self.lp_state = [0.0; NUM_LINES];
        self.fb_buf   = [0.0; NUM_LINES];
    }
}

/// Schroeder reverb — classic 4 comb + 2 allpass topology.
pub struct SchroederReverb {
    combs:    [CombFilter; 4],
    allpasses: [SchroederAllpass; 2],
    pub mix: f32,
}

struct CombFilter {
    buf:     Vec<f32>,
    ptr:     usize,
    len:     usize,
    fb:      f32,
    damp:    f32,
    state:   f32,
}

impl CombFilter {
    fn new(len: usize, fb: f32) -> Self {
        Self { buf: vec![0.0; len], ptr: 0, len, fb, damp: 0.2, state: 0.0 }
    }
    #[inline]
    fn process(&mut self, x: f32) -> f32 {
        let out = self.buf[self.ptr];
        self.state = out * (1.0 - self.damp) + self.state * self.damp;
        self.buf[self.ptr] = x + self.state * self.fb;
        self.ptr = (self.ptr + 1) % self.len;
        out
    }
}

struct SchroederAllpass {
    buf: Vec<f32>,
    ptr: usize,
    len: usize,
    fb:  f32,
}

impl SchroederAllpass {
    fn new(len: usize, fb: f32) -> Self {
        Self { buf: vec![0.0; len], ptr: 0, len, fb }
    }
    #[inline]
    fn process(&mut self, x: f32) -> f32 {
        let delayed = self.buf[self.ptr];
        let v = x + delayed * (-self.fb);
        self.buf[self.ptr] = x + delayed * self.fb;
        self.ptr = (self.ptr + 1) % self.len;
        v
    }
}

impl SchroederReverb {
    pub fn new(_sample_rate: f32) -> Self {
        Self {
            combs: [
                CombFilter::new(1557, 0.84),
                CombFilter::new(1617, 0.84),
                CombFilter::new(1491, 0.84),
                CombFilter::new(1422, 0.84),
            ],
            allpasses: [
                SchroederAllpass::new(225, 0.5),
                SchroederAllpass::new(556, 0.5),
            ],
            mix: 0.25,
        }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let wet = self.combs.iter_mut().map(|c| c.process(x)).sum::<f32>() * 0.25;
        let a1 = self.allpasses[0].process(wet);
        let a2 = self.allpasses[1].process(a1);
        x * (1.0 - self.mix) + a2 * self.mix
    }
}

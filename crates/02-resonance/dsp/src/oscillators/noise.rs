/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x4e6f6973 | REVISION: 2026.04.20                           │
 * │ PATH: crates/02-resonance/dsp/src/oscillators/noise.rs                   │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: High-performance noise generation primitives.               │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

/// Fast 32-bit PRNG using Xorshift algorithm.
struct Xorshift {
    state: u32,
}

impl Xorshift {
    fn new(seed: u32) -> Self {
        Self { state: seed.max(1) }
    }

    fn process(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    fn next_f32(&mut self) -> f32 {
        (self.process() as f32 / u32::MAX as f32) * 2.0 - 1.0
    }
}

/// White noise generator.
pub struct WhiteNoise {
    rng: Xorshift,
}

impl WhiteNoise {
    pub fn new(seed: u32) -> Self {
        Self { rng: Xorshift::new(seed) }
    }

    #[inline(always)]
    pub fn process(&mut self) -> f32 {
        self.rng.next_f32()
    }
}

/// Pink noise generator using Voss-McCartney algorithm.
pub struct PinkNoise {
    rng: Xorshift,
    rows: [f32; 16],
    running_sum: f32,
    index: u32,
    count: u32,
}

impl PinkNoise {
    pub fn new(seed: u32) -> Self {
        Self {
            rng: Xorshift::new(seed),
            rows: [0.0; 16],
            running_sum: 0.0,
            index: 0,
            count: 0,
        }
    }

    #[inline(always)]
    pub fn process(&mut self) -> f32 {
        let mut i = 0;
        let mut tmp = self.index;
        while (tmp & 1) == 0 && i < 15 {
            i += 1;
            tmp >>= 1;
        }

        self.running_sum -= self.rows[i];
        let new_val = self.rng.next_f32() * 0.05; // Scaling for individual rows
        self.rows[i] = new_val;
        self.running_sum += new_val;

        self.index += 1;
        if self.index >= (1 << 16) {
            self.index = 0;
        }

        // Add a bit of white noise to fill the spectrum
        (self.running_sum + self.rng.next_f32() * 0.05) * 4.0
    }
}

impl Default for WhiteNoise {
    fn default() -> Self {
        Self::new(12345)
    }
}

impl Default for PinkNoise {
    fn default() -> Self {
        Self::new(54321)
    }
}

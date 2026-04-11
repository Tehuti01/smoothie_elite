//! Lock-free, allocation-free pseudo-random generators for audio use.

/// Xorshift32 PRNG — extremely fast, 32-bit state.
pub struct Xorshift32 {
    state: u32,
}

impl Xorshift32 {
    pub fn new(seed: u32) -> Self {
        Self { state: if seed == 0 { 1 } else { seed } }
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    /// Returns a float in [-1, 1].
    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0
    }

    /// Returns a float in [0, 1].
    #[inline]
    pub fn next_f32_01(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }
}

/// White noise generator.
pub struct WhiteNoise(Xorshift32);

impl WhiteNoise {
    pub fn new(seed: u32) -> Self { Self(Xorshift32::new(seed)) }
    #[inline]
    pub fn next(&mut self) -> f32 { self.0.next_f32() }
}

/// Pink noise via Paul Kellet's method (6-stage IIR approximation).
pub struct PinkNoise {
    rng: Xorshift32,
    b: [f32; 7],
}

impl PinkNoise {
    pub fn new(seed: u32) -> Self {
        Self { rng: Xorshift32::new(seed), b: [0.0; 7] }
    }

    #[inline]
    pub fn next(&mut self) -> f32 {
        let white = self.rng.next_f32();
        self.b[0] = 0.99886 * self.b[0] + white * 0.0555179;
        self.b[1] = 0.99332 * self.b[1] + white * 0.0750759;
        self.b[2] = 0.96900 * self.b[2] + white * 0.1538520;
        self.b[3] = 0.86650 * self.b[3] + white * 0.3104856;
        self.b[4] = 0.55000 * self.b[4] + white * 0.5329522;
        self.b[5] = -0.7616 * self.b[5] - white * 0.0168980;
        let pink = self.b[0]+self.b[1]+self.b[2]+self.b[3]+self.b[4]+self.b[5]+self.b[6]+white*0.5362;
        self.b[6] = white * 0.115926;
        pink * 0.11
    }
}

/// Velvet noise — sparse impulses with controlled density.
pub struct VelvetNoise {
    rng:     Xorshift32,
    density: f32, // fraction of samples that are impulses
}

impl VelvetNoise {
    pub fn new(density: f32, seed: u32) -> Self {
        Self { rng: Xorshift32::new(seed), density: density.clamp(0.0, 1.0) }
    }

    #[inline]
    pub fn next(&mut self) -> f32 {
        if self.rng.next_f32_01() < self.density {
            if self.rng.next_u32() & 1 == 0 { 1.0 } else { -1.0 }
        } else {
            0.0
        }
    }
}

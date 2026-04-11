//! Stereo widener, mid/side encoder/decoder, stereo imager.

/// Mid/Side encode a stereo pair.
#[inline]
pub fn ms_encode(l: f32, r: f32) -> (f32, f32) {
    ((l + r) * 0.5, (l - r) * 0.5)
}

/// Mid/Side decode back to L/R.
#[inline]
pub fn ms_decode(mid: f32, side: f32) -> (f32, f32) {
    (mid + side, mid - side)
}

/// Stereo widener via mid/side width control.
/// `width` = 1.0 is unchanged, 0.0 is mono, 2.0 doubles the side signal.
#[inline]
pub fn stereo_width(l: f32, r: f32, width: f32) -> (f32, f32) {
    let (mid, side) = ms_encode(l, r);
    ms_decode(mid, side * width)
}

/// Balance: `balance` in [-1, 1] attenuates one side.
#[inline]
pub fn stereo_balance(l: f32, r: f32, balance: f32) -> (f32, f32) {
    let l_gain = (1.0 - balance.max(0.0)).clamp(0.0, 1.0);
    let r_gain = (1.0 + balance.min(0.0)).clamp(0.0, 1.0);
    (l * l_gain, r * r_gain)
}

/// Haas effect: stereo widening by small time offset on one channel.
pub struct HaasWidener {
    buf:   Vec<f32>,
    ptr:   usize,
    delay: usize,
    /// Swap channels.
    pub swap: bool,
}

impl HaasWidener {
    /// `delay_ms` is typically 1–30 ms.
    pub fn new(delay_ms: f32, sample_rate: f32) -> Self {
        let delay = (delay_ms * sample_rate / 1000.0) as usize + 1;
        Self {
            buf: vec![0.0; delay + 4],
            ptr: 0,
            delay,
            swap: false,
        }
    }

    #[inline]
    pub fn process(&mut self, l: f32, r: f32) -> (f32, f32) {
        // Delay one channel
        let delayed = self.buf[self.ptr];
        self.buf[self.ptr] = if self.swap { l } else { r };
        self.ptr = (self.ptr + 1) % (self.delay + 1);
        if self.swap { (delayed, r) } else { (l, delayed) }
    }
}

/// Stereo crossfeed (for headphone listening) — blends a small amount of
/// high-cut L into R and vice-versa to simulate speaker imaging.
pub struct Crossfeed {
    hp_l: f32,
    hp_r: f32,
    lp_l: f32,
    lp_r: f32,
    /// Crossfeed level [0, 1].
    pub level: f32,
    /// High-cut coefficient.
    pub hf_cut: f32,
}

impl Crossfeed {
    pub fn new(sample_rate: f32) -> Self {
        let cutoff = 700.0_f32;
        let hf_cut = (-2.0 * std::f32::consts::PI * cutoff / sample_rate).exp();
        Self { hp_l: 0.0, hp_r: 0.0, lp_l: 0.0, lp_r: 0.0, level: 0.2, hf_cut }
    }

    #[inline]
    pub fn process(&mut self, l: f32, r: f32) -> (f32, f32) {
        // Low-pass the cross signal
        self.lp_l += (1.0 - self.hf_cut) * (l - self.lp_l);
        self.lp_r += (1.0 - self.hf_cut) * (r - self.lp_r);
        let out_l = l + self.lp_r * self.level;
        let out_r = r + self.lp_l * self.level;
        (out_l, out_r)
    }
}

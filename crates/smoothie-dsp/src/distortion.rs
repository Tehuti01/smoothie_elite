//! Waveshaping / saturation algorithms — all pure functions, zero allocation.

/// Soft-clip using a cubic waveshaper: y = x - x³/3  (Doidic et al.)
#[inline]
pub fn softclip(x: f32) -> f32 {
    let x = x.clamp(-1.5, 1.5);
    x - (x * x * x) / 3.0
}

/// Hardclip — pure brick-wall saturation.
#[inline]
pub fn hardclip(x: f32) -> f32 { x.clamp(-1.0, 1.0) }

/// Hyperbolic tangent shaper — warm tube-like saturation.
#[inline]
pub fn tanh_shaper(x: f32) -> f32 { x.tanh() }

/// Foldback distortion — the signal folds when it exceeds threshold.
#[inline]
pub fn foldback(x: f64, threshold: f64) -> f64 {
    if x.abs() > threshold {
        let over = x.abs() - threshold;
        if x > 0.0 { threshold - over } else { -threshold + over }
    } else {
        x
    }
}

/// A physical model of a Triode Vacuum Tube characteristic.
/// Provides asymmetrical soft-clipping and rich 2nd-order harmonics.
pub fn triode_model(x: f64, gain: f64, bias: f64) -> f64 {
    let drive = x * gain + bias;
    if drive > 0.0 {
        // Power-law distortion (Child's law approximation)
        drive.powf(1.5).min(1.0) * 2.0 - 1.0
    } else {
        -1.0
    }
}

/// A simplified magnetic tape hysteresis model.
/// Simulates the 'memory' effect of magnetic particles in tape.
pub struct TapeSaturator {
    last_out: f64,
    drive: f64,
}

impl TapeSaturator {
    pub fn new(drive: f64) -> Self {
        Self { last_out: 0.0, drive }
    }

    pub fn process(&mut self, x: f64) -> f64 {
        // Hysteresis calculation: output depends on current input and previous state
        let saturation_limit = 0.9;
        let delta = (x - self.last_out) * self.drive;
        let out = (self.last_out + delta).clamp(-saturation_limit, saturation_limit);
        
        // Soft-clipping at the limit
        let soft_out = if out.abs() > 0.7 {
            let sign = out.signum();
            let over = out.abs() - 0.7;
            sign * (0.7 + (over / (1.0 + over * over)))
        } else {
            out
        };
        
        self.last_out = soft_out;
        soft_out
    }
}

/// Simulated Tape Wow & Flutter (Low-frequency pitch modulation).
pub struct WowFlutter {
    lfo_wow: crate::oscillator::Oscillator,
    lfo_flutter: crate::oscillator::Oscillator,
    wow_amount: f32,
    flutter_amount: f32,
}

impl WowFlutter {
    pub fn new(sample_rate: f32) -> Self {
        let mut lfo_wow = crate::oscillator::Oscillator::new(crate::oscillator::WaveShape::Sine, sample_rate);
        let mut lfo_flutter = crate::oscillator::Oscillator::new(crate::oscillator::WaveShape::Sine, sample_rate);
        
        lfo_wow.set_frequency(0.5); // Wow: 0.5 Hz
        lfo_flutter.set_frequency(15.0); // Flutter: 15 Hz
        
        Self {
            lfo_wow,
            lfo_flutter,
            wow_amount: 0.0,
            flutter_amount: 0.0,
        }
    }

    pub fn set_amounts(&mut self, wow: f32, flutter: f32) {
        self.wow_amount = wow;
        self.flutter_amount = flutter;
    }

    /// Returns a playback speed multiplier (e.g., 1.0 + epsilon)
    pub fn next_multiplier(&mut self) -> f32 {
        let w = self.lfo_wow.next_sample() * self.wow_amount * 0.01;
        let f = self.lfo_flutter.next_sample() * self.flutter_amount * 0.005;
        1.0 + w + f
    }
}

/// A physical model of an iron-core Transformer.
/// Provides low-end 'mojo' and frequency-dependent saturation.
pub struct Transformer {
    lp: crate::filters::OnePoleFilter,
    hp: crate::filters::OnePoleFilter,
    drive: f64,
}

impl Transformer {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            lp: crate::filters::OnePoleFilter::new(sample_rate as f64, 400.0, FilterType::LowPass),
            hp: crate::filters::OnePoleFilter::new(sample_rate as f64, 20.0, FilterType::HighPass),
            drive: 1.0,
        }
    }

    pub fn set_drive(&mut self, val: f64) {
        self.drive = val;
    }

    pub fn process(&mut self, x: f64) -> f64 {
        // Low frequencies are saturated more in a transformer
        let low = self.lp.process(x);
        let high = x - low;
        
        let saturated_low = (low * self.drive).tanh();
        let out = (saturated_low + high);
        
        // Final DC block
        self.hp.process(out)
    }
}

/// Asymmetric tube saturation (second-harmonic generation).
#[inline]
pub fn tube_asymmetric(x: f32, drive: f32) -> f32 {
    let x = x * drive;
    if x >= 0.0 {
        tanh_shaper(x)
    } else {
        softclip(x) * 0.8
    }
}

/// Wavefolder — generates rich upper harmonics.
#[inline]
pub fn wavefold(x: f32) -> f32 {
    let x = x * std::f32::consts::PI;
    x.sin()
}

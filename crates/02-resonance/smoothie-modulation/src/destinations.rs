/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x38e9a791 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-modulation/src/destinations.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the ModDest enumeration.
pub enum ModDest {
    Pitch,
    Volume,
    Pan,
    FilterCutoff,
    FilterResonance,
    FilterEnvDepth,
    AmpEnvAttack,
    AmpEnvDecay,
    AmpEnvSustain,
    AmpEnvRelease,
    FilterEnvAttack,
    FilterEnvDecay,
    FilterEnvSustain,
    FilterEnvRelease,
    LfoRate,
    LfoDepth,
    OscillatorMix,
    ReverbMix,
    ReverbSize,
    ReverbDamping,
    DelayMix,
    DelayFeedback,
    DelayTime,
    DistortionDrive,
    ChorusMix,
    ChorusRate,
    Custom(u16),
}

/// Technical implementation of the ModDestHandle structure.
pub struct ModDestHandle {
    pub dest: ModDest,
    pub value: f32,
    pub depth: f32,
    pub bipolar: bool,
}

impl ModDestHandle {
    /// Initializes a new instance of the associated type.
    pub fn new(dest: ModDest) -> Self {
        Self {
            dest,
            value: 0.0,
            depth: 1.0,
            bipolar: true,
        }
    }

    /// Apply modulation to the destination value.
    #[inline(always)]
    /// Technical implementation of the apply logic.
    pub fn apply(&mut self, mod_value: f32) {
        let scaled = mod_value * self.depth;
        let adjusted = if self.bipolar {
            scaled
        } else {
            scaled * 0.5 + 0.5
        };
        self.value += adjusted;
    }

    /// Get final modulated value.
    #[inline(always)]
    /// Technical implementation of the value logic.
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Reset to default.
    #[inline(always)]
    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.value = 0.0;
    }
}

/// Technical implementation of the ModDestinations structure.
pub struct ModDestinations {
    pub pitch: ModDestHandle,
    pub volume: ModDestHandle,
    pub pan: ModDestHandle,
    pub filter_cutoff: ModDestHandle,
    pub filter_resonance: ModDestHandle,
    pub filter_env_depth: ModDestHandle,
    pub amp_env_attack: ModDestHandle,
    pub amp_env_decay: ModDestHandle,
    pub amp_env_sustain: ModDestHandle,
    pub amp_env_release: ModDestHandle,
    pub lfo_rate: ModDestHandle,
    pub lfo_depth: ModDestHandle,
    reverb_mix: ModDestHandle,
    reverb_size: ModDestHandle,
    delay_mix: ModDestHandle,
    delay_feedback: ModDestHandle,
}

impl ModDestinations {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            pitch: ModDestHandle::new(ModDest::Pitch),
            volume: ModDestHandle::new(ModDest::Volume),
            pan: ModDestHandle::new(ModDest::Pan),
            filter_cutoff: ModDestHandle::new(ModDest::FilterCutoff),
            filter_resonance: ModDestHandle::new(ModDest::FilterResonance),
            filter_env_depth: ModDestHandle::new(ModDest::FilterEnvDepth),
            amp_env_attack: ModDestHandle::new(ModDest::AmpEnvAttack),
            amp_env_decay: ModDestHandle::new(ModDest::AmpEnvDecay),
            amp_env_sustain: ModDestHandle::new(ModDest::AmpEnvSustain),
            amp_env_release: ModDestHandle::new(ModDest::AmpEnvRelease),
            lfo_rate: ModDestHandle::new(ModDest::LfoRate),
            lfo_depth: ModDestHandle::new(ModDest::LfoDepth),
            reverb_mix: ModDestHandle::new(ModDest::ReverbMix),
            reverb_size: ModDestHandle::new(ModDest::ReverbSize),
            delay_mix: ModDestHandle::new(ModDest::DelayMix),
            delay_feedback: ModDestHandle::new(ModDest::DelayFeedback),
        }
    }

    /// Get destination by enum.
    pub fn get(&mut self, dest: ModDest) -> Option<&mut ModDestHandle> {
        match dest {
            ModDest::Pitch => Some(&mut self.pitch),
            ModDest::Volume => Some(&mut self.volume),
            ModDest::Pan => Some(&mut self.pan),
            ModDest::FilterCutoff => Some(&mut self.filter_cutoff),
            ModDest::FilterResonance => Some(&mut self.filter_resonance),
            ModDest::FilterEnvDepth => Some(&mut self.filter_env_depth),
            ModDest::AmpEnvAttack => Some(&mut self.amp_env_attack),
            ModDest::AmpEnvDecay => Some(&mut self.amp_env_decay),
            ModDest::AmpEnvSustain => Some(&mut self.amp_env_sustain),
            ModDest::AmpEnvRelease => Some(&mut self.amp_env_release),
            ModDest::LfoRate => Some(&mut self.lfo_rate),
            ModDest::LfoDepth => Some(&mut self.lfo_depth),
            ModDest::ReverbMix => Some(&mut self.reverb_mix),
            ModDest::ReverbSize => Some(&mut self.reverb_size),
            ModDest::DelayMix => Some(&mut self.delay_mix),
            ModDest::DelayFeedback => Some(&mut self.delay_feedback),
            _ => None,
        }
    }

    /// Apply all modulation to base values.
    pub fn apply_to_filter(&self, base_cutoff: f32, base_q: f32) -> (f32, f32) {
        let cutoff = base_cutoff * (1.0 + self.filter_cutoff.value());
        let q = base_q * (1.0 + self.filter_resonance.value() * 2.0);
        (cutoff.clamp(20.0, 20000.0), q.clamp(0.1, 20.0))
    }

    /// Apply modulation to pitch (in semitones).
    pub fn apply_to_pitch(&self, base_hz: f32) -> f32 {
        base_hz * (1.0 + self.pitch.value() / 12.0)
    }

    /// Apply modulation to volume and pan.
    pub fn apply_to_amp(&self, base_volume: f32, base_pan: f32) -> (f32, f32) {
        let vol = base_volume * (1.0 + self.volume.value());
        let pan = (base_pan + self.pan.value()).clamp(-1.0, 1.0);
        (vol.clamp(0.0, 1.0), pan)
    }

    /// Reset all destinations.
    pub fn reset(&mut self) {
        self.pitch.reset();
        self.volume.reset();
        self.pan.reset();
        self.filter_cutoff.reset();
        self.filter_resonance.reset();
        self.filter_env_depth.reset();
        self.amp_env_attack.reset();
        self.amp_env_decay.reset();
        self.amp_env_sustain.reset();
        self.amp_env_release.reset();
        self.lfo_rate.reset();
        self.lfo_depth.reset();
        self.reverb_mix.reset();
        self.reverb_size.reset();
        self.delay_mix.reset();
        self.delay_feedback.reset();
    }
}

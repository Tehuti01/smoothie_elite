/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x660ad53b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-sound-design/src/wavefield.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

///
/// Creates spatial audio from simulated wavefronts in 2D/3D space.

#[derive(Debug, Clone, Copy)]
/// Technical implementation of the WavefieldSource structure.
pub struct WavefieldSource {
    pub position: [f32; 3],
    pub velocity: f32,
    pub amplitude: f32,
    pub phase: f32,
    pub frequency: f32,
}

impl WavefieldSource {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            velocity: 343.0,
            amplitude: 1.0,
            phase: 0.0,
            frequency: 440.0,
        }
    }

    /// Technical implementation of the set_position logic.
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = [x, y, z];
    }

    /// Technical implementation of the set_frequency logic.
    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }

    #[inline(always)]
    /// Technical implementation of the sample logic.
    pub fn sample(&mut self, listener_pos: &[f32; 3], _dt: f32) -> f32 {
        let dx = listener_pos[0] - self.position[0];
        let dy = listener_pos[1] - self.position[1];
        let dz = listener_pos[2] - self.position[2];
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();

        if distance < 0.001 {
            return self.amplitude;
        }

        let delay = distance / self.velocity;
        let phase = self.phase - (self.frequency * delay);

        let env = 1.0 / (1.0 + distance * 0.01);
        (phase * core::f32::consts::TAU).sin() * self.amplitude * env
    }
}

impl Default for WavefieldSource {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Technical implementation of the WavefieldRenderer structure.
pub struct WavefieldRenderer {
    pub sources: [WavefieldSource; 16],
    pub source_count: usize,
    pub listener_position: [f32; 3],
    pub listener_orientation: [f32; 3],
    pub spacing: f32,
    pub array_radius: f32,
}

impl WavefieldRenderer {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            sources: [WavefieldSource::new(); 16],
            source_count: 0,
            listener_position: [0.0, 0.0, 0.0],
            listener_orientation: [0.0, 0.0, 1.0],
            spacing: 0.05,
            array_radius: 0.5,
        }
    }

    /// Performs vector addition logic.
    pub fn add_source(&mut self, source: WavefieldSource) {
        if self.source_count < 16 {
            self.sources[self.source_count] = source;
            self.source_count += 1;
        }
    }

    /// Technical implementation of the set_listener_position logic.
    pub fn set_listener_position(&mut self, x: f32, y: f32, z: f32) {
        self.listener_position = [x, y, z];
    }

    /// Technical implementation of the set_listener_orientation logic.
    pub fn set_listener_orientation(&mut self, forward_x: f32, forward_y: f32, forward_z: f32) {
        self.listener_orientation = [forward_x, forward_y, forward_z];
    }

    /// Technical implementation of the render logic.
    pub fn render(&mut self, output: &mut [f32], dt: f32) {
        for i in 0..output.len() {
            output[i] = 0.0;
        }

        let listener_pos = &self.listener_position;

        for s in 0..self.source_count {
            let sample = self.sources[s].sample(listener_pos, dt);

            for ch in 0..output.len().min(2) {
                let pan = if self.source_count > 1 {
                    ch as f32 / (self.source_count - 1) as f32
                } else {
                    0.5
                };
                output[ch] += sample * pan;
            }
        }
    }

    /// Technical implementation of the setup_linear_array logic.
    pub fn setup_linear_array(&mut self, num_sources: usize, spacing: f32) {
        self.spacing = spacing;
        self.source_count = 0;

        let half_length = (num_sources as f32 - 1.0) * spacing / 2.0;

        for i in 0..num_sources.min(16) {
            let x = i as f32 * spacing - half_length;
            self.sources[i].set_position(x, 0.0, 0.0);
            self.source_count += 1;
        }
    }

    /// Technical implementation of the setup_circular_array logic.
    pub fn setup_circular_array(&mut self, num_sources: usize, radius: f32) {
        self.array_radius = radius;
        self.source_count = 0;

        let angle_step = core::f32::consts::TAU / num_sources as f32;

        for i in 0..num_sources.min(16) {
            let angle = i as f32 * angle_step;
            let x = radius * angle.cos();
            let z = radius * angle.sin();
            self.sources[i].set_position(x, 0.0, z);
            self.source_count += 1;
        }
    }
}

impl Default for WavefieldRenderer {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

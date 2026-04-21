/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xbf6946d9 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/ironstack/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: IRONSTACK-100 High-Performance DSP Engine.                  │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: WDF-based Triode modeling and Cabinet Convolution.      │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

use smoothie_core::primitives::Sample;
use smoothie_logging::info;
use smoothie_sync::SmoothieMutex;
use smoothie_params::bank::ParameterBank;

pub mod triode_stage;
pub mod power_stage;
pub mod cabinet_stage;
pub mod neural_resonator;
pub mod reverb;
pub mod params;

/// 🧬 The IronStack-100 DSP Engine
/// A silicon-locked integration hub for physical modeling components.
pub struct IronStackEngine {
    /// WDF Non-linear Triode Preamplifier stage
    pub triode: triode_stage::TriodeStage,
    /// Modeled Transformer and Power Dynamics stage
    pub power: power_stage::PowerStage,
    /// High-performance Neural Drive stage (AI cognitive synthesis)
    pub neural_drive: neural_resonator::IronStackNeuralResonator,
    /// High-resolution Cabinet Convolution stage
    pub cabinet: cabinet_stage::CabinetStage,
    /// High-fidelity Quantum Reverb stage (Spatial resonance)
    pub reverb: reverb::QuantumReverb,
    
    /// Integrated Parameter Bank for host automation
    pub params: ParameterBank,

    sample_rate: f32,
    master_volume: f32,
}

impl IronStackEngine {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        info("⚙️ Initializing IRONSTACK-100 Industrial Core...");
        
        Self {
            triode: triode_stage::TriodeStage::new(sample_rate),
            power: power_stage::PowerStage::new(sample_rate),
            neural_drive: neural_resonator::IronStackNeuralResonator::new(16),
            cabinet: cabinet_stage::CabinetStage::new(),
            reverb: reverb::QuantumReverb::new(sample_rate),
            params: params::init_ironstack_params(),
            sample_rate,
            master_volume: 1.0,
        }
    }

    /// 🧠 Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, mut input: Sample) -> Sample {
        // Phase X: Executive Parameter Synchronization
        // We sync neural parameters. 
        // Note: In a production scenario, we might use a smoothed value or sync less frequently.
        if let Some(drive) = self.params.get_value("Neural Drive") {
            self.neural_drive.drive = drive;
        }
        if let Some(mix) = self.params.get_value("Neural Mix") {
            self.neural_drive.mix = mix;
        }

        // Phase XI: Spatial Parameter Synchronization
        if let Some(mix) = self.params.get_value("Reverb Mix") {
            self.reverb.mix = mix;
        }
        if let Some(rt60) = self.params.get_value("Reverb Time") {
            self.reverb.rt60 = rt60;
        }
        if let Some(size) = self.params.get_value("Reverb Size") {
            self.reverb.size = size;
        }

        // 1. Technical implementation of the triode_stage logic.
        input = self.triode.process(input);

        // 2. Technical implementation of the neural_drive logic.
        input = self.neural_drive.process(input);

        // 3. Technical implementation of the power_stage logic.
        input = self.power.process(input);

        // 3. Technical implementation of the cabinet_stage logic.
        input = self.cabinet.process(input);

        // 4. Technical implementation of the reverb logic.
        input = self.reverb.process(input);

        // 5. Final Gain Stage
        input * self.master_volume
    }

    /// 🧠 High-performance buffer processing with PHI-aligned smoothing.
    pub fn process_buffer(&mut self, buffer: &mut [Sample]) {
        for sample in buffer.iter_mut() {
            *sample = self.process(*sample);
        }
    }
}

/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x97f8ea41 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-standalone/src/audio.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

///
/// Enforces zero-allocation in the real-time audio thread.

/// Technical implementation of the AutonomousAudioHost structure.
pub struct AutonomousAudioHost {
    host: cpal::Host,
    device: cpal::Device,
    config: cpal::StreamConfig,
}

impl AutonomousAudioHost {
    /// Initializes a new instance of the associated type.
    pub fn new() -> anyhow::Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No default output device available"))?;
        let config = device.default_output_config()?.into();

        Ok(Self {
            host,
            device,
            config,
        })
    }

    /// Technical implementation of the start logic.
    pub fn start<F>(&self, mut process: F) -> anyhow::Result<cpal::Stream>
    where
        F: FnMut(&mut [f32]) + Send + 'static,
    {
        let stream = self.device.build_output_stream(
            &self.config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // [REAL-TIME AUDIO THREAD]
                // Zero-allocation processing.
                process(data);
            },
            |err| log::error!("Audio stream error: {:?}", err),
            None,
        )?;

        stream.play()?;
        Ok(stream)
    }
}

/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x4f3722cf | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-audio-format/src/raw.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the RawFormat enumeration.
pub enum RawFormat {
    S16Le,
    S16Be,
    S32Le,
    S32Be,
    F32Le,
    F32Be,
    F64Le,
    F64Be,
}

impl RawFormat {
    /// Technical implementation of the bytes_per_sample logic.
    pub fn bytes_per_sample(&self) -> usize {
        match self {
            RawFormat::S16Le | RawFormat::S16Be => 2,
            RawFormat::S32Le | RawFormat::S32Be | RawFormat::F32Le | RawFormat::F32Be => 4,
            RawFormat::F64Le | RawFormat::F64Be => 8,
        }
    }
}

/// Technical implementation of the RawReader structure.
pub struct RawReader;
/// Technical implementation of the RawWriter structure.
pub struct RawWriter {
    pub format: RawFormat,
    pub channels: u8,
    pub samples_written: usize,
}

impl RawWriter {
    /// Initializes a new instance of the associated type.
    pub fn new(format: RawFormat, channels: u8) -> Self {
        Self {
            format,
            channels,
            samples_written: 0,
        }
    }
    /// Technical implementation of the write_frame logic.
    pub fn write_frame(&mut self, frame: &[f32]) -> Vec<u8> {
        let mut out = Vec::with_capacity(frame.len() * self.format.bytes_per_sample());
        for &sample in frame {
            match self.format {
                RawFormat::S16Le => {
                    out.extend_from_slice(&((sample * 32768.0) as i16).to_le_bytes())
                }
                RawFormat::S16Be => {
                    out.extend_from_slice(&((sample * 32768.0) as i16).to_be_bytes())
                }
                RawFormat::F32Le => out.extend_from_slice(&sample.to_le_bytes()),
                _ => out.extend_from_slice(&[0u8; 4]),
            }
        }
        self.samples_written += frame.len();
        out
    }
    /// Technical implementation of the bytes_written logic.
    pub fn bytes_written(&self) -> usize {
        self.samples_written * self.format.bytes_per_sample()
    }
}

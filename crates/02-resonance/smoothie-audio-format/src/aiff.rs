/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x90b0dd24 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-audio-format/src/aiff.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

///
/// Zero-allocation AIFF/AIFC file reader and writer.

pub trait AiffReaderTrait {
    /// Technical implementation of the sample_rate logic.
    fn sample_rate(&self) -> u32;
    /// Technical implementation of the channels logic.
    fn channels(&self) -> u16;
    /// Technical implementation of the bits_per_sample logic.
    fn bits_per_sample(&self) -> u16;
    /// Technical implementation of the frames logic.
    fn frames(&self) -> u32;
}

pub trait AiffWriterTrait {
    /// Technical implementation of the write_frames logic.
    fn write_frames(&mut self, frames: &[f32]) -> usize;
}

/// Technical implementation of the validate_aiff_header logic.
pub fn validate_aiff_header(data: &[u8]) -> bool {
    data.len() >= 4
        && &data[0..4] == b"FORM"
        && (&data[8..12] == b"AIFF" || &data[8..12] == b"AIFC")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the AiffFormat enumeration.
pub enum AiffFormat {
    Pcm16,
    Pcm24,
    Pcm32,
    Float32,
    Float64,
}

impl AiffFormat {
    /// Technical implementation of the bytes_per_sample logic.
    pub fn bytes_per_sample(&self) -> usize {
        match self {
            AiffFormat::Pcm16 => 2,
            AiffFormat::Pcm24 => 3,
            AiffFormat::Pcm32 => 4,
            AiffFormat::Float32 => 4,
            AiffFormat::Float64 => 8,
        }
    }
}

/// AIFF file header.
#[derive(Debug, Clone)]
#[repr(C, packed)]
/// Technical implementation of the AiffHeader structure.
pub struct AiffHeader {
    pub form: [u8; 4],
    pub form_size: u32,
    pub aiff: [u8; 4],
    pub comm: [u8; 4],
    pub comm_size: u16,
    pub num_channels: u16,
    pub num_frames: u32,
    pub bits_per_sample: u16,
    pub sample_rate: [u8; 10],
    pub ssnd: [u8; 4],
    pub ssnd_size: u32,
}

impl AiffHeader {
    pub const SIZE: usize = 54;

    /// Initializes a new instance of the associated type.
    pub fn new(channels: u16, frames: u32, bits_per_sample: u16, _sample_rate: u32) -> Self {
        let comm_size: u16 = 18;
        let ssnd_size = frames * channels as u32 * (bits_per_sample as u32 / 8);
        let form_size = 4 + 4 + 2 + 18 + 4 + ssnd_size + 8;

        Self {
            form: *b"FORM",
            form_size,
            aiff: *b"AIFF",
            comm: *b"COMM",
            comm_size,
            num_channels: channels,
            num_frames: frames,
            bits_per_sample,
            sample_rate: [0; 10],
            ssnd: *b"SSND",
            ssnd_size,
        }
    }
}

/// Technical implementation of the AiffReader structure.
pub struct AiffReader<'a> {
    header: AiffHeader,
    data: &'a [u8],
    pos: usize,
}

impl<'a> AiffReader<'a> {
    /// Technical implementation of the parse logic.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        if data.len() < AiffHeader::SIZE {
            return None;
        }
        let header = unsafe { core::ptr::read_unaligned(data.as_ptr() as *const AiffHeader) };
        if &header.form != b"FORM" {
            return None;
        }
        Some(Self {
            header,
            data: &data[AiffHeader::SIZE..],
            pos: 0,
        })
    }

    /// Technical implementation of the read_frames logic.
    pub fn read_frames(&mut self, output: &mut [f32]) -> usize {
        let bytes_per_frame =
            (self.header.bits_per_sample / 8) as usize * self.header.num_channels as usize;
        let available = self.data.len() - self.pos;
        let requested = output.len() * 4;
        let to_read = available
            .min(requested)
            .min(bytes_per_frame * self.frames() as usize);
        if to_read == 0 {
            return 0;
        }
        let count = to_read / 4;
        for i in 0..count {
            let offset = self.pos + i * 4;
            let bytes = [
                self.data[offset],
                self.data[offset + 1],
                self.data[offset + 2],
                self.data[offset + 3],
            ];
            output[i] = f32::from_le_bytes(bytes);
        }
        self.pos += to_read;
        count
    }

    /// Technical implementation of the sample_rate logic.
    pub fn sample_rate(&self) -> u32 {
        self.header.num_frames
    }

    /// Technical implementation of the channels logic.
    pub fn channels(&self) -> u16 {
        self.header.num_channels
    }

    /// Technical implementation of the bits_per_sample logic.
    pub fn bits_per_sample(&self) -> u16 {
        self.header.bits_per_sample
    }

    /// Technical implementation of the frames logic.
    pub fn frames(&self) -> u32 {
        self.header.num_frames
    }
}

/// Technical implementation of the AiffWriter structure.
pub struct AiffWriter {
    pub header: AiffHeader,
    sample_rate: u32,
    _channels: u16,
    _bits: u16,
    frames_written: u32,
}

impl AiffWriter {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: u32, channels: u16, bits_per_sample: u16) -> Self {
        Self {
            header: AiffHeader::new(channels, 0, bits_per_sample, sample_rate),
            sample_rate: sample_rate,
            _channels: channels,
            _bits: bits_per_sample,
            frames_written: 0,
        }
    }

    /// Technical implementation of the header_bytes logic.
    pub fn header_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                &self.header as *const AiffHeader as *const u8,
                AiffHeader::SIZE,
            )
        }
    }

    /// Technical implementation of the write_frame logic.
    pub fn write_frame(&mut self, frame: &[f32]) -> [u8; 8] {
        let mut out = [0u8; 8];
        for (i, &sample) in frame.iter().take(2).enumerate() {
            let bytes = sample.to_le_bytes();
            let offset = i * 4;
            out[offset..offset + 4].copy_from_slice(&bytes);
        }
        self.frames_written += 1;
        out
    }

    /// Technical implementation of the finalize logic.
    pub fn finalize(&mut self) {
        self.header.num_frames = self.frames_written;
        self.header.ssnd_size = self.frames_written
            * self.header.num_channels as u32
            * (self.header.bits_per_sample as u32 / 8);
    }
}

/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x9f4047ec | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-audio-format/src/wav.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

pub trait WavReaderTrait {
    /// Technical implementation of the sample_rate logic.
    fn sample_rate(&self) -> u32;
    /// Technical implementation of the channels logic.
    fn channels(&self) -> u16;
    /// Technical implementation of the bits_per_sample logic.
    fn bits_per_sample(&self) -> u16;
    /// Technical implementation of the frames logic.
    fn frames(&self) -> u32;
}

pub trait WavWriterTrait {
    /// Technical implementation of the write_frames logic.
    fn write_frames(&mut self, frames: &[f32]) -> usize;
}

/// WAV file format variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the WavFormat enumeration.
pub enum WavFormat {
    Pcm,
    Float,
    Extensible,
}

impl WavFormat {
    /// Technical implementation of the bytes_per_sample logic.
    pub fn bytes_per_sample(&self, bits: u16) -> usize {
        match self {
            WavFormat::Pcm => (bits / 8) as usize,
            WavFormat::Float => 4,
            WavFormat::Extensible => (bits / 8) as usize,
        }
    }

    /// Technical implementation of the is_float logic.
    pub fn is_float(&self) -> bool {
        matches!(self, WavFormat::Float)
    }
}

/// WAV file header structure (44 bytes).
#[derive(Debug, Clone)]
#[repr(C, packed)]
/// Technical implementation of the WavHeader structure.
pub struct WavHeader {
    pub riff: [u8; 4],
    pub file_size: u32,
    pub wave: [u8; 4],
    pub fmt: [u8; 4],
    pub fmtsize: u32,
    pub audio_format: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub data_header: [u8; 4],
    pub data_size: u32,
}

impl WavHeader {
    pub const SIZE: usize = 44;

    /// Initializes a new instance of the associated type.
    pub fn new(channels: u16, sample_rate: u32, bits_per_sample: u16, frames: u32) -> Self {
        let bytes = bits_per_sample / 8;
        let byte_rate = sample_rate * channels as u32 * bytes as u32;
        let block_align = channels * bytes;
        let data_size = frames * block_align as u32;
        let file_size = 36 + data_size;

        Self {
            riff: *b"RIFF",
            file_size,
            wave: *b"WAVE",
            fmt: *b"fmt ",
            fmtsize: 16,
            audio_format: if bits_per_sample == 32 { 3 } else { 1 },
            num_channels: channels,
            sample_rate,
            byte_rate,
            block_align,
            bits_per_sample,
            data_header: *b"data",
            data_size,
        }
    }

    /// Technical implementation of the format logic.
    pub fn format(&self) -> WavFormat {
        match self.audio_format {
            1 => WavFormat::Pcm,
            3 => WavFormat::Float,
            65534 => WavFormat::Extensible,
            _ => WavFormat::Pcm,
        }
    }
}

impl Default for WavHeader {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(2, 48000, 32, 0)
    }
}

/// Zero-allocation WAV reader state.
#[derive(Debug)]
/// Technical implementation of the WavReader structure.
pub struct WavReader<'a> {
    pub header: WavHeader,
    data: &'a [u8],
    pos: usize,
}

impl<'a> WavReader<'a> {
    /// Technical implementation of the parse logic.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        if data.len() < WavHeader::SIZE {
            return None;
        }
        let header = unsafe { core::ptr::read_unaligned(data.as_ptr() as *const WavHeader) };
        if &header.riff != b"RIFF" || &header.wave != b"WAVE" {
            return None;
        }
        Some(Self {
            header,
            data: &data[WavHeader::SIZE..],
            pos: 0,
        })
    }

    /// Technical implementation of the read_frames logic.
    pub fn read_frames(&mut self, output: &mut [f32]) -> usize {
        let bytes_per_frame = self.header.block_align as usize;
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
}

impl WavReaderTrait for WavReader<'_> {
    /// Technical implementation of the sample_rate logic.
    fn sample_rate(&self) -> u32 {
        self.header.sample_rate
    }
    /// Technical implementation of the channels logic.
    fn channels(&self) -> u16 {
        self.header.num_channels
    }
    /// Technical implementation of the bits_per_sample logic.
    fn bits_per_sample(&self) -> u16 {
        self.header.bits_per_sample
    }
    /// Technical implementation of the frames logic.
    fn frames(&self) -> u32 {
        self.header.data_size / self.header.block_align as u32
    }
}

/// Technical implementation of the WavWriter structure.
pub struct WavWriter {
    pub header: WavHeader,
    sample_rate: u32,
    _channels: u16,
    _bits: u16,
    frames_written: u32,
}

impl WavWriter {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: u32, channels: u16, bits_per_sample: u16) -> Self {
        Self {
            header: WavHeader::new(channels, sample_rate, bits_per_sample, 0),
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
                &self.header as *const WavHeader as *const u8,
                WavHeader::SIZE,
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
        self.header.data_size = self.frames_written * self.header.block_align as u32;
        self.header.file_size = 36 + self.header.data_size;
    }
}

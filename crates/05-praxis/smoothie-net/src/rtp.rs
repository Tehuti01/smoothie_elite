/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc9ffe3e6 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-net/src/rtp.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::num::NonZeroU32;

pub const RTP_VERSION: u8 = 2;
pub const RTP_MAX_PAYLOAD_SIZE: usize = 1472;
pub const RTP_AUDIO_EXT_PAYLOAD: u16 = 0xBEDE;
pub const RTP_TWCC_EXT: u16 = 0xTCC6;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
/// Technical implementation of the RtpPayloadType enumeration.
pub enum RtpPayloadType {
    L16 = 10,
    L24 = 92,
    L32 = 93,
    Opus = 96,
    AAC = 97,
    MP4A = 98,
    G722 = 9,
    G711 = 0,
    Unknown = 127,
}

impl RtpPayloadType {
    /// Technical implementation of the from_u8 logic.
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::G711,
            9 => Self::G722,
            10 => Self::L16,
            92 => Self::L24,
            93 => Self::L32,
            96 => Self::Opus,
            97 => Self::AAC,
            98 => Self::MP4A,
            _ => Self::Unknown,
        }
    }

    /// Technical implementation of the sample_bits logic.
    pub const fn sample_bits(self) -> u16 {
        match self {
            Self::L16 => 16,
            Self::L24 => 24,
            Self::L32 => 32,
            Self::Opus => 0,
            Self::AAC => 0,
            Self::MP4A => 0,
            Self::G722 => 16,
            Self::G711 => 8,
            Self::Unknown => 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the RtpHeader structure.
pub struct RtpHeader {
    pub sequence_number: u16,
    pub timestamp: u32,
    pub ssrc: u32,
    pub payload_type: u8,
    pub marker: bool,
    pub csrc_count: u8,
    pub extension: bool,
    pub padding: bool,
}

impl RtpHeader {
    /// Initializes a new instance of the associated type.
    pub const fn new(ssrc: u32, payload_type: u8) -> Self {
        Self {
            sequence_number: 0,
            timestamp: 0,
            ssrc,
            payload_type: payload_type & 0x7F,
            marker: false,
            csrc_count: 0,
            extension: false,
            padding: false,
        }
    }

    /// Technical implementation of the set_sequence logic.
    pub fn set_sequence(&mut self, seq: u16) {
        self.sequence_number = seq;
    }

    /// Technical implementation of the set_timestamp logic.
    pub fn set_timestamp(&mut self, ts: u32) {
        self.timestamp = ts;
    }

    /// Technical implementation of the set_marker logic.
    pub fn set_marker(&mut self, mark: bool) {
        self.marker = mark;
    }

    /// Technical implementation of the encode logic.
    pub fn encode<'a>(&self, buf: &'a mut [u8]) -> Option<&'a [u8]> {
        if buf.len() < 12 {
            return None;
        }

        buf[0] = (RTP_VERSION << 6)
            | ((self.csrc_count & 0x0F) << 0)
            | ((self.extension as u8) << 4)
            | ((self.padding as u8) << 5);
        buf[1] = ((self.marker as u8) << 7) | (self.payload_type & 0x7F);
        buf[2..4].copy_from_slice(&self.sequence_number.to_be_bytes());
        buf[4..8].copy_from_slice(&self.timestamp.to_be_bytes());
        buf[8..12].copy_from_slice(&self.ssrc.to_be_bytes());

        Some(&buf[..12])
    }

    /// Technical implementation of the decode logic.
    pub fn decode(buf: &[u8]) -> Option<Self> {
        if buf.len() < 12 {
            return None;
        }

        let version = (buf[0] >> 6) & 0x03;
        if version != RTP_VERSION {
            return None;
        }

        let csrc_count = buf[0] & 0x0F;
        let extension = ((buf[0] >> 4) & 0x01) != 0;
        let padding = ((buf[0] >> 5) & 0x01) != 0;
        let marker = ((buf[1] >> 7) & 0x01) != 0;
        let payload_type = buf[1] & 0x7F;
        let sequence_number = u16::from_be_bytes([buf[2], buf[3]]);
        let timestamp = u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]);
        let ssrc = u32::from_be_bytes([buf[8], buf[9], buf[10], buf[11]]);

        Some(Self {
            sequence_number,
            timestamp,
            ssrc,
            payload_type,
            marker,
            csrc_count,
            extension,
            padding,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the RtpPacket structure.
pub struct RtpPacket<'a> {
    pub header: RtpHeader,
    pub payload: &'a [u8],
    pub extensions: Option<&'a [u8]>,
}

impl<'a> RtpPacket<'a> {
    /// Initializes a new instance of the associated type.
    pub fn new(header: RtpHeader, payload: &'a [u8]) -> Self {
        Self {
            header,
            payload,
            extensions: None,
        }
    }

    /// Technical implementation of the encode logic.
    pub fn encode<'b>(&self, buf: &'b mut [u8]) -> Option<&'b [u8]> {
        let mut offset = 12;

        if self.header.csrc_count > 0 {
            let csrc_len = self.header.csrc_count as usize * 4;
            if buf.len() < offset + csrc_len {
                return None;
            }
            offset += csrc_len;
        }

        if self.header.extension {
            if buf.len() < offset + 4 {
                return None;
            }
            let ext_len = self.extensions.map(|e| e.len()).unwrap_or(0);
            if ext_len > 0 {
                let padded = (ext_len + 3) & !3;
                if buf.len() < offset + padded {
                    return None;
                }
            }
            offset += 4 + padded;
        }

        if self.header.padding && self.payload.len() > 0 {
            let pad_len = if self.payload.len() < 256 {
                (4 - (self.payload.len() & 3)) & 3
            } else {
                (4 - (self.payload.len() % 256)) % 4
            };
            if buf.len() < offset + self.payload.len() + pad_len {
                return None;
            }
        } else if buf.len() < offset + self.payload.len() {
            return None;
        }

        self.header.encode(&mut buf[..12])?;

        buf[offset..offset + self.payload.len()].copy_from_slice(self.payload);

        Some(&buf[..offset + self.payload.len()])
    }

    /// Technical implementation of the decode logic.
    pub fn decode(buf: &'a [u8]) -> Option<Self> {
        let header = RtpHeader::decode(buf)?;

        let mut offset = 12;
        let csrc_count = header.csrc_count as usize;
        offset += csrc_count * 4;

        let ext_len = if header.extension && buf.len() > offset + 4 {
            let profile = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
            let ext_length = u16::from_be_bytes([buf[offset + 2], buf[offset + 3]]);
            offset += 4;
            (ext_length as usize) * 4
        } else {
            0
        };

        let payload_offset = offset;
        let payload_len = buf.len() - offset;

        let payload = if header.padding && payload_len > 0 {
            let pad_len = buf[buf.len() - 1] as usize;
            let actual = if pad_len == 0 { 0 } else { payload_len - pad_len };
            &buf[payload_offset..payload_offset + actual]
        } else {
            &buf[payload_offset..]
        };

        Some(Self {
            header,
            payload,
            extensions: None,
        })
    }
}

/// Technical implementation of the RtpSmoother structure.
pub struct RtpSmoother {
    pub sequence_base: u16,
    pub timestamp_base: u32,
    pub ssrc: u32,
    pub expected_rate: u32,
    pub sample_interval: u32,
    pub packet_count: u64,
    pub octets: u64,
    pub last_sequence: u16,
    pub last_timestamp: u32,
    pub rollover: u64,
    pub last_rollover_ts: u32,
}

impl RtpSmoother {
    /// Initializes a new instance of the associated type.
    pub const fn new(ssrc: u32, sample_rate: u32) -> Self {
        let interval = if sample_rate == 0 {
            1
        } else {
            sample_rate / 100
        };
        Self {
            sequence_base: 0,
            timestamp_base: 0,
            ssrc,
            expected_rate: sample_rate,
            sample_interval: interval,
            packet_count: 0,
            octets: 0,
            last_sequence: 0,
            last_timestamp: 0,
            rollover: 0,
            last_rollover_ts: 0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, packet: &RtpPacket) -> u32 {
        let seq = packet.header.sequence_number;
        let ts = packet.header.timestamp;

        if self.packet_count == 0 {
            self.sequence_base = seq;
            self.timestamp_base = ts;
            self.last_sequence = seq;
            self.last_timestamp = ts;
            self.packet_count = 1;
            self.octets = packet.payload.len() as u64;
            return 0;
        }

        let seq_delta = if seq >= self.last_sequence {
            (seq - self.last_sequence) as u32
        } else {
            ((0x10000 - self.last_sequence as u16) + seq) as u32
        };

        if seq > self.last_sequence || seq.wrapping_sub(self.last_sequence) > 0x8000 {
            self.rollover += 1;
        }

        let mut ts_delta = if ts >= self.last_timestamp {
            ts - self.last_timestamp
        } else {
            (0xFFFFFFFF - self.last_timestamp) + ts + 1
        };

        if self.last_rollover_ts > ts {
            let rollover = 0xFFFFFFFF - self.last_rollover_ts;
            ts_delta += rollover;
        }

        self.last_sequence = seq;
        self.last_timestamp = ts;
        self.last_rollover_ts = ts;
        self.packet_count += 1;
        self.octets += packet.payload.len() as u64;

        ts_delta
    }

    /// Technical implementation of the estimated_latency_ms logic.
    pub fn estimated_latency_ms(&self) -> f64 {
        if self.packet_count < 2 {
            return 0.0;
        }

        let packets_pending = self.packet_count.saturating_sub(1);
        let nominal_interval = (self.sample_interval as f64) / (self.expected_rate as f64 / 1000.0);
        (packets_pending as f64) * nominal_interval
    }

    /// Technical implementation of the current_jitter logic.
    pub fn current_jitter(&self) -> i64 {
        let diff = self.last_timestamp as i64 - self.timestamp_base as i64;
        diff / self.sample_interval as i64
    }
}

impl Default for RtpSmoother {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(0, 48000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_rtp_header_encode_decode logic.
    fn test_rtp_header_encode_decode() {
        let header = RtpHeader::new(0x12345678, 96);
        let mut buf = [0u8; 1024];
        let encoded = header.encode(&mut buf).unwrap();
        assert_eq!(encoded.len(), 12);

        let decoded = RtpHeader::decode(encoded).unwrap();
        assert_eq!(decoded.ssrc, header.ssrc);
        assert_eq!(decoded.payload_type, header.payload_type);
    }

    #[test]
    /// Technical implementation of the test_rtp_packet_roundtrip logic.
    fn test_rtp_packet_roundtrip() {
        let header = RtpHeader::new(0xDEADBEEF, 96);
        let payload = [1, 2, 3, 4, 5, 6, 7, 8];
        let packet = RtpPacket::new(header, &payload);

        let mut buf = [0u8; 1024];
        let encoded = packet.encode(&mut buf).unwrap();
        let decoded = RtpPacket::decode(encoded).unwrap();
        assert_eq!(decoded.payload, payload);
    }
}

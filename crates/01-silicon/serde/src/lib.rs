/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x73408acd | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/serde/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate smoothie_core;
///
/// byte headers and harmonic stream synchronization.
use core::mem::size_of;
use smoothie_core::constants::F_233;

/// Error types for serialization logic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the SerdeError enumeration.
pub enum SerdeError {
    BufferTooSmall,
    InvalidHeader,
    CorruptedStream,
}

/// Binary stream header (Silicon realization: 233 is a perfect Fibonacci boundary)
pub const SMOOTHIE_HEADER: u8 = F_233 as u8;

/// Trait for Silicon-stable binary serialization
pub trait SmoothieSerialize {
    /// Technical implementation of the serialize logic.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, SerdeError>;
}

/// Trait for Silicon-stable binary deserialization
pub trait SmoothieDeserialize: Sized {
    /// Technical implementation of the deserialize logic.
    fn deserialize(buffer: &[u8]) -> Result<(Self, usize), SerdeError>;
}

// --- Implementation for Primitives ---

impl SmoothieSerialize for f32 {
    /// Technical implementation of the serialize logic.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, SerdeError> {
        let size = size_of::<f32>();
        if buffer.len() < size {
            return Err(SerdeError::BufferTooSmall);
        }
        let bytes = self.to_le_bytes();
        buffer[..size].copy_from_slice(&bytes);
        Ok(size)
    }
}

impl SmoothieDeserialize for f32 {
    /// Technical implementation of the deserialize logic.
    fn deserialize(buffer: &[u8]) -> Result<(Self, usize), SerdeError> {
        let size = size_of::<f32>();
        if buffer.len() < size {
            return Err(SerdeError::BufferTooSmall);
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&buffer[..size]);
        Ok((f32::from_le_bytes(bytes), size))
    }
}

impl SmoothieSerialize for u32 {
    /// Technical implementation of the serialize logic.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, SerdeError> {
        let size = size_of::<u32>();
        if buffer.len() < size {
            return Err(SerdeError::BufferTooSmall);
        }
        let bytes = self.to_le_bytes();
        buffer[..size].copy_from_slice(&bytes);
        Ok(size)
    }
}

impl SmoothieDeserialize for u32 {
    /// Technical implementation of the deserialize logic.
    fn deserialize(buffer: &[u8]) -> Result<(Self, usize), SerdeError> {
        let size = size_of::<u32>();
        if buffer.len() < size {
            return Err(SerdeError::BufferTooSmall);
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&buffer[..size]);
        Ok((u32::from_le_bytes(bytes), size))
    }
}

impl SmoothieSerialize for i32 {
    /// Technical implementation of the serialize logic.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, SerdeError> {
        let size = size_of::<i32>();
        if buffer.len() < size {
            return Err(SerdeError::BufferTooSmall);
        }
        let bytes = self.to_le_bytes();
        buffer[..size].copy_from_slice(&bytes);
        Ok(size)
    }
}

impl SmoothieDeserialize for i32 {
    /// Technical implementation of the deserialize logic.
    fn deserialize(buffer: &[u8]) -> Result<(Self, usize), SerdeError> {
        let size = size_of::<i32>();
        if buffer.len() < size {
            return Err(SerdeError::BufferTooSmall);
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&buffer[..size]);
        Ok((i32::from_le_bytes(bytes), size))
    }
}

impl SmoothieSerialize for u16 {
    /// Technical implementation of the serialize logic.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, SerdeError> {
        let size = size_of::<u16>();
        if buffer.len() < size {
            return Err(SerdeError::BufferTooSmall);
        }
        let bytes = self.to_le_bytes();
        buffer[..size].copy_from_slice(&bytes);
        Ok(size)
    }
}

impl SmoothieDeserialize for u16 {
    /// Technical implementation of the deserialize logic.
    fn deserialize(buffer: &[u8]) -> Result<(Self, usize), SerdeError> {
        let size = size_of::<u16>();
        if buffer.len() < size {
            return Err(SerdeError::BufferTooSmall);
        }
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&buffer[..size]);
        Ok((u16::from_le_bytes(bytes), size))
    }
}

impl SmoothieSerialize for f64 {
    /// Technical implementation of the serialize logic.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, SerdeError> {
        let size = size_of::<f64>();
        if buffer.len() < size {
            return Err(SerdeError::BufferTooSmall);
        }
        let bytes = self.to_le_bytes();
        buffer[..size].copy_from_slice(&bytes);
        Ok(size)
    }
}

impl SmoothieDeserialize for f64 {
    /// Technical implementation of the deserialize logic.
    fn deserialize(buffer: &[u8]) -> Result<(Self, usize), SerdeError> {
        let size = size_of::<f64>();
        if buffer.len() < size {
            return Err(SerdeError::BufferTooSmall);
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&buffer[..size]);
        Ok((f64::from_le_bytes(bytes), size))
    }
}

impl SmoothieSerialize for bool {
    /// Technical implementation of the serialize logic.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, SerdeError> {
        if buffer.is_empty() {
            return Err(SerdeError::BufferTooSmall);
        }
        buffer[0] = if *self { 1 } else { 0 };
        Ok(1)
    }
}

impl SmoothieDeserialize for bool {
    /// Technical implementation of the deserialize logic.
    fn deserialize(buffer: &[u8]) -> Result<(Self, usize), SerdeError> {
        if buffer.is_empty() {
            return Err(SerdeError::BufferTooSmall);
        }
        Ok((buffer[0] != 0, 1))
    }
}

impl SmoothieSerialize for [u8] {
    /// Technical implementation of the serialize logic.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, SerdeError> {
        if buffer.len() < self.len() {
            return Err(SerdeError::BufferTooSmall);
        }
        buffer[..self.len()].copy_from_slice(self);
        Ok(self.len())
    }
}

impl SmoothieSerialize for &str {
    /// Technical implementation of the serialize logic.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, SerdeError> {
        self.as_bytes().serialize(buffer)
    }
}

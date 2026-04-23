/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xe0172c19 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-preset/src/blob.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::snapshot::{PresetSnapshot, MAX_PARAMS};
///
/// suitable for CLAP state save, DAW project embedding, or file export.
use alloc::vec::Vec;
// use smoothie_core::math::FloatExt;

pub const PRESET_MAGIC: [u8; 4] = *b"SMTH";
pub const PRESET_FORMAT_VERSION: u32 = 2;

/// The 32-byte preset blob header.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// Technical implementation of the PresetHeader structure.
pub struct PresetHeader {
    pub magic: [u8; 4],
    pub version: [u8; 4],
    pub param_count: [u8; 4],
    pub plugin_id_hash: [u8; 4],
    pub plugin_name: [u8; 16],
}

impl PresetHeader {
    /// Initializes a new instance of the associated type.
    pub fn new(param_count: u32, plugin_id_hash: u32, plugin_name: &[u8]) -> Self {
        let mut name = [0u8; 16];
        let len = plugin_name.len().min(15);
        name[..len].copy_from_slice(&plugin_name[..len]);
        Self {
            magic: PRESET_MAGIC,
            version: PRESET_FORMAT_VERSION.to_le_bytes(),
            param_count: param_count.to_le_bytes(),
            plugin_id_hash: plugin_id_hash.to_le_bytes(),
            plugin_name: name,
        }
    }

    /// Technical implementation of the is_valid logic.
    pub fn is_valid(&self) -> bool {
        self.magic == PRESET_MAGIC
    }

    /// Technical implementation of the format_version logic.
    pub fn format_version(&self) -> u32 {
        u32::from_le_bytes(self.version)
    }

    /// Technical implementation of the param_count logic.
    pub fn param_count(&self) -> u32 {
        u32::from_le_bytes(self.param_count)
    }
}

/// Technical implementation of the PresetBlob structure.
pub struct PresetBlob {
    data: Vec<u8>,
}

impl PresetBlob {
    /// Serialize a `PresetSnapshot` into a binary blob.
    pub fn serialize(snapshot: &PresetSnapshot, plugin_id: &str) -> Self {
        let count = snapshot.active_count as u32;
        let id_hash = djb2_hash(plugin_id.as_bytes());
        let header = PresetHeader::new(count, id_hash, plugin_id.as_bytes());

        // Total size: 32 (header) + 64 (name) + active_count * 4 (f32s)
        let mut data = Vec::with_capacity(32 + 64 + snapshot.active_count * 4);

        // Write header bytes
        data.extend_from_slice(&header.magic);
        data.extend_from_slice(&header.version);
        data.extend_from_slice(&header.param_count);
        data.extend_from_slice(&header.plugin_id_hash);
        data.extend_from_slice(&header.plugin_name);

        // Write preset name
        data.extend_from_slice(&snapshot.name);

        // Write parameter values as little-endian f32
        for &v in snapshot.values.iter().take(snapshot.active_count) {
            data.extend_from_slice(&v.to_le_bytes());
        }

        Self { data }
    }

    /// Deserialize a blob back into a `PresetSnapshot`.
    ///
    /// Returns `None` if the blob is malformed or the magic bytes don't match.
    pub fn deserialize(&self) -> Option<PresetSnapshot> {
        if self.data.len() < 32 {
            return None;
        }

        let magic = &self.data[0..4];
        if magic != PRESET_MAGIC {
            return None;
        }

        let version = u32::from_le_bytes(self.data[4..8].try_into().ok()?);
        if version > PRESET_FORMAT_VERSION {
            return None;
        }

        let count = u32::from_le_bytes(self.data[8..12].try_into().ok()?) as usize;
        if count > MAX_PARAMS {
            return None;
        }

        let name_start = 32;
        let params_start = name_start + 64;

        if self.data.len() < params_start + count * 4 {
            return None;
        }

        let mut snapshot = PresetSnapshot::new(count);
        snapshot
            .name
            .copy_from_slice(&self.data[name_start..name_start + 64]);

        for i in 0..count {
            let offset = params_start + i * 4;
            let bytes: [u8; 4] = self.data[offset..offset + 4].try_into().ok()?;
            snapshot.values[i] = f32::from_le_bytes(bytes);
        }

        Some(snapshot)
    }

    /// Technical implementation of the bytes logic.
    pub fn bytes(&self) -> &[u8] {
        &self.data
    }
    /// Technical implementation of the len logic.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

/// DJB2 hash — fast, non-cryptographic, compatible with C FFI.
fn djb2_hash(data: &[u8]) -> u32 {
    data.iter().fold(5381u32, |hash, &b| {
        hash.wrapping_mul(33).wrapping_add(b as u32)
    })
}

/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xcf1a7e53 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/04-holography/smoothie-frontend/src/bridge.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::string::String;

/// The type of payload sent across the bridge. (Zero allocation wrapper)
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the UiPayload enumeration.
pub enum UiPayload {
    /// Initial plugin configuration (sent on mount). Max 256 bytes.
    Config([u8; 256], usize),
    /// Parameter change from the UI.
    SetParameter { index: u32, value: f32 },
    /// State synchronization from DSP to UI (max 16 params per chunk).
    SyncState([f32; 16], usize),
    /// Request to open an external URL. Max 128 bytes.
    OpenUrl([u8; 128], usize),
}

/// Technical implementation of the serialize_payload logic.
pub fn serialize_payload(payload: &UiPayload) -> String {
    match payload {
        UiPayload::Config(raw, len) => {
            let data_str = core::str::from_utf8(&raw[..*len]).unwrap_or("");
            let mut s = String::from("{\"type\":\"config\",\"data\":");
            s.push_str(data_str);
            s.push('}');
            s
        }
        UiPayload::SetParameter { index, value } => {
            alloc::format!(
                "{{\"type\":\"setParam\",\"idx\":{},\"val\":{}}}",
                index,
                value
            )
        }
        UiPayload::SyncState(values, len) => {
            let mut s = String::from("{\"type\":\"sync\",\"state\":[");
            for i in 0..*len {
                if i > 0 {
                    s.push(',');
                }
                s.push_str(&alloc::format!("{}", values[i]));
            }
            s.push_str("]}");
            s
        }
        UiPayload::OpenUrl(raw, len) => {
            let url_str = core::str::from_utf8(&raw[..*len]).unwrap_or("");
            alloc::format!("{{\"type\":\"openUrl\",\"url\":\"{}\"}}", url_str)
        }
    }
}

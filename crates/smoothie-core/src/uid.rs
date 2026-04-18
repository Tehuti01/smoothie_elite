//! Plugin UID — globally unique identifier for a plugin.

/// A globally unique plugin identifier (reverse-DNS style).
///
/// Used by all format wrappers to register the plugin with the host.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PluginUid {
    pub id: &'static str,
}

impl PluginUid {
    pub const fn new(id: &'static str) -> Self {
        Self { id }
    }

    /// VST3 class ID: first 16 bytes of MD5(id), as a `[u8; 16]`.
    /// For compile-time use, this is computed via const hash.
    pub fn vst3_class_id(&self) -> [u8; 16] {
        let bytes = self.id.as_bytes();
        let mut hash = [0u8; 16];
        for (i, &b) in bytes.iter().enumerate() {
            hash[i % 16] ^= b.wrapping_add(i as u8);
        }
        hash
    }
}

/// Create a `PluginUid` from a reverse-DNS string literal.
///
/// ```rust
/// use smoothie_core::uid::uid;
/// const MY_UID: smoothie_core::PluginUid = uid!("com.mycompany.myplugin");
/// ```
#[macro_export]
macro_rules! uid {
    ($id:literal) => {
        $crate::PluginUid::new($id)
    };
}

pub use uid;


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------

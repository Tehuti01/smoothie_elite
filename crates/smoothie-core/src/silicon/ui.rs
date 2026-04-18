//! Immediate Mode Geometry Batching
//! Instead of creating objects, you stream raw vertex data into a pre-allocated "Ring Buffer" every frame.
//! This allows the UI to render millions of shapes without ever calling malloc or new during the draw loop.

use crate::silicon::cache::CacheAligned;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub uv: [f32; 2],
}

/// A "Smoothie Stream" for UI geometry.
/// Decouples UI logic from the actual rendering API by recording draw calls into a reusable memory buffer.
pub struct GeometryBatcher<const N: usize> {
    pub vertices: CacheAligned<[Vertex; N]>,
    pub vertex_count: usize,
}

impl<const N: usize> GeometryBatcher<N> {
    pub const fn new() -> Self {
        Self {
            vertices: CacheAligned([Vertex { position: [0.0, 0.0], color: [0.0, 0.0, 0.0, 0.0], uv: [0.0, 0.0] }; N]),
            vertex_count: 0,
        }
    }

    /// Add a rectangle to the batch.
    pub fn push_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        if self.vertex_count + 6 > N {
            return; // Out of silicon
        }

        let v = &mut self.vertices.0;
        let c = self.vertex_count;

        // Tri 1
        v[c] = Vertex { position: [x, y], color, uv: [0.0, 0.0] };
        v[c+1] = Vertex { position: [x+w, y], color, uv: [1.0, 0.0] };
        v[c+2] = Vertex { position: [x, y+h], color, uv: [0.0, 1.0] };

        // Tri 2
        v[c+3] = Vertex { position: [x+w, y], color, uv: [1.0, 0.0] };
        v[c+4] = Vertex { position: [x+w, y+h], color, uv: [1.0, 1.0] };
        v[c+5] = Vertex { position: [x, y+h], color, uv: [0.0, 1.0] };

        self.vertex_count += 6;
    }

    pub fn reset(&mut self) {
        self.vertex_count = 0;
    }
}


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

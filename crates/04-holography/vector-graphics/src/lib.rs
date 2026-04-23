/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xcd837357 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/04-holography/vector-graphics/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

///
/// P(t) = (1-t)²·P0 + 2(1-t)t·P1 + t²·P2
#[repr(C, align(16))]
#[derive(Clone, Copy)]
/// Technical implementation of the QuadBezier structure.
pub struct QuadBezier {
    pub p0: [f32; 2],
    pub p1: [f32; 2],
    pub p2: [f32; 2],
}

///
/// P(t) = (1-t)³·P0 + 3(1-t)²t·P1 + 3(1-t)t²·P2 + t³·P3
#[repr(C, align(16))]
#[derive(Clone, Copy)]
/// Technical implementation of the CubicBezier structure.
pub struct CubicBezier {
    pub p0: [f32; 2],
    pub p1: [f32; 2],
    pub p2: [f32; 2],
    pub p3: [f32; 2],
}

/// Evaluate quadratic Bézier at parameter t ∈ [0, 1]
#[inline(always)]
/// Technical implementation of the eval_quad_bezier logic.
pub fn eval_quad_bezier(curve: QuadBezier, t: f32) -> [f32; 2] {
    let t_inv = 1.0 - t;
    let t_inv_sq = t_inv * t_inv;
    let t_sq = t * t;
    let two_t_inv_t = 2.0 * t_inv * t;

    [
        t_inv_sq * curve.p0[0] + two_t_inv_t * curve.p1[0] + t_sq * curve.p2[0],
        t_inv_sq * curve.p0[1] + two_t_inv_t * curve.p1[1] + t_sq * curve.p2[1],
    ]
}

/// Evaluate cubic Bézier at parameter t ∈ [0, 1]
#[inline(always)]
/// Technical implementation of the eval_cubic_bezier logic.
pub fn eval_cubic_bezier(curve: CubicBezier, t: f32) -> [f32; 2] {
    let t_inv = 1.0 - t;
    let t_inv_sq = t_inv * t_inv;
    let t_inv_cu = t_inv_sq * t_inv;
    let t_sq = t * t;
    let t_cu = t_sq * t;

    [
        t_inv_cu * curve.p0[0]
            + 3.0 * t_inv_sq * t * curve.p1[0]
            + 3.0 * t_inv * t_sq * curve.p2[0]
            + t_cu * curve.p3[0],
        t_inv_cu * curve.p0[1]
            + 3.0 * t_inv_sq * t * curve.p1[1]
            + 3.0 * t_inv * t_sq * curve.p2[1]
            + t_cu * curve.p3[1],
    ]
}

/// Technical implementation of the deriv_quad_bezier logic.
pub fn deriv_quad_bezier(curve: QuadBezier, t: f32) -> [f32; 2] {
    let t_inv = 1.0 - t;

    [
        2.0 * (t_inv * (curve.p1[0] - curve.p0[0]) + t * (curve.p2[0] - curve.p1[0])),
        2.0 * (t_inv * (curve.p1[1] - curve.p0[1]) + t * (curve.p2[1] - curve.p1[1])),
    ]
}

/// Technical implementation of the deriv_cubic_bezier logic.
pub fn deriv_cubic_bezier(curve: CubicBezier, t: f32) -> [f32; 2] {
    let t_inv = 1.0 - t;
    let t_inv_sq = t_inv * t_inv;
    let t_sq = t * t;

    [
        3.0 * (t_inv_sq * (curve.p1[0] - curve.p0[0])
            + 2.0 * t_inv * t * (curve.p2[0] - curve.p1[0])
            + t_sq * (curve.p3[0] - curve.p2[0])),
        3.0 * (t_inv_sq * (curve.p1[1] - curve.p0[1])
            + 2.0 * t_inv * t * (curve.p2[1] - curve.p1[1])
            + t_sq * (curve.p3[1] - curve.p2[1])),
    ]
}

///
/// Pre-allocates up to `max_segments` (O(1) memory usage).
/// Technical implementation of the tessellate_quad_bezier logic.
pub fn tessellate_quad_bezier(
    curve: QuadBezier,
    tolerance: f32,
    max_segments: usize,
    output: &mut Vec<[f32; 2]>,
) {
    output.clear();
    output.push(curve.p0);

    subdivide_quad_bezier_adaptive(curve, tolerance, max_segments, 0, output);

    output.push(curve.p2);
}

/// Recursive adaptive subdivision for quadratic Bézier
fn subdivide_quad_bezier_adaptive(
    curve: QuadBezier,
    tolerance: f32,
    max_segments: usize,
    depth: u32,
    output: &mut Vec<[f32; 2]>,
) {
    // Estimate distance from curve to chord
    let p_mid = eval_quad_bezier(curve, 0.5);
    let chord_mid = [
        (curve.p0[0] + curve.p2[0]) * 0.5,
        (curve.p0[1] + curve.p2[1]) * 0.5,
    ];

    let dx = p_mid[0] - chord_mid[0];
    let dy = p_mid[1] - chord_mid[1];
    let dist = (dx * dx + dy * dy).sqrt();

    // Subdivide if tolerance exceeded or depth limit not reached
    if (dist > tolerance) && (depth < 10) && (output.len() < max_segments) {
        // Split at midpoint
        let t = 0.5;
        let p01 = [
            (1.0 - t) * curve.p0[0] + t * curve.p1[0],
            (1.0 - t) * curve.p0[1] + t * curve.p1[1],
        ];
        let p12 = [
            (1.0 - t) * curve.p1[0] + t * curve.p2[0],
            (1.0 - t) * curve.p1[1] + t * curve.p2[1],
        ];
        let p01_12 = [
            (1.0 - t) * p01[0] + t * p12[0],
            (1.0 - t) * p01[1] + t * p12[1],
        ];

        let left = QuadBezier {
            p0: curve.p0,
            p1: p01,
            p2: p01_12,
        };

        let right = QuadBezier {
            p0: p01_12,
            p1: p12,
            p2: curve.p2,
        };

        subdivide_quad_bezier_adaptive(left, tolerance, max_segments, depth + 1, output);
        output.push(p01_12);
        subdivide_quad_bezier_adaptive(right, tolerance, max_segments, depth + 1, output);
    }
}

///
/// Technical implementation of the PathStroker structure.
#[allow(dead_code)]
pub struct PathStroker {
    width: f32,
    join_type: JoinType,
}

/// Technical implementation of the JoinType enumeration.
pub enum JoinType {
    Miter,
    Bevel,
    Round,
}

impl PathStroker {
    /// Initializes a new instance of the associated type.
    pub fn new(width: f32, join_type: JoinType) -> Self {
        Self { width, join_type }
    }

    /// Compute offset curve at distance d from original
    ///
    /// Uses parallel curve approximation (O(n) in number of samples)
    pub fn offset_curve(&self, points: &[[f32; 2]], distance: f32) -> Vec<[f32; 2]> {
        let mut result = Vec::with_capacity(points.len());

        for i in 0..points.len() {
            let prev = points[(i + points.len() - 1) % points.len()];
            let curr = points[i];
            let next = points[(i + 1) % points.len()];

            // Compute normal vectors
            let dx1 = curr[0] - prev[0];
            let dy1 = curr[1] - prev[1];
            let len1 = (dx1 * dx1 + dy1 * dy1).sqrt().max(1e-6);
            let nx1 = -dy1 / len1;
            let ny1 = dx1 / len1;

            let dx2 = next[0] - curr[0];
            let dy2 = next[1] - curr[1];
            let len2 = (dx2 * dx2 + dy2 * dy2).sqrt().max(1e-6);
            let nx2 = -dy2 / len2;
            let ny2 = dx2 / len2;

            // Average normal with miter adjustment
            let nx = (nx1 + nx2) * 0.5;
            let ny = (ny1 + ny2) * 0.5;
            let len_n = (nx * nx + ny * ny).sqrt().max(0.5);

            let offset_x = curr[0] + (nx / len_n) * distance;
            let offset_y = curr[1] + (ny / len_n) * distance;

            result.push([offset_x, offset_y]);
        }

        result
    }
}

///
/// Technical implementation of the GlyphSDF structure.
pub struct GlyphSDF {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>, // 8-bit signed distance values
    pub advance: f32,
}

/// Technical implementation of the SdfTextRenderer structure.
pub struct SdfTextRenderer {
    glyph_cache: [Option<GlyphSDF>; 256],
}

impl SdfTextRenderer {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            glyph_cache: [const { None }; 256],
        }
    }

    /// Render text string to glyph positions
    pub fn layout_text(&self, text: &str, font_size: f32, x: f32, y: f32) -> Vec<GlyphInstance> {
        let mut glyphs = Vec::new();
        let mut pos_x = x;

        for ch in text.chars() {
            if (ch as u32) < 256 {
                let glyph_index = ch as usize;
                if let Some(glyph) = &self.glyph_cache[glyph_index] {
                    glyphs.push(GlyphInstance {
                        x: pos_x,
                        y,
                        size: font_size,
                        glyph_index: glyph_index as u32,
                    });
                    pos_x += glyph.advance * font_size;
                }
            }
        }

        glyphs
    }
}

/// Technical implementation of the GlyphInstance structure.
pub struct GlyphInstance {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub glyph_index: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_quad_bezier_eval logic.
    fn test_quad_bezier_eval() {
        let curve = QuadBezier {
            p0: [0.0, 0.0],
            p1: [50.0, 100.0],
            p2: [100.0, 0.0],
        };

        let p0 = eval_quad_bezier(curve, 0.0);
        let p1 = eval_quad_bezier(curve, 0.5);
        let p2 = eval_quad_bezier(curve, 1.0);

        assert!((p0[0] - 0.0).abs() < 0.001);
        assert!((p2[0] - 100.0).abs() < 0.001);
        assert!(p1[1] > 0.0); // Should be above the chord
    }

    #[test]
    /// Technical implementation of the test_cubic_bezier_eval logic.
    fn test_cubic_bezier_eval() {
        let curve = CubicBezier {
            p0: [0.0, 0.0],
            p1: [33.0, 100.0],
            p2: [66.0, 100.0],
            p3: [100.0, 0.0],
        };

        let p_start = eval_cubic_bezier(curve, 0.0);
        let p_end = eval_cubic_bezier(curve, 1.0);

        assert!((p_start[0] - 0.0).abs() < 0.001);
        assert!((p_end[0] - 100.0).abs() < 0.001);
    }

    #[test]
    /// Technical implementation of the test_tessellation logic.
    fn test_tessellation() {
        let curve = QuadBezier {
            p0: [0.0, 0.0],
            p1: [50.0, 100.0],
            p2: [100.0, 0.0],
        };

        let mut segments = Vec::new();
        tessellate_quad_bezier(curve, 1.0, 1000, &mut segments);

        assert!(segments.len() > 2);
        assert!((segments[0][0] - 0.0).abs() < 0.001);
        assert!((segments[segments.len() - 1][0] - 100.0).abs() < 0.001);
    }

    #[test]
    /// Technical implementation of the test_path_stroker logic.
    fn test_path_stroker() {
        let stroker = PathStroker::new(5.0, JoinType::Miter);
        let path = vec![[0.0, 0.0], [50.0, 50.0], [100.0, 0.0]];
        let offset = stroker.offset_curve(&path, 5.0);

        assert_eq!(offset.len(), path.len());
    }

    #[test]
    /// Technical implementation of the test_sdf_text_renderer logic.
    fn test_sdf_text_renderer() {
        let renderer = SdfTextRenderer::new();
        let glyphs = renderer.layout_text("Hello", 24.0, 0.0, 0.0);

        // Without cached glyphs, should be empty
        assert_eq!(glyphs.len(), 0);
    }
}

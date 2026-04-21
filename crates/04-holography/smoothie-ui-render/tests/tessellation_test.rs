/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xef6a4340 | REVISION: 2026.04.20                           │
 * │ PATH: crates/04-holography/smoothie-ui-render/tests/tessellation_test.rs │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Integration tests for the UI tessellation engine.           │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Verification of triangulation logic.                    │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_ui_render::tessellation::Tessellator;
use smoothie_ui_render::svg::SvgCommand;

#[test]
fn test_polygon_tessellation() {
    let mut tess = Tessellator::new();
    let commands = vec![
        SvgCommand::MoveTo([0.0, 0.0]),
        SvgCommand::LineTo([100.0, 0.0]),
        SvgCommand::LineTo([100.0, 100.0]),
        SvgCommand::Close,
    ];
    
    tess.parse_and_tessellate(&commands);
    
    assert_eq!(tess.vertices.len(), 3);
    assert_eq!(tess.indices.len(), 3);
}

/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xbf29c340 | REVISION: 2026.04.20                           │
 * │ PATH: crates/04-holography/smoothie-ui-render/tests/materials_test.rs    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Integration tests for the PBR material engine.              │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Verification of material compilation and BRDF logic.    │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_ui_render::materials::{PbrMaterial, CookTorranceBRDF};
use smoothie_ui_render::SdfGenerator;
use smoothie_ui_core::Material;

#[test]
fn test_sdf_compilation() {
    let m = Material::BrushedMetal {
        anisotropy: 0.5,
        roughness: 0.1,
        grain_scale: 1.0,
    };
    let code = SdfGenerator::compile_material(m);
    assert!(code.contains("Brushed Metal"));
}

#[test]
fn test_brdf_calculation() {
    let material = PbrMaterial::default();
    let light_dir = [0.0, 1.0, 0.0];
    let view_dir = [0.0, 0.0, 1.0];
    let result = CookTorranceBRDF::calculate_brdf(&material, light_dir, view_dir);
    assert!(result[0] >= 0.0);
}

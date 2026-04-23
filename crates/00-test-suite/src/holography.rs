/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xbf29c340 | REVISION: 2026.04.20                           │
 * │ PATH: crates/00-test-suite/src/holography.rs                             │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Consolidated Holography Integration Tests.                  │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

use smoothie_ui_core::{Material, widgets::*};
use smoothie_ui_render::materials::{CookTorranceBRDF, PbrMaterial};
use smoothie_ui_render::SdfGenerator;
use smoothie_ui_render::svg::{SvgCommand, SvgDocument};
use smoothie_math::matrix::Vec3;
use smoothie_ui_render::raytrace::Ray;
use smoothie_ui_render::tessellation::Tessellator;

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

#[test]
fn test_svg_parsing_stub() {
    let raw = "<svg><path d='M 0 0 L 100 100 Z'/></svg>";
    let doc = SvgDocument::parse(raw);
    assert_eq!(doc.paths.len(), 1);
    assert_eq!(doc.paths[0][0], SvgCommand::MoveTo([0.0, 0.0]));
}

#[test]
fn test_ray_plane_intersection() {
    let origin = Vec3::new(0.0, 0.0, 10.0);
    let direction = Vec3::new(0.0, 0.0, -1.0);
    let ray = Ray::new(origin, direction);

    let plane_normal = Vec3::new(0.0, 0.0, 1.0);
    let plane_origin = Vec3::new(0.0, 0.0, 0.0);

    let t = ray.intersect_plane(plane_normal, plane_origin);
    assert_eq!(t, Some(10.0));
}

#[test]
fn test_ray_plane_no_intersection() {
    let origin = Vec3::new(0.0, 0.0, 10.0);
    let direction = Vec3::new(1.0, 0.0, 0.0); // Parallel to plane
    let ray = Ray::new(origin, direction);

    let plane_normal = Vec3::new(0.0, 0.0, 1.0);
    let plane_origin = Vec3::new(0.0, 0.0, 0.0);

    let t = ray.intersect_plane(plane_normal, plane_origin);
    assert_eq!(t, None);
}

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

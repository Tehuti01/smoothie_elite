/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x86ffa740 | REVISION: 2026.04.20                           │
 * │ PATH: crates/04-holography/smoothie-ui-render/tests/raytrace_test.rs     │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Integration tests for the UI raytracing engine.             │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Verification of intersection accuracy.                  │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_ui_render::raytrace::Ray;
use smoothie_math::matrix::Vec3;

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

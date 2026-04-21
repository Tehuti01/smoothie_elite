/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xd773b321 | REVISION: 2026.04.20                           │
 * │ PATH: crates/04-holography/smoothie-cli-frontend/tests/build_test.rs     │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Integration tests for the build command UI.                 │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Verification of UI orchestration flows.                 │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_cli_frontend::build::perform_build;

#[test]
fn test_build_ui_debug() {
    perform_build(false, None);
}

#[test]
fn test_build_ui_release() {
    perform_build(true, Some("vst3"));
}

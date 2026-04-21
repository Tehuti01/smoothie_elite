/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xf46d29c1 | REVISION: 2026.04.20                           │
 * │ PATH: crates/04-holography/smoothie-cli-frontend/tests/init_test.rs      │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Integration tests for the init command UI.                  │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Verification of environment sync feedback.              │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_cli_frontend::init::perform_init;

#[test]
fn test_init_ui() {
    perform_init();
}

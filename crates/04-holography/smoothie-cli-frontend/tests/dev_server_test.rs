/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb734e1a1 | REVISION: 2026.04.20                           │
 * │ PATH: crates/04-holography/smoothie-cli-frontend/tests/dev_server_test.rs│
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Integration tests for the dev server UI.                    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Verification of UI server status feedback.              │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_cli_frontend::dev_server::start_dev_server;

#[test]
fn test_dev_server_ui() {
    start_dev_server(3000);
}

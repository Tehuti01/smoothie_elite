/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xf46d29c1 | REVISION: 2026.04.20                           │
 * │ PATH: crates/00-test-suite/src/cli.rs                                    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Consolidated CLI Frontend Integration Tests.                │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */


#[test]
fn test_init_ui() {
    perform_init();
}

#[test]
fn test_build_ui_debug() {
    perform_build(false, None);
}

#[test]
fn test_build_ui_release() {
    perform_build(true, Some("vst3"));
}

#[test]
fn test_dev_server_ui() {
    start_dev_server(3000);
}

#[test]
fn test_scaffold_ui() {
    show_scaffold_start("MyProject", "instrument");
    show_scaffold_complete("MyProject", "./MyProject");
}

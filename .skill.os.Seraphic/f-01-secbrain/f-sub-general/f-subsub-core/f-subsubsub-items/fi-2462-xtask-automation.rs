---
id: fi-2462-xtask-automation.rs
category: f-01-secbrain
---

/// 🏗️ Example: xtask Automation
/// Rust-based CI/CD scripts.
fn main() {
    let task = std::env::args().nth(1);
    match task.as_deref() {
        Some("dist") => package_binary(),
        Some("audit") => run_full_audit(),
        _ => print_help(),
    }
}

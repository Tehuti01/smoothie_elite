use std::process::Command;
/// 🛠️ clippy_enforcer.rs v0.2.0
/// Enforces pedantic clippy rules for the Billion-Dollar Fullstack.
fn main() {
    println!("🚀 ENFORCING PEDANTIC CLIPPY...");
    let output = Command::new("cargo")
        .args(["clippy", "--workspace", "--", "-D", "clippy::pedantic", "-D", "warnings"])
        .output()
        .expect("Failed to execute clippy");
    if output.status.success() {
        println!("✅ SUCCESS: Code is Clippy-Pedantic compliant.");
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

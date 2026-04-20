# SKILL 009-B: CARGO ADVANCED

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        CARGO ADVANCED
                     Build Optimization & Custom Commands
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Advanced Cargo usage including custom commands, workspace optimization, and build scripts.

---

## WORKSPACES

### 1.1 Workspace Config

```toml
[workspace]
members = [
    "crates/*",
    "apps/*",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"

[workspace.dependencies]
tokio = "1.35"
serde = "1.0"

[workspace.metadata]
release = "stable"
```

### 1.2 Cargo.toml Profiles

```toml
[profile.dev]
opt-level = 0
debug = true

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1

[profile.bench]
inherits = "release"
debug = true
```

---

## BUILD SCRIPTS

### 2.1 build.rs

```rust
fn main() {
    println!("cargo:rerun-if-changed=src/bindings.h");
    println!("cargo:rustc-env=VERSION={}", std::env!("VERSION"));
    
    // Generate bindings
    let bindings = generate_bindings().unwrap();
    std::fs::write("src/generated.rs", bindings).unwrap();
}
```

---

## CUSTOM COMMANDS

### 3.1 Subcommands

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "myproject")]
#[command(about = "MyProject CLI tools")]
pub enum Cli {
    /// Build the project
    Build(BuildArgs),
    /// Run tests with coverage
    Test(TestArgs),
    /// Generate docs
    Docs(DocsArgs),
}

#[derive(Args)]
pub struct BuildArgs {
    #[arg(long, default_value = "release")]
    profile: String,
}
```

---

*Skill ID: 009-B | Category: Build | Complexity: Expert*
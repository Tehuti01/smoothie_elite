---
id: fi-174-cargo.toml.md
category: f-05-sysarch
---

[package]
name = "sovereign-rs"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
description.workspace = true
repository.workspace = true
homepage.workspace = true
keywords.workspace = true
categories.workspace = true
rust-version.workspace = true

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
clap = { version = "4.0", features = ["derive"] }
regex = "1.0"
walkdir = "2.3"
chrono = "0.4"
anyhow = "1.0"

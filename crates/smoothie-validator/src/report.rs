//! Validation report types.

use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckStatus {
    Pass,
    Warning,
    Fail,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub name:     String,
    pub status:   CheckStatus,
    pub severity: Severity,
    pub message:  String,
    pub detail:   Option<String>,
    pub fix:      Option<String>,
}

impl CheckResult {
    pub fn pass(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: CheckStatus::Pass,
            severity: Severity::Info,
            message: message.into(),
            detail: None,
            fix: None,
        }
    }

    pub fn warn(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: CheckStatus::Warning,
            severity: Severity::Warning,
            message: message.into(),
            detail: None,
            fix: None,
        }
    }

    pub fn fail(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: CheckStatus::Fail,
            severity: Severity::Error,
            message: message.into(),
            detail: None,
            fix: None,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn with_fix(mut self, fix: impl Into<String>) -> Self {
        self.fix = Some(fix.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub name:   String,
    pub checks: Vec<CheckResult>,
}

impl Section {
    pub fn passed(&self) -> usize {
        self.checks.iter().filter(|c| c.status == CheckStatus::Pass).count()
    }
    pub fn failed(&self) -> usize {
        self.checks.iter().filter(|c| c.status == CheckStatus::Fail).count()
    }
    pub fn warnings(&self) -> usize {
        self.checks.iter().filter(|c| c.status == CheckStatus::Warning).count()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub plugin_path: PathBuf,
    pub timestamp:   String,
    pub sections:    Vec<Section>,
}

impl ValidationReport {
    pub fn new(path: &Path) -> Self {
        Self {
            plugin_path: path.to_path_buf(),
            timestamp: chrono_now(),
            sections: Vec::new(),
        }
    }

    pub fn add_section(&mut self, name: &str, checks: Vec<CheckResult>) {
        self.sections.push(Section { name: name.to_string(), checks });
    }

    pub fn is_healthy(&self) -> bool {
        self.sections.iter().all(|s| s.failed() == 0)
    }

    pub fn total_passed(&self) -> usize { self.sections.iter().map(|s| s.passed()).sum() }
    pub fn total_failed(&self) -> usize { self.sections.iter().map(|s| s.failed()).sum() }
    pub fn total_warnings(&self) -> usize { self.sections.iter().map(|s| s.warnings()).sum() }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn print_summary(&self) {
        println!("=== Smoothie Elite Plugin Validation ===");
        println!("Plugin: {}", self.plugin_path.display());
        for section in &self.sections {
            println!("\n[{}]", section.name);
            for check in &section.checks {
                let icon = match check.status {
                    CheckStatus::Pass          => "✅",
                    CheckStatus::Warning       => "⚠️ ",
                    CheckStatus::Fail          => "❌",
                    CheckStatus::NotApplicable => "—  ",
                };
                println!("  {} {} — {}", icon, check.name, check.message);
                if let Some(detail) = &check.detail { println!("     Detail: {}", detail); }
                if let Some(fix)    = &check.fix    { println!("     Fix:    {}", fix); }
            }
        }
        println!("\n--- Summary ---");
        println!("Passed: {}  Warnings: {}  Failed: {}",
            self.total_passed(), self.total_warnings(), self.total_failed());
        if self.is_healthy() {
            println!("Status: HEALTHY — plugin is good to go!");
        } else {
            println!("Status: ISSUES FOUND — review failures above.");
        }
    }
}

fn chrono_now() -> String {
    // Simple timestamp without chrono dependency
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{}", secs)
}

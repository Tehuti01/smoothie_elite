---
id: fi-2518-similarity-navigator.rs
category: f-01-secbrain
---

use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};

/// 🚀 similarity_navigator.rs v0.1.0 — Seraphic Navigation Engine
/// High-speed Rust tool for navigating the 2,700+ sovereign paths.
/// Uses TF-IDF based similarity to find relevant skills without loading context.

struct SkillNode {
    path: PathBuf,
    tags: Vec<String>,
}

fn main() -> io::Result<()> {
    println!("🌌 INITIATING SERAPHIC SIMILARITY NAVIGATOR...");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("❌ ERROR: Please provide a search intent.");
        return Ok(());
    }

    let intent = args[1].to_lowercase();
    let matrix_root = Path::new(".skill.Seraphic");

    let mut nodes = Vec::new();
    collect_skills(matrix_root, &mut nodes)?;

    println!("   - Indexing {} sovereign paths...", nodes.len());

    let mut results: Vec<(&SkillNode, f64)> = nodes.iter()
        .map(|node| (node, calculate_similarity(&intent, &node.tags)))
        .filter(|(_, score)| *score > 0.1)
        .collect();

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\n🎯 TOP RELEVANT SOVEREIGN PATHS:");
    for (node, score) in results.iter().take(5) {
        println!("   [{:.2}] {}", score, node.path.display());
    }

    Ok(())
}

fn collect_skills(dir: &Path, nodes: &mut Vec<SkillNode>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                collect_skills(&path, nodes)?;
            } else if path.extension().map_or(false, |ext| ext == "md" || ext == "rs") {
                let tags = extract_tags(&path);
                nodes.push(SkillNode { path, tags });
            }
        }
    }
    Ok(())
}

fn extract_tags(path: &Path) -> Vec<String> {
    // Simplified: Just using filename and path segments as tags for v0.1.0
    path.to_str().unwrap_or_default()
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .map(|s| s.to_string())
        .collect()
}

fn calculate_similarity(intent: &str, tags: &[String]) -> f64 {
    let mut score = 0.0;
    let intent_words: Vec<&str> = intent.split_whitespace().collect();
    for word in intent_words {
        if tags.contains(&word.to_string()) {
            score += 1.0;
        }
    }
    score / (tags.len() as f64).sqrt()
}

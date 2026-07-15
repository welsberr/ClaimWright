use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

const REQUIRED_FILES: &[&str] = &[
    "MOU.md",
    "LICENSE",
    "policies/principles.yaml",
    "policies/claim_states.yaml",
    "policies/enforcement.yaml",
    "checks/pre_action.yaml",
    "checks/post_action.yaml",
    "roles/claim-auditor.md",
    "roles/adversarial-reviewer.md",
    "roles/synthesis-mapper.md",
    "roles/knowledge-base-maintainer.md",
    "roles/citation-reviewer.md",
    "roles/publication-gatekeeper.md",
    "schemas/claim-record.schema.json",
    "schemas/citation-review.schema.json",
    "sources/pennock-scientific-virtues.md",
    "roadmap/ROADMAP.md",
];

const REQUIRED_MOU_TERMS: &[&str] = &[
    "Grounded work",
    "Public Defensibility",
    "Anti-Flattery",
    "Durable Correction",
    "Negative Results",
    "Capability And Cost Matching",
];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "check" {
        eprintln!("usage: claimwright check <repo-root>");
        process::exit(2);
    }

    let root = PathBuf::from(&args[2]);
    let mut failures = Vec::new();

    for rel in REQUIRED_FILES {
        let path = root.join(rel);
        if !path.is_file() {
            failures.push(format!("missing required file: {}", rel));
        } else if is_empty(&path) {
            failures.push(format!("empty required file: {}", rel));
        }
    }

    let mou_path = root.join("MOU.md");
    if let Ok(mou) = fs::read_to_string(&mou_path) {
        for term in REQUIRED_MOU_TERMS {
            if !mou.contains(term) {
                failures.push(format!("MOU.md missing required term: {}", term));
            }
        }
    }

    let enforcement = root.join("policies/enforcement.yaml");
    if let Ok(text) = fs::read_to_string(&enforcement) {
        for gate in [
            "unresolved_high_risk_public_claim",
            "fabricated_or_unverified_citation",
            "private_material_publication",
            "destructive_irreversible_action",
            "contradicted_or_stale_claim",
        ] {
            if !text.contains(gate) {
                failures.push(format!("enforcement.yaml missing hard gate: {}", gate));
            }
        }
    }

    if failures.is_empty() {
        println!("ClaimWright check passed: policy substrate is present.");
    } else {
        for failure in &failures {
            eprintln!("error: {}", failure);
        }
        process::exit(1);
    }
}

fn is_empty(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => metadata.len() == 0,
        Err(_) => true,
    }
}

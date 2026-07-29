use std::collections::HashSet;
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
    "roles/prior-work-reviewer.md",
    "schemas/claim-record.schema.json",
    "schemas/citation-review.schema.json",
    "sources/pennock-scientific-virtues.md",
    "roadmap/ROADMAP.md",
    "fixtures/groundrecall/institutional_policy_cases.json",
];

const INSTITUTIONAL_POLICY_ACTIONS: &[&str] = &[
    "discover_federation_catalog",
    "read_federation_catalog_entry",
    "propose_group_contribution",
    "review_group_contribution",
    "accept_group_contribution",
    "publish_federation_catalog",
    "import_federation_catalog",
    "manage_federation_subscription",
    "export_incremental_changes",
    "import_incremental_changes",
    "record_federation_feedback",
    "transfer_knowledge_custody",
    "retire_federation_instance",
    "generate_scope_orientation",
    "generate_stewardship_view",
    "generate_change_impact_report",
    "publish_knowledge_release_pack",
    "withdraw_knowledge_release",
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

    let institutional_fixture = root.join("fixtures/groundrecall/institutional_policy_cases.json");
    if let Ok(text) = fs::read_to_string(&institutional_fixture) {
        if !text.contains("groundrecall.institutional_policy_fixtures.v1") {
            failures.push("institutional policy fixture has an unknown schema version".to_string());
        }
        for action in INSTITUTIONAL_POLICY_ACTIONS {
            if !text.contains(action) {
                failures.push(format!(
                    "institutional policy fixture missing action: {}",
                    action
                ));
            }
        }
        let mut case_ids = HashSet::new();
        for line in text.lines().filter(|line| line.contains("\"case_id\"")) {
            if let Some(value) = line
                .split("\"case_id\": \"")
                .nth(1)
                .and_then(|rest| rest.split('"').next())
            {
                if !case_ids.insert(value.to_string()) {
                    failures.push(format!(
                        "institutional policy fixture has duplicate case ID: {}",
                        value
                    ));
                }
            } else {
                failures.push("institutional policy fixture has malformed case ID".to_string());
            }
        }
        let allowed_decisions = ["allow", "require_review", "soft_gate", "hard_gate", "deny"];
        for line in text
            .lines()
            .filter(|line| line.contains("\"expected_decision\""))
        {
            let value = line
                .split("\"expected_decision\": \"")
                .nth(1)
                .and_then(|rest| rest.split('"').next());
            if !value.is_some_and(|item| allowed_decisions.contains(&item)) {
                failures.push(
                    "institutional policy fixture has an invalid expected decision".to_string(),
                );
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

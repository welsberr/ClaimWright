use super::artifact;
use super::hash::sha256_hex;
use super::model::{CheckRecord, CheckStatus, Decision, ReviewRecord, SimilarityReview};
use super::schema::REQUIRED_CHECK_IDS;
use serde_json::to_string_pretty;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn init_review(args: &[String]) -> Result<String, String> {
    let mut artifact_path = None;
    let mut scope = None;
    let mut output = None;
    let mut similarity_reports: Vec<String> = Vec::new();
    let mut force = false;
    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--artifact" => {
                i += 1;
                artifact_path = args.get(i).cloned();
            }
            "--release-scope" => {
                i += 1;
                scope = args.get(i).cloned();
            }
            "--output" => {
                i += 1;
                output = args.get(i).cloned();
            }
            "--similarity-report" => {
                i += 1;
                if let Some(path) = args.get(i) {
                    similarity_reports.push(path.clone());
                }
            }
            "--force" => force = true,
            other => return Err(format!("unknown option: {other}")),
        }
        i += 1;
    }
    let artifact_path = artifact_path.ok_or("--artifact is required")?;
    let scope = scope.ok_or("--release-scope is required")?;
    let output = output.ok_or("--output is required")?;
    let artifact = PathBuf::from(&artifact_path);
    let out = PathBuf::from(&output);
    if out.exists() && !force {
        return Err(format!(
            "refusing to overwrite existing review: {} (use --force)",
            out.display()
        ));
    }
    let info = artifact::inspect(&artifact)?;
    let policy_path = Path::new("policies/academic_publication.yaml");
    let policy = fs::read(policy_path).map_err(|e| format!("cannot read policy: {e}"))?;
    let checks = REQUIRED_CHECK_IDS
        .iter()
        .map(|id| CheckRecord {
            id: id.to_string(),
            status: CheckStatus::Unresolved,
            evidence: vec!["pending integrity review".into()],
            limitations: Some(vec!["No publication decision has been made.".into()]),
            corrective_actions: Some(vec!["Complete human review before release.".into()]),
            rationale: None,
        })
        .collect();
    let record = ReviewRecord {
        schema_version: "claimwright.publication_integrity_review.v1".into(),
        artifact_id: artifact_path,
        artifact_sha256: info.sha256,
        artifact_text_sha256: info.text_sha256,
        release_scope: scope,
        policy_id: "claimwright.academic_publication_integrity.v1".into(),
        policy_sha256: sha256_hex(&policy),
        tool_version: env!("CARGO_PKG_VERSION").into(),
        destination: "unspecified".into(),
        destination_policy_version: None,
        checks,
        similarity_review: SimilarityReview {
            method: "not yet performed".into(),
            corpus_limitations: vec!["Similarity review is required before public release.".into()],
            material_matches: vec![],
        },
        decision: Decision::HardGate,
        decision_rationale: Some(
            "Initialized review; human integrity review is incomplete.".into(),
        ),
        human_reviewer: "UNASSIGNED".into(),
        second_reviewer: None,
        ai_use: None,
        prior_publication: None,
        related_work: None,
        source_references: None,
        permission_references: None,
        reviewed_at: timestamp(),
        notes: Some(format!(
            "Artifact kind: {}. Extracted text is required for non-text artifacts.",
            artifact::kind_name(info.kind)
        )),
    };
    if let Some(parent) = out.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent).map_err(|e| format!("cannot create output directory: {e}"))?;
    }
    fs::write(
        &out,
        format!(
            "{}\n",
            to_string_pretty(&record).map_err(|e| e.to_string())?
        ),
    )
    .map_err(|e| format!("cannot write review: {e}"))?;
    Ok(format!("Initialized publication review: {}", out.display()))
}

pub fn check_review(args: &[String]) -> Result<(String, i32), String> {
    let mut artifact_path = None;
    let mut review_path = None;
    let mut format = "human";
    let mut output = None;
    let mut similarity_reports: Vec<String> = Vec::new();
    let mut destination_policy = None;
    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--artifact" => {
                i += 1;
                artifact_path = args.get(i).cloned()
            }
            "--review" => {
                i += 1;
                review_path = args.get(i).cloned()
            }
            "--format" => {
                i += 1;
                format = args.get(i).map(String::as_str).unwrap_or("human")
            }
            "--output" => {
                i += 1;
                output = args.get(i).cloned()
            }
            "--similarity-report" => {
                i += 1;
                if let Some(path) = args.get(i) {
                    similarity_reports.push(path.clone());
                }
            }
            "--destination-policy" => {
                i += 1;
                destination_policy = args.get(i).cloned();
            }
            x => return Err(format!("unknown option: {x}")),
        }
        i += 1;
    }
    if !["human", "json"].contains(&format) {
        return Err("--format must be human or json".into());
    }
    let ap = artifact_path.ok_or("--artifact is required")?;
    let rp = review_path.ok_or("--review is required")?;
    let info = artifact::inspect(Path::new(&ap))?;
    let review = fs::read_to_string(&rp).map_err(|e| format!("cannot read review: {e}"))?;
    let policy = fs::read("policies/academic_publication.yaml")
        .map_err(|e| format!("cannot read policy: {e}"))?;
    let ph = sha256_hex(&policy);
    let mut similarity_candidates = Vec::new();
    for path in similarity_reports {
        let imported = super::similarity::load(Path::new(&path), &info.sha256)?;
        similarity_candidates.extend(super::similarity::summaries(&imported));
    }
    let record = super::schema::validate_review_json(&review, Some(&info.sha256), Some(&ph))
        .map_err(|e| {
            e.iter()
                .map(|x| format!("{}: {}", x.reason_code, x.message))
                .collect::<Vec<_>>()
                .join("\n")
        })?;
    let mut result =
        super::evaluator::evaluate(&record, &info, &ph, timestamp(), similarity_candidates);
    if let Some(path) = destination_policy {
        let profile = super::destination::load(Path::new(&path))?;
        for (code, msg) in super::destination::findings(&profile, &record) {
            result.findings.push(super::evaluator::Finding {
                reason_code: code,
                policy_id: record.policy_id.clone(),
                check_id: "venue_and_release_policy".into(),
                decision: "hard_gate".into(),
                severity: "high".into(),
                message: msg,
                artifact_location: None,
                evidence_refs: vec![],
                limitations: vec![],
                required_actions: vec!["Provide destination-required evidence.".into()],
                human_review_required: true,
            });
        }
        if !result.findings.is_empty() {
            result.overall_decision = "hard_gate".into();
        }
    }
    let text = if format == "json" {
        super::report::json(&result)?
    } else {
        super::report::human(&result)
    };
    if let Some(path) = output {
        fs::write(path, &text).map_err(|e| format!("cannot write report: {e}"))?;
    }
    let code = if result.overall_decision == "pass" {
        0
    } else {
        1
    };
    Ok((text, code))
}

fn timestamp() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days = seconds / 86400;
    let rem = seconds % 86400;
    let (year, month, day) = civil(days as i64);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}
fn civil(days: i64) -> (i64, u32, u32) {
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    (y + if m <= 2 { 1 } else { 0 }, m as u32, d as u32)
}

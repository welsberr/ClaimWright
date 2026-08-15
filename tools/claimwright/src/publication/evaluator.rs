use super::artifact::ArtifactInfo;
use super::model::{CheckStatus, Decision, ReviewRecord};
use super::similarity::CandidateSummary;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Finding {
    pub reason_code: String,
    pub policy_id: String,
    pub check_id: String,
    pub decision: String,
    pub severity: String,
    pub message: String,
    pub artifact_location: Option<String>,
    pub evidence_refs: Vec<String>,
    pub limitations: Vec<String>,
    pub required_actions: Vec<String>,
    pub human_review_required: bool,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EvaluationReport {
    pub report_version: String,
    pub artifact_sha256: String,
    pub artifact_text_sha256: Option<String>,
    pub policy_id: String,
    pub policy_sha256: String,
    pub tool_version: String,
    pub evaluation_timestamp: String,
    pub overall_decision: String,
    pub findings: Vec<Finding>,
    pub similarity_candidates: Vec<CandidateSummary>,
    pub similarity_method: String,
    pub corpus_limitations: Vec<String>,
    pub checks_evaluated: Vec<String>,
    pub checks_not_applicable: Vec<String>,
    pub required_actions: Vec<String>,
    pub human_reviewer: String,
    pub approval_state: String,
    pub statement: String,
}
pub fn evaluate(
    record: &ReviewRecord,
    artifact: &ArtifactInfo,
    policy_sha256: &str,
    timestamp: String,
    similarity_candidates: Vec<CandidateSummary>,
) -> EvaluationReport {
    let mut findings = Vec::new();
    let mut evaluated = Vec::new();
    let mut na = Vec::new();
    for c in &record.checks {
        evaluated.push(c.id.clone());
        if matches!(c.status, CheckStatus::DocumentedNotApplicable) {
            na.push(c.id.clone());
        }
        if matches!(c.status, CheckStatus::Fail | CheckStatus::Unresolved) {
            findings.push(Finding {
                reason_code: format!("publication.{}.unresolved", c.id),
                policy_id: record.policy_id.clone(),
                check_id: c.id.clone(),
                decision: "hard_gate".into(),
                severity: "high".into(),
                message: format!("{} review remains unresolved or failed", c.id),
                artifact_location: None,
                evidence_refs: c.evidence.clone(),
                limitations: c.limitations.clone().unwrap_or_default(),
                required_actions: c
                    .corrective_actions
                    .clone()
                    .unwrap_or_else(|| vec!["Resolve this review category before release.".into()]),
                human_review_required: true,
            });
        }
    }
    if artifact.sha256 != record.artifact_sha256 {
        findings.push(simple(
            "publication.artifact.hash_mismatch",
            "artifact hash does not match review binding",
            "artifact",
        ));
    }
    if record.policy_sha256 != policy_sha256 {
        findings.push(simple(
            "publication.review.policy_hash_mismatch",
            "policy hash does not match evaluated policy",
            "policy",
        ));
    }
    findings.sort_by(|a, b| {
        (
            &a.decision,
            &a.check_id,
            &a.reason_code,
            a.artifact_location.as_deref().unwrap_or(""),
        )
            .cmp(&(
                &b.decision,
                &b.check_id,
                &b.reason_code,
                b.artifact_location.as_deref().unwrap_or(""),
            ))
    });
    for c in &similarity_candidates {
        if c.materiality == "material"
            && (c.disposition.eq_ignore_ascii_case("unresolved")
                || c.disposition.eq_ignore_ascii_case("pending"))
        {
            findings.push(Finding {
                reason_code: "publication.similarity.unresolved_match".into(),
                policy_id: record.policy_id.clone(),
                check_id: "similarity_and_text_recycling".into(),
                decision: "hard_gate".into(),
                severity: "high".into(),
                message: format!(
                    "material similarity candidate at {} lacks human disposition",
                    c.artifact_location
                ),
                artifact_location: Some(c.artifact_location.clone()),
                evidence_refs: vec![c.source_id.clone()],
                limitations: vec![],
                required_actions: vec!["Record a human disposition before release.".into()],
                human_review_required: true,
            });
        }
    }
    findings.sort_by(|a, b| {
        (
            &a.decision,
            &a.check_id,
            &a.reason_code,
            a.artifact_location.as_deref().unwrap_or(""),
        )
            .cmp(&(
                &b.decision,
                &b.check_id,
                &b.reason_code,
                b.artifact_location.as_deref().unwrap_or(""),
            ))
    });
    let overall = if findings.iter().any(|f| f.decision == "deny") {
        "deny"
    } else if findings.is_empty() && matches!(record.decision, Decision::Pass) {
        "pass"
    } else {
        "hard_gate"
    };
    let actions = findings
        .iter()
        .flat_map(|f| f.required_actions.clone())
        .collect();
    EvaluationReport {
        report_version: "claimwright.publication_report.v1".into(),
        artifact_sha256: artifact.sha256.clone(),
        artifact_text_sha256: artifact.text_sha256.clone(),
        policy_id: record.policy_id.clone(),
        policy_sha256: policy_sha256.into(),
        tool_version: record.tool_version.clone(),
        evaluation_timestamp: timestamp,
        overall_decision: overall.into(),
        findings,
        similarity_candidates,
        similarity_method: record.similarity_review.method.clone(),
        corpus_limitations: record.similarity_review.corpus_limitations.clone(),
        checks_evaluated: evaluated,
        checks_not_applicable: na,
        required_actions: actions,
        human_reviewer: record.human_reviewer.clone(),
        approval_state: if record.decision == Decision::Pass {
            "approved".into()
        } else {
            "pending".into()
        },
        statement: "Policy findings are not permission grants or misconduct adjudications.".into(),
    }
}
fn simple(code: &str, message: &str, location: &str) -> Finding {
    Finding {
        reason_code: code.into(),
        policy_id: "claimwright.academic_publication_integrity.v1".into(),
        check_id: "artifact".into(),
        decision: "hard_gate".into(),
        severity: "high".into(),
        message: message.into(),
        artifact_location: Some(location.into()),
        evidence_refs: vec![],
        limitations: vec![],
        required_actions: vec!["Reinitialize or correct the review binding.".into()],
        human_review_required: true,
    }
}

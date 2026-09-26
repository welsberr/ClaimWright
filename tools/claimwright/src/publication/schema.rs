use super::model::{CheckStatus, Decision, ReviewRecord};
use serde_json::Value;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

pub const REQUIRED_CHECK_IDS: &[&str] = &[
    "plagiarism_and_attribution",
    "similarity_and_text_recycling",
    "fabrication_falsification_and_manipulation",
    "citation_and_source_integrity",
    "authorship_contribution_and_ai_disclosure",
    "copyright_license_and_permissions",
    "ethics_consent_conflicts_and_funding",
    "confidentiality_and_private_material",
    "harmful_or_unprofessional_content",
    "venue_and_release_policy",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub reason_code: String,
    pub message: String,
}

impl ValidationError {
    fn new(reason_code: &str, message: impl Into<String>) -> Self {
        Self {
            reason_code: reason_code.to_string(),
            message: message.into(),
        }
    }
}

/// Parse and semantically validate a review record. Hash arguments are optional
/// so initialization/templates can be checked before an artifact is available.
pub fn validate_review_json(
    input: &str,
    expected_artifact_sha256: Option<&str>,
    expected_policy_sha256: Option<&str>,
) -> Result<ReviewRecord, Vec<ValidationError>> {
    let value: Value = match serde_json::from_str(input) {
        Ok(value) => value,
        Err(error) => {
            return Err(vec![ValidationError::new(
                "publication.review.invalid_json",
                error.to_string(),
            )])
        }
    };
    let mut errors = validate_shape(&value);
    let record: ReviewRecord = match serde_json::from_value(value) {
        Ok(record) => record,
        Err(error) => {
            errors.push(ValidationError::new(
                "publication.review.schema_invalid",
                error.to_string(),
            ));
            return Err(errors);
        }
    };
    errors.extend(validate_semantics(
        &record,
        expected_artifact_sha256,
        expected_policy_sha256,
    ));
    if errors.is_empty() {
        Ok(record)
    } else {
        Err(errors)
    }
}

fn validate_shape(value: &Value) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let Some(object) = value.as_object() else {
        errors.push(ValidationError::new(
            "publication.review.schema_invalid",
            "review must be a JSON object",
        ));
        return errors;
    };
    let allowed: HashSet<&str> = [
        "schema_version",
        "artifact_id",
        "artifact_sha256",
        "artifact_text_sha256",
        "release_scope",
        "policy_id",
        "policy_sha256",
        "tool_version",
        "destination",
        "destination_policy_version",
        "checks",
        "similarity_review",
        "decision",
        "decision_rationale",
        "human_reviewer",
        "second_reviewer",
        "ai_use",
        "prior_publication",
        "related_work",
        "source_references",
        "permission_references",
        "reviewed_at",
        "notes",
    ]
    .into_iter()
    .collect();
    for key in object.keys() {
        if !allowed.contains(key.as_str()) {
            errors.push(ValidationError::new(
                "publication.review.unknown_field",
                format!("unknown field: {key}"),
            ));
        }
    }
    errors
}

fn validate_semantics(
    record: &ReviewRecord,
    expected_artifact: Option<&str>,
    expected_policy: Option<&str>,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    if record.schema_version != "claimwright.publication_integrity_review.v1" {
        errors.push(ValidationError::new(
            "publication.review.schema_version",
            "unsupported review schema version",
        ));
    }
    if record.policy_id != "claimwright.academic_publication_integrity.v1" {
        errors.push(ValidationError::new(
            "publication.review.policy_id",
            "review is not bound to the academic publication policy",
        ));
    }
    for (name, hash) in [
        ("artifact_sha256", &record.artifact_sha256),
        ("policy_sha256", &record.policy_sha256),
    ] {
        if !is_sha256(hash) {
            errors.push(ValidationError::new(
                "publication.review.invalid_hash",
                format!("{name} must be a SHA-256 hex digest"),
            ));
        }
    }
    if let Some(expected) = expected_artifact {
        if !hashes_equal(&record.artifact_sha256, expected) {
            errors.push(ValidationError::new(
                "publication.artifact.hash_mismatch",
                "artifact hash does not match evaluated artifact",
            ));
        }
    }
    if let Some(expected) = expected_policy {
        if !hashes_equal(&record.policy_sha256, expected) {
            errors.push(ValidationError::new(
                "publication.review.policy_hash_mismatch",
                "policy hash does not match evaluated policy",
            ));
        }
    }
    if record.human_reviewer.trim().is_empty() {
        errors.push(ValidationError::new(
            "publication.review.reviewer_missing",
            "human reviewer is required",
        ));
    }
    let reviewed_at = timestamp_seconds(&record.reviewed_at);
    if reviewed_at.is_none() {
        errors.push(ValidationError::new(
            "publication.review.invalid_timestamp",
            "reviewed_at must be an RFC3339 timestamp",
        ));
    }
    if reviewed_at.is_some_and(|seconds| {
        seconds
            > SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64
    }) {
        errors.push(ValidationError::new(
            "publication.review.future_timestamp",
            "review timestamp is in the future",
        ));
    }
    if record.decision == Decision::Pass
        && record
            .human_reviewer
            .trim()
            .eq_ignore_ascii_case("UNASSIGNED")
    {
        errors.push(ValidationError::new(
            "publication.review.reviewer_missing",
            "a passing review must name its human reviewer",
        ));
    }
    for (field, value) in [
        ("artifact_id", &record.artifact_id),
        ("release_scope", &record.release_scope),
        ("destination", &record.destination),
        ("tool_version", &record.tool_version),
    ] {
        if value.trim().is_empty() {
            errors.push(ValidationError::new(
                "publication.review.schema_invalid",
                format!("{field} must be nonempty"),
            ));
        }
    }

    let mut seen = HashSet::new();
    for check in &record.checks {
        if !seen.insert(check.id.as_str()) {
            errors.push(ValidationError::new(
                "publication.review.duplicate_check",
                format!("duplicate check ID: {}", check.id),
            ));
        }
        if !REQUIRED_CHECK_IDS.contains(&check.id.as_str()) && !check.id.starts_with("extension.") {
            errors.push(ValidationError::new(
                "publication.review.unknown_check",
                format!("unknown check ID: {}", check.id),
            ));
        }
        if check.evidence.iter().all(|item| item.trim().is_empty()) {
            errors.push(ValidationError::new(
                "publication.review.evidence_missing",
                format!("check {} has no evidence", check.id),
            ));
        }
        if matches!(check.status, CheckStatus::DocumentedNotApplicable)
            && check.rationale.as_deref().unwrap_or("").trim().is_empty()
        {
            errors.push(ValidationError::new(
                "publication.review.na_rationale_missing",
                format!("check {} requires a rationale", check.id),
            ));
        }
    }
    for required in REQUIRED_CHECK_IDS {
        if !seen.contains(required) {
            errors.push(ValidationError::new(
                "publication.review.required_check_missing",
                format!("missing required check: {required}"),
            ));
        }
    }
    if matches!(record.decision, Decision::Pass)
        && record
            .checks
            .iter()
            .any(|check| matches!(check.status, CheckStatus::Fail | CheckStatus::Unresolved))
    {
        errors.push(ValidationError::new(
            "publication.review.pass_with_blocking_check",
            "pass cannot coexist with failed or unresolved checks",
        ));
    }
    if matches!(record.decision, Decision::Deny)
        && record
            .decision_rationale
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
    {
        errors.push(ValidationError::new(
            "publication.review.deny_rationale_missing",
            "deny requires an authorized human rationale",
        ));
    }
    if record.similarity_review.method.trim().is_empty() {
        errors.push(ValidationError::new(
            "publication.similarity.method_missing",
            "similarity method is required",
        ));
    }
    if record
        .similarity_review
        .corpus_limitations
        .iter()
        .all(|item| item.trim().is_empty())
    {
        errors.push(ValidationError::new(
            "publication.similarity.limitations_missing",
            "similarity corpus limitations are required",
        ));
    }
    for candidate in &record.similarity_review.material_matches {
        if candidate.source.trim().is_empty()
            || candidate.location.trim().is_empty()
            || candidate.disposition.trim().is_empty()
            || candidate.rationale.trim().is_empty()
        {
            errors.push(ValidationError::new("publication.similarity.disposition_missing", "every material similarity match needs source, location, disposition, and rationale"));
        }
        if candidate.disposition.eq_ignore_ascii_case("unresolved")
            || candidate.disposition.eq_ignore_ascii_case("pending")
        {
            errors.push(ValidationError::new(
                "publication.similarity.unresolved_match",
                format!("unresolved material match at {}", candidate.location),
            ));
        }
    }
    errors
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
fn hashes_equal(left: &str, right: &str) -> bool {
    left.eq_ignore_ascii_case(right)
}
// RFC3339 calendar and UTC-offset validation, at whole-second precision.
// Leap-second notation is deliberately rejected rather than normalized silently.
fn timestamp_seconds(value: &str) -> Option<i64> {
    if !value.is_ascii() || value.len() < 20 {
        return None;
    }
    let b = value.as_bytes();
    if b[4] != b'-'
        || b[7] != b'-'
        || ![b'T', b't'].contains(&b[10])
        || b[13] != b':'
        || b[16] != b':'
    {
        return None;
    }
    let number = |start: usize, end: usize| -> Option<i64> {
        let part = value.get(start..end)?;
        if !part.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        part.parse().ok()
    };
    let (year, month, day) = (number(0, 4)?, number(5, 7)?, number(8, 10)?);
    let (hour, minute, second) = (number(11, 13)?, number(14, 16)?, number(17, 19)?);
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let months = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    if !(1..=12).contains(&month)
        || !(1..=months[(month - 1) as usize]).contains(&day)
        || hour > 23
        || minute > 59
        || second > 59
    {
        return None;
    }
    let mut end = 19;
    if b.get(end) == Some(&b'.') {
        end += 1;
        let start = end;
        while b.get(end).is_some_and(u8::is_ascii_digit) {
            end += 1;
        }
        if end == start {
            return None;
        }
    }
    let offset = match value.get(end..)? {
        "Z" | "z" => 0,
        zone if zone.len() == 6
            && [b'+', b'-'].contains(&zone.as_bytes()[0])
            && zone.as_bytes()[3] == b':' =>
        {
            let h = number(end + 1, end + 3)?;
            let m = number(end + 4, end + 6)?;
            if h > 23 || m > 59 {
                return None;
            }
            (h * 3600 + m * 60) * if zone.starts_with('-') { -1 } else { 1 }
        }
        _ => return None,
    };
    let before = |y: i64| 365 * y + (y + 3) / 4 - (y + 99) / 100 + (y + 399) / 400;
    let days =
        before(year) - before(1970) + months[..(month - 1) as usize].iter().sum::<i64>() + day - 1;
    Some(days * 86400 + hour * 3600 + minute * 60 + second - offset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn fixture(name: &str) -> String {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/publication")
            .join(name);
        fs::read_to_string(path).expect("fixture")
    }

    #[test]
    fn valid_fixture_passes() {
        assert!(validate_review_json(&fixture("valid-review.json"), None, None).is_ok());
    }

    #[test]
    fn invalid_fixture_reports_stable_reasons() {
        let errors = validate_review_json(&fixture("invalid-review.json"), None, None).unwrap_err();
        let codes: Vec<&str> = errors
            .iter()
            .map(|error| error.reason_code.as_str())
            .collect();
        assert!(codes.contains(&"publication.review.invalid_hash"));
        assert!(codes.contains(&"publication.review.required_check_missing"));
        assert!(codes.contains(&"publication.review.reviewer_missing"));
    }

    #[test]
    fn pass_cannot_hide_unresolved_check() {
        let mut value: Value = serde_json::from_str(&fixture("valid-review.json")).unwrap();
        value["checks"][0]["status"] = Value::String("unresolved".into());
        let errors = validate_review_json(&value.to_string(), None, None).unwrap_err();
        assert!(errors
            .iter()
            .any(|error| error.reason_code == "publication.review.pass_with_blocking_check"));
    }

    #[test]
    fn material_match_requires_disposition() {
        let mut value: Value = serde_json::from_str(&fixture("valid-review.json")).unwrap();
        value["similarity_review"]["material_matches"] = serde_json::json!([{
            "source": "prior-work", "location": "p. 1", "disposition": "unresolved", "rationale": "pending"
        }]);
        let errors = validate_review_json(&value.to_string(), None, None).unwrap_err();
        assert!(errors
            .iter()
            .any(|error| error.reason_code == "publication.similarity.unresolved_match"));
    }

    #[test]
    fn timestamps_validate_calendar_and_offsets() {
        assert_eq!(timestamp_seconds("1970-01-01T00:00:00Z"), Some(0));
        assert_eq!(
            timestamp_seconds("2026-03-16T17:46:33-04:00"),
            timestamp_seconds("2026-03-16T21:46:33Z")
        );
        assert_eq!(
            timestamp_seconds("2026-03-16T21:46:33.123+00:00"),
            timestamp_seconds("2026-03-16T21:46:33Z")
        );
        for bad in [
            "2026-02-29T00:00:00Z",
            "2026-04-31T00:00:00Z",
            "2026-03-16T25:00:00Z",
            "2026-03-16T00:00:00+25:00",
            "2026-03-16T00:00:00.Z",
        ] {
            assert!(timestamp_seconds(bad).is_none(), "{bad}");
        }
        assert!(timestamp_seconds("2024-02-29T00:00:00Z").is_some());
    }
}

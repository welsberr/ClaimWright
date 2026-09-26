use super::model::ReviewRecord;
#[derive(Debug, Clone)]
pub struct Profile {
    pub destination: String,
    pub version: String,
    pub ai: bool,
    pub prior: bool,
    pub ethics: bool,
    pub conflicts: bool,
    pub data: bool,
}
pub fn load(path: &std::path::Path) -> Result<Profile, String> {
    let t = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read destination policy: {e}"))?;
    let allowed = [
        "schema_version",
        "destination",
        "policy_version",
        "require_ai_disclosure",
        "require_prior_publication_disclosure",
        "require_ethics_approval",
        "require_conflict_statement",
        "require_data_availability",
    ];
    let mut fields = std::collections::BTreeMap::new();
    for line in t
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let (key, value) = line
            .split_once(':')
            .ok_or("publication.destination.invalid_profile: expected flat key: value entries")?;
        let key = key.trim();
        if !allowed.contains(&key) || fields.contains_key(key) {
            return Err(format!(
                "publication.destination.invalid_profile: unknown or duplicate field {key}"
            ));
        }
        fields.insert(key.to_string(), value.trim().trim_matches('"').to_string());
    }
    let get = |key: &str| fields.get(key).cloned();
    if get("schema_version").as_deref() != Some("claimwright.destination_policy.v1") {
        return Err(
            "publication.destination.invalid_profile: missing or unsupported schema_version".into(),
        );
    }
    // This is intentionally a flat key/value profile, not a general YAML parser.
    // Refuse ambiguous booleans rather than silently disabling a requirement.
    for key in [
        "require_ai_disclosure",
        "require_prior_publication_disclosure",
        "require_ethics_approval",
        "require_conflict_statement",
        "require_data_availability",
    ] {
        if get(key).is_some_and(|value| value != "true" && value != "false") {
            return Err(format!(
                "publication.destination.invalid_profile: {key} must be true or false"
            ));
        }
    }
    let destination = get("destination")
        .filter(|x| !x.is_empty())
        .ok_or("publication.destination.invalid_profile: destination missing")?;
    let version = get("policy_version")
        .filter(|x| !x.is_empty())
        .ok_or("publication.destination.invalid_profile: policy_version missing")?;
    Ok(Profile {
        destination,
        version,
        ai: get("require_ai_disclosure").is_some_and(|x| x == "true"),
        prior: get("require_prior_publication_disclosure").is_some_and(|x| x == "true"),
        ethics: get("require_ethics_approval").is_some_and(|x| x == "true"),
        conflicts: get("require_conflict_statement").is_some_and(|x| x == "true"),
        data: get("require_data_availability").is_some_and(|x| x == "true"),
    })
}
pub fn findings(p: &Profile, r: &ReviewRecord) -> Vec<(String, String)> {
    let mut o = Vec::new();
    if r.destination != p.destination {
        o.push((
            "publication.destination.name_mismatch".into(),
            "review destination does not match the supplied profile".into(),
        ));
    }
    if r.destination_policy_version.as_deref() != Some(p.version.as_str()) {
        o.push((
            "publication.destination.version_mismatch".into(),
            "review destination policy version does not match the supplied profile".into(),
        ));
    }
    if p.ai && r.ai_use.is_none() {
        o.push((
            "publication.destination.ai_disclosure_missing".into(),
            "destination requires AI-use disclosure".into(),
        ))
    }
    if p.prior && r.prior_publication.is_none() {
        o.push((
            "publication.destination.prior_publication_missing".into(),
            "destination requires prior-publication disclosure".into(),
        ))
    }
    if p.ethics
        && !r.checks.iter().any(|c| {
            c.id == "ethics_consent_conflicts_and_funding" && format!("{:?}", c.status) == "Pass"
        })
    {
        o.push((
            "publication.destination.ethics_approval_missing".into(),
            "destination requires ethics approval evidence".into(),
        ))
    }
    if p.conflicts
        && !r.checks.iter().any(|c| {
            c.id == "ethics_consent_conflicts_and_funding"
                && c.evidence
                    .iter()
                    .any(|e| e.to_lowercase().contains("conflict"))
        })
    {
        o.push((
            "publication.destination.conflict_statement_missing".into(),
            "destination requires conflict statement".into(),
        ))
    }
    if p.data
        && !r.checks.iter().any(|c| {
            c.id == "venue_and_release_policy"
                && c.evidence.iter().any(|e| e.to_lowercase().contains("data"))
        })
    {
        o.push((
            "publication.destination.data_availability_missing".into(),
            "destination requires data availability evidence".into(),
        ))
    }
    o
}

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
    let get = |k: &str| {
        t.lines().find_map(|l| {
            let mut p = l.splitn(2, ':');
            if p.next()?.trim() == k {
                Some(p.next()?.trim().trim_matches('"').to_string())
            } else {
                None
            }
        })
    };
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

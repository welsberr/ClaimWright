use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CandidateSummary {
    pub source_id: String,
    pub artifact_location: String,
    pub overlap_kind: String,
    pub materiality: String,
    pub disposition: String,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct SimilarityReport {
    pub schema_version: String,
    pub artifact_sha256: String,
    pub method: String,
    pub tool_version: String,
    pub corpus: String,
    pub corpus_limitations: Vec<String>,
    pub candidates: Vec<Candidate>,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Candidate {
    pub source_id: String,
    pub artifact_location: String,
    pub source_location: Option<String>,
    pub overlap_kind: String,
    pub score: Option<f64>,
    pub score_semantics: Option<String>,
    pub excerpt: Option<String>,
    pub excerpt_hash: Option<String>,
    pub materiality: String,
    pub disposition: String,
    pub disposition_rationale: Option<String>,
}
pub fn load(path: &std::path::Path, artifact_hash: &str) -> Result<SimilarityReport, String> {
    let text =
        std::fs::read_to_string(path).map_err(|e| format!("cannot read similarity report: {e}"))?;
    let r: SimilarityReport =
        serde_json::from_str(&text).map_err(|e| format!("invalid similarity report: {e}"))?;
    if r.schema_version != "claimwright.similarity_candidate_report.v1" {
        return Err("publication.similarity.schema_version: unsupported report version".into());
    }
    if !r.artifact_sha256.eq_ignore_ascii_case(artifact_hash) {
        return Err(
            "publication.similarity.artifact_hash_mismatch: report does not match artifact".into(),
        );
    }
    if r.method.trim().is_empty() || r.corpus.trim().is_empty() || r.corpus_limitations.is_empty() {
        return Err("publication.similarity.limitations_missing: method, corpus, and limitations are required".into());
    }
    for c in &r.candidates {
        if c.excerpt.as_ref().is_some_and(|e| e.len() > 1000) {
            return Err(
                "publication.similarity.evidence_too_long: excerpt exceeds 1000 characters".into(),
            );
        }
    }
    Ok(r)
}
pub fn summaries(r: &SimilarityReport) -> Vec<CandidateSummary> {
    r.candidates
        .iter()
        .map(|c| CandidateSummary {
            source_id: c.source_id.clone(),
            artifact_location: c.artifact_location.clone(),
            overlap_kind: c.overlap_kind.clone(),
            materiality: c.materiality.clone(),
            disposition: c.disposition.clone(),
        })
        .collect()
}

pub fn generate(
    artifact: &std::path::Path,
    corpus: &std::path::Path,
    output: &std::path::Path,
    ngram: usize,
    threshold: f64,
) -> Result<(), String> {
    let text =
        std::fs::read_to_string(artifact).map_err(|e| format!("cannot read artifact text: {e}"))?;
    let words: Vec<String> = normalize(&text)
        .split_whitespace()
        .map(str::to_string)
        .collect();
    let mut candidates = Vec::new();
    for entry in std::fs::read_dir(corpus).map_err(|e| e.to_string())? {
        let p = entry.map_err(|e| e.to_string())?.path();
        if !p.is_file() {
            continue;
        }
        let n = normalize(&std::fs::read_to_string(&p).unwrap_or_default());
        for i in 0..words.len().saturating_sub(ngram.saturating_sub(1)) {
            let phrase = words[i..i + ngram].join(" ");
            if n.contains(&phrase) {
                candidates.push(Candidate {
                    source_id: p.display().to_string(),
                    artifact_location: format!(
                        "byte:{}-{}",
                        text.to_lowercase().find(&words[i]).unwrap_or(0),
                        text.to_lowercase().find(&words[i]).unwrap_or(0) + phrase.len()
                    ),
                    source_location: None,
                    overlap_kind: "exact".into(),
                    score: None,
                    score_semantics: None,
                    excerpt: Some(phrase),
                    excerpt_hash: None,
                    materiality: "unknown".into(),
                    disposition: "unresolved".into(),
                    disposition_rationale: None,
                });
                break;
            }
        }
        if !candidates
            .iter()
            .any(|c: &Candidate| c.source_id == p.display().to_string())
        {
            let a: std::collections::HashSet<String> = words.iter().cloned().collect();
            let b: std::collections::HashSet<String> =
                n.split_whitespace().map(str::to_string).collect();
            let j = a.intersection(&b).count() as f64 / a.union(&b).count().max(1) as f64;
            if j >= threshold {
                candidates.push(Candidate {
                    source_id: p.display().to_string(),
                    artifact_location: "byte:0-0".into(),
                    source_location: None,
                    overlap_kind: "near_exact".into(),
                    score: Some(j),
                    score_semantics: Some(format!("token Jaccard threshold {threshold}")),
                    excerpt: None,
                    excerpt_hash: None,
                    materiality: "unknown".into(),
                    disposition: "unresolved".into(),
                    disposition_rationale: None,
                });
            }
        }
    }
    let bytes = std::fs::read(artifact).map_err(|e| e.to_string())?;
    let report = SimilarityReport {
        schema_version: "claimwright.similarity_candidate_report.v1".into(),
        artifact_sha256: super::hash::sha256_hex(&bytes),
        method: format!("offline exact token n-gram (size {ngram}); near-duplicate threshold {threshold}"),
        tool_version: env!("CARGO_PKG_VERSION").into(),
        corpus: corpus.display().to_string(),
        corpus_limitations: vec![
            "Local corpus only; quotation, references, and boilerplate remain visible candidates; threshold is discovery-only.".into(),
        ],
        candidates,
    };
    std::fs::write(
        output,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
        ),
    )
    .map_err(|e| format!("cannot write report: {e}"))?;
    Ok(())
}
fn normalize(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ReviewRecord {
    pub schema_version: String,
    pub artifact_id: String,
    pub artifact_sha256: String,
    pub artifact_text_sha256: Option<String>,
    pub release_scope: String,
    pub policy_id: String,
    pub policy_sha256: String,
    pub tool_version: String,
    pub destination: String,
    pub destination_policy_version: Option<String>,
    pub checks: Vec<CheckRecord>,
    pub similarity_review: SimilarityReview,
    pub decision: Decision,
    pub decision_rationale: Option<String>,
    pub human_reviewer: String,
    pub second_reviewer: Option<String>,
    pub ai_use: Option<AiUse>,
    pub prior_publication: Option<PriorPublication>,
    pub related_work: Option<Vec<String>>,
    pub source_references: Option<Vec<String>>,
    pub permission_references: Option<Vec<String>>,
    pub reviewed_at: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CheckRecord {
    pub id: String,
    pub status: CheckStatus,
    pub evidence: Vec<String>,
    pub limitations: Option<Vec<String>>,
    pub corrective_actions: Option<Vec<String>>,
    pub rationale: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CheckStatus {
    Pass,
    Fail,
    Unresolved,
    DocumentedNotApplicable,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SimilarityReview {
    pub method: String,
    pub corpus_limitations: Vec<String>,
    pub material_matches: Vec<MaterialMatch>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct MaterialMatch {
    pub source: String,
    pub location: String,
    pub disposition: String,
    pub rationale: String,
    pub evidence_refs: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Decision {
    Pass,
    HardGate,
    Deny,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct AiUse {
    pub used: bool,
    pub disclosure: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PriorPublication {
    pub has_prior_work: bool,
    pub disclosure: String,
    pub references: Option<Vec<String>>,
}

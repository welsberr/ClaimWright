use super::evaluator::EvaluationReport;
pub fn json(report: &EvaluationReport) -> Result<String, String> {
    Ok(format!(
        "{}\n",
        serde_json::to_string_pretty(report).map_err(|e| e.to_string())?
    ))
}
pub fn human(r: &EvaluationReport) -> String {
    let mut s = format!(
        "Decision: {}\nFindings: {}\n",
        r.overall_decision,
        r.findings.len()
    );
    for f in &r.findings {
        s.push_str(&format!(
            "- {} [{}] {}\n",
            f.reason_code, f.decision, f.message
        ));
    }
    s
}

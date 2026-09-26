use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

struct Fixture {
    root: PathBuf,
    repo: PathBuf,
    review: Value,
}
impl Fixture {
    fn new() -> Self {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let root = std::env::temp_dir().join(format!(
            "claimwright-publication-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&root).unwrap();
        let review = serde_json::from_slice(
            &fs::read(repo.join("fixtures/publication/passing-review.json")).unwrap(),
        )
        .unwrap();
        Self { root, repo, review }
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_claimwright"))
            .current_dir(&self.repo)
            .args(args)
            .output()
            .unwrap()
    }
    fn check(&self, extra: &[&str]) -> Output {
        let path = self.root.join("review.json");
        fs::write(&path, self.review.to_string()).unwrap();
        let mut args = vec![
            "publication",
            "check",
            "--artifact",
            "fixtures/publication/passing-artifact.txt",
            "--review",
            path.to_str().unwrap(),
            "--format",
            "json",
        ];
        args.extend_from_slice(extra);
        self.run(&args)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
fn report(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).expect(&String::from_utf8_lossy(&output.stderr))
}

#[test]
fn pass_and_deny_remain_distinct() {
    let mut f = Fixture::new();
    let out = f.check(&[]);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(report(&out)["overall_decision"], "pass");
    f.review["decision"] = json!("deny");
    let out = f.check(&[]);
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(report(&out)["overall_decision"], "deny");
    assert_eq!(report(&out)["approval_state"], "pending");
}

#[test]
fn initialized_review_is_gated_and_not_overwritten() {
    let f = Fixture::new();
    let path = f.root.join("initialized.json");
    let args = [
        "publication",
        "init-review",
        "--artifact",
        "fixtures/publication/passing-artifact.txt",
        "--release-scope",
        "synthetic test",
        "--output",
        path.to_str().unwrap(),
    ];
    assert!(f.run(&args).status.success());
    assert_eq!(f.run(&args).status.code(), Some(2));
    let out = f.run(&[
        "publication",
        "check",
        "--artifact",
        "fixtures/publication/passing-artifact.txt",
        "--review",
        path.to_str().unwrap(),
        "--format",
        "json",
    ]);
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(report(&out)["overall_decision"], "hard_gate");
}

#[test]
fn destination_binding_gates_without_weakening_deny() {
    let mut f = Fixture::new();
    let profile = f.root.join("destination.yaml");
    fs::write(
        &profile,
        "schema_version: claimwright.destination_policy.v1\ndestination: example-journal\npolicy_version: v1\nrequire_ai_disclosure: true\n",
    )
    .unwrap();
    let extra = ["--destination-policy", profile.to_str().unwrap()];
    let out = f.check(&extra);
    assert_eq!(out.status.code(), Some(1));
    let r = report(&out);
    assert_eq!(r["approval_state"], "pending");
    assert!(r["findings"]
        .as_array()
        .unwrap()
        .iter()
        .any(|x| x["reason_code"] == "publication.destination.name_mismatch"));
    assert!(!r["required_actions"].as_array().unwrap().is_empty());
    f.review["destination"] = json!("example-journal");
    f.review["destination_policy_version"] = json!("v1");
    f.review["ai_use"] = json!({"used":true,"disclosure":"Synthetic disclosure for a fixture."});
    assert_eq!(f.check(&extra).status.code(), Some(0));
    f.review["decision"] = json!("deny");
    assert_eq!(report(&f.check(&extra))["overall_decision"], "deny");
    fs::write(
        &profile,
        "schema_version: claimwright.destination_policy.v1\ndestination: example-journal\npolicy_version: v1\nrequire_ai_disclosure: ture\n",
    )
    .unwrap();
    assert_eq!(f.check(&extra).status.code(), Some(2));
}

#[test]
fn unknown_similarity_requires_review_and_init_preserves_candidate() {
    let f = Fixture::new();
    let mut similarity: Value = serde_json::from_slice(
        &fs::read(f.repo.join("fixtures/publication/similarity-pass.json")).unwrap(),
    )
    .unwrap();
    similarity["candidates"][0]["materiality"] = json!("unknown");
    similarity["candidates"][0]["disposition"] = json!("unresolved");
    let path = f.root.join("similarity.json");
    fs::write(&path, similarity.to_string()).unwrap();
    let out = f.check(&["--similarity-report", path.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(report(&out)["approval_state"], "pending");
    let initialized = f.root.join("initialized.json");
    let out = f.run(&[
        "publication",
        "init-review",
        "--artifact",
        "fixtures/publication/passing-artifact.txt",
        "--release-scope",
        "fixture",
        "--output",
        initialized.to_str().unwrap(),
        "--similarity-report",
        path.to_str().unwrap(),
    ]);
    assert!(out.status.success());
    let record: Value = serde_json::from_slice(&fs::read(initialized).unwrap()).unwrap();
    assert_eq!(
        record["similarity_review"]["material_matches"][0]["disposition"],
        "pending"
    );
}

#[test]
fn malformed_inputs_and_io_have_distinct_exit_codes() {
    let mut f = Fixture::new();
    assert_eq!(f.check(&["--similarity-report"]).status.code(), Some(2));
    assert_eq!(f.check(&["--destination-policy"]).status.code(), Some(2));
    assert_eq!(
        f.check(&["--output", f.root.join("absent/out.json").to_str().unwrap()])
            .status
            .code(),
        Some(3)
    );
    let out = f.run(&[
        "publication",
        "check",
        "--artifact",
        "no-such-artifact",
        "--review",
        "no-such-review",
    ]);
    assert_eq!(out.status.code(), Some(3));
    f.review["artifact_sha256"] = json!("0".repeat(64));
    let out = f.check(&[]);
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("hash_mismatch"));
}

#[test]
fn similarity_generation_rejects_bad_parameters_and_unreadable_corpus() {
    let f = Fixture::new();
    let corpus = f.root.join("corpus");
    fs::create_dir(&corpus).unwrap();
    fs::write(
        corpus.join("source.txt"),
        "A fixture artifact for publication integrity checking.\n",
    )
    .unwrap();
    let output = f.root.join("candidates.json");
    let base = [
        "publication",
        "similarity",
        "generate",
        "--artifact",
        "fixtures/publication/passing-artifact.txt",
        "--comparison-corpus",
        corpus.to_str().unwrap(),
        "--output",
        output.to_str().unwrap(),
    ];
    for (option, value) in [
        ("--ngram-size", "0"),
        ("--ngram-size", "oops"),
        ("--jaccard-threshold", "NaN"),
        ("--jaccard-threshold", "2"),
    ] {
        let mut args = base.to_vec();
        args.extend([option, value]);
        assert_eq!(f.run(&args).status.code(), Some(2));
    }
    assert!(f.run(&base).status.success());
    fs::write(corpus.join("binary.dat"), [255u8]).unwrap();
    assert_eq!(f.run(&base).status.code(), Some(3));
}

#[test]
fn nonempty_reviewer_and_real_calendar_timestamp_are_required() {
    let mut f = Fixture::new();
    f.review["reviewed_at"] = json!("2026-07-31T04:42:30-04:00");
    assert_eq!(f.check(&[]).status.code(), Some(0));
    f.review["reviewed_at"] = json!("2026-02-30T04:42:30Z");
    assert_eq!(f.check(&[]).status.code(), Some(2));
    f.review["reviewed_at"] = json!("2099-01-01T00:00:00Z");
    assert_eq!(f.check(&[]).status.code(), Some(2));
    f.review["reviewed_at"] = json!("2026-07-31T08:42:30Z");
    f.review["human_reviewer"] = json!("UNASSIGNED");
    assert_eq!(f.check(&[]).status.code(), Some(2));
}

#[test]
fn text_binding_and_non_text_artifacts_cannot_bypass_review() {
    let mut f = Fixture::new();
    f.review["artifact_text_sha256"] = json!("0".repeat(64));
    let out = f.check(&[]);
    assert_eq!(out.status.code(), Some(1));
    assert!(report(&out)["findings"]
        .as_array()
        .unwrap()
        .iter()
        .any(|x| x["reason_code"] == "publication.artifact.text_hash_mismatch"));
    let pdf = f.root.join("synthetic.pdf");
    fs::write(&pdf, "%PDF-synthetic fixture, not a real document").unwrap();
    let review = f.root.join("pdf-review.json");
    let out = f.run(&[
        "publication",
        "init-review",
        "--artifact",
        pdf.to_str().unwrap(),
        "--release-scope",
        "test",
        "--output",
        review.to_str().unwrap(),
    ]);
    assert!(out.status.success());
    let mut value: Value = serde_json::from_slice(&fs::read(&review).unwrap()).unwrap();
    value["decision"] = json!("pass");
    value["human_reviewer"] = json!("synthetic-reviewer");
    for check in value["checks"].as_array_mut().unwrap() {
        check["status"] = json!("pass");
    }
    fs::write(&review, value.to_string()).unwrap();
    let out = f.run(&[
        "publication",
        "check",
        "--artifact",
        pdf.to_str().unwrap(),
        "--review",
        review.to_str().unwrap(),
        "--format",
        "json",
    ]);
    assert_eq!(out.status.code(), Some(1));
    assert!(report(&out)["findings"]
        .as_array()
        .unwrap()
        .iter()
        .any(|x| x["reason_code"] == "publication.artifact.extracted_text_required"));
}

#[test]
fn unknown_review_and_destination_fields_are_rejected() {
    let mut f = Fixture::new();
    f.review["checks"][0]["unknown_field"] = json!(true);
    assert_eq!(f.check(&[]).status.code(), Some(2));
    f.review["checks"][0]
        .as_object_mut()
        .unwrap()
        .remove("unknown_field");
    let profile = f.root.join("policy.yaml");
    for extra in ["require_ai_dislosure: true", "destination: duplicate"] {
        fs::write(&profile,format!("schema_version: claimwright.destination_policy.v1\ndestination: example\npolicy_version: v1\n{extra}\n")).unwrap();
        assert_eq!(
            f.check(&["--destination-policy", profile.to_str().unwrap()])
                .status
                .code(),
            Some(2)
        );
    }
}

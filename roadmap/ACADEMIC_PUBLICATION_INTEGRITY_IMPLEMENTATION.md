# Academic Publication Integrity Gate: Implementation Plan

Status: implementation-ready  
Policy authority: `policies/academic_publication.yaml`  
Review record: `schemas/publication-integrity-review.schema.json`  
Last reviewed: 2026-07-31

## 1. Objective

Implement an executable ClaimWright publication gate that:

1. binds an integrity review to the exact artifact being released;
2. validates that all ten policy review areas have evidence and a disposition;
3. imports or generates textual-similarity candidates without declaring
   plagiarism automatically;
4. blocks release when required evidence, human review, disclosures,
   permissions, or destination-policy checks are missing or unresolved;
5. emits deterministic human-readable and machine-readable findings; and
6. remains offline and safe for confidential manuscripts by default.

The first production-capable version is an evidence and policy evaluator. It is
not a universal plagiarism detector, research-misconduct adjudicator, legal
review system, or substitute for an accountable human publication decision.

## 2. Controlling Semantics

The coding model must not invent alternate policy meanings.

- `policies/academic_publication.yaml` is the controlling rule set.
- `hard_gate` means the artifact must not be released while the finding
  remains unresolved.
- `deny` is reserved for confirmed misconduct, unlawful disclosure, or an
  uncorrectable rights or ethics violation recorded by authorized human
  review. Automated code must not independently accuse a person of misconduct.
- Similarity results are candidates for human review, not plagiarism findings.
- A low similarity score cannot produce a pass.
- A high similarity score cannot by itself produce `deny`.
- `documented_not_applicable` is valid only with a non-empty rationale.
- Passing this gate does not replace the other ClaimWright gates or final human
  publication approval.
- Honest error and disagreement must be distinguishable from misconduct
  allegations, while still requiring correction when the artifact is wrong.

## 3. Scope

### 3.1 MVP inputs

Support:

- UTF-8 plain text;
- Markdown;
- extracted text supplied for HTML or PDF artifacts;
- a JSON publication-integrity review record;
- zero or more JSON similarity-candidate reports;
- an optional local comparison corpus;
- an optional destination-policy profile.

Do not silently scan raw PDF, office, image, archive, or binary files as text.
If extracted text is not supplied for such an artifact, return a hard gate with
reason code `publication.artifact_text_unavailable`.

### 3.2 MVP outputs

Support:

- concise terminal report;
- versioned JSON report;
- exit status suitable for CI;
- optional initialized review-record template;
- no network writes and no artifact modification.

### 3.3 Deferred capabilities

Defer until the core evaluator is stable:

- commercial similarity-service integrations;
- automatic internet corpus collection;
- cross-language semantic similarity;
- image-forensics judgments;
- retraction-database and journal-submission integrations;
- autonomous correction of manuscript prose;
- automatic misconduct classification;
- automatic final publication approval.

## 4. Command-Line Contract

Preserve the existing command:

```text
claimwright check <repo-root>
```

Add:

```text
claimwright publication init-review \
  --artifact <path> \
  --release-scope <description> \
  --output <review.json>

claimwright publication check \
  --artifact <path> \
  --review <review.json> \
  [--extracted-text <path>] \
  [--similarity-report <path>]... \
  [--comparison-corpus <directory>] \
  [--destination-policy <path>] \
  [--format human|json] \
  [--output <path>]
```

Never overwrite an existing review or report unless the caller supplies an
explicit `--force` flag. `--force` must not change evaluation results or waive
findings.

Exit codes:

| Code | Meaning |
|---|---|
| `0` | Integrity decision is `pass` |
| `1` | Decision is `hard_gate` or `deny` |
| `2` | Invalid arguments, malformed input, or schema failure |
| `3` | Internal I/O or evaluator failure |

The JSON output must be written even for exit code `1` when the inputs are
valid. It must not claim a pass if report writing fails.

## 5. Data Model

### 5.1 Strengthen the review schema first

Extend `schemas/publication-integrity-review.schema.json` with:

- `schema_version`;
- `artifact_sha256`;
- `artifact_text_sha256` when extracted text is used;
- `policy_id`;
- `policy_sha256`;
- `tool_version`;
- `destination`;
- `destination_policy_version` when applicable;
- a required rationale for `documented_not_applicable`;
- reviewer identity and review timestamp;
- optional second-reviewer identity;
- explicit AI-use disclosure data;
- explicit prior-publication and related-work disclosures;
- source and permission references that use identifiers or paths rather than
  embedding confidential source content.

The evaluator, rather than JSON Schema alone, must ensure:

- each of the ten required check IDs occurs exactly once;
- no unknown check ID is accepted unless namespaced as an extension;
- `documented_not_applicable` has a rationale;
- `fail` or `unresolved` cannot coexist with a review-level `pass`;
- every material similarity match has exactly one human disposition;
- the artifact and policy hashes match the files evaluated;
- timestamps are valid and the review is not from a future time beyond a small
  clock-skew allowance;
- the named human reviewer is non-empty;
- a `deny` decision includes an authorized human rationale.

### 5.2 Finding model

Create a Rust finding type with at least:

```text
reason_code
policy_id
check_id
decision
severity
message
artifact_location
evidence_refs
limitations
required_actions
human_review_required
```

Use stable reason codes beginning with:

```text
publication.plagiarism.
publication.similarity.
publication.fabrication.
publication.citation.
publication.authorship.
publication.rights.
publication.ethics.
publication.confidentiality.
publication.harm.
publication.venue.
publication.artifact.
publication.review.
```

Messages may improve over time; reason codes are compatibility contracts.

### 5.3 Report model

The versioned JSON report must contain:

- artifact and extracted-text hashes;
- policy ID and hash;
- tool version;
- evaluation timestamp;
- overall decision;
- findings sorted deterministically by decision, check ID, reason code, and
  artifact location;
- similarity method and corpus limitations;
- checks evaluated and checks not applicable;
- required actions;
- human reviewers and approval state;
- a statement that policy findings are not permission grants or misconduct
  adjudications.

Do not include secrets, entire matched documents, confidential manuscript
contents, or unnecessarily long excerpts in the report.

## 6. Rust Architecture

Refactor the current single-file CLI without changing existing behavior:

```text
tools/claimwright/src/
  main.rs
  lib.rs
  substrate.rs
  publication/
    mod.rs
    cli.rs
    model.rs
    schema.rs
    artifact.rs
    review.rs
    evaluator.rs
    similarity.rs
    destination.rs
    report.rs
```

Responsibilities:

- `substrate.rs`: current repository substrate checks.
- `artifact.rs`: safe file classification, text loading, normalization, and
  SHA-256 binding.
- `schema.rs`: JSON schema and semantic validation.
- `review.rs`: review initialization and review-record parsing.
- `evaluator.rs`: pure policy evaluation from validated inputs.
- `similarity.rs`: generic report import and optional local candidate
  discovery.
- `destination.rs`: optional destination-policy profile parsing.
- `report.rs`: deterministic human and JSON rendering.
- `cli.rs`: argument parsing and exit-code mapping only.

Keep policy evaluation pure where practical: input structs in, report struct
out. File and terminal operations belong at the edges.

Expected dependencies:

- `clap` with derive support;
- `serde`, `serde_json`, and `serde_yaml`;
- `sha2`;
- `chrono` with minimal features;
- a maintained JSON Schema validator compatible with draft 2020-12;
- `thiserror`;
- `tempfile` as a development dependency.

Pin resolved versions in `Cargo.lock`. Do not add async or network dependencies
for the MVP.

## 7. Similarity Candidate Handling

### 7.1 Generic import format

Define `schemas/similarity-candidate-report.schema.json` with:

- schema version;
- artifact hash;
- method/tool name and version;
- corpus name, date, and limitations;
- candidate source identifier;
- artifact location;
- source location when available;
- overlap kind: `exact`, `near_exact`, `paraphrase_candidate`,
  `self_overlap`, or `cross_language_candidate`;
- score plus score semantics;
- short bounded excerpts or excerpt hashes;
- materiality status;
- human disposition;
- disposition rationale.

The importer must preserve the originating score semantics. It must not compare
scores from unrelated tools as though they use the same scale.

### 7.2 Optional local candidate discovery

After generic import works, add an offline local-corpus candidate generator:

1. decode UTF-8 safely;
2. normalize Unicode and line endings while retaining source offsets;
3. exclude configured reference-list and quoted-block regions only from the
   aggregate score, not from reporting;
4. generate exact phrase and token n-gram candidates;
5. optionally use winnowing or MinHash for near-duplicate discovery;
6. report candidates with offsets and bounded excerpts;
7. never emit `pass`, `plagiarism`, or `deny`;
8. require human disposition for material candidates.

Cross-language and semantic-paraphrase detection remain explicit coverage
limitations until a reviewed implementation exists.

### 7.3 Confidentiality

- Run locally by default.
- Do not transmit artifacts or candidate text.
- Require explicit configuration for any future remote adapter.
- A remote adapter must declare retention, training, jurisdiction, and
  confidentiality properties before use.
- Refuse to send a confidential artifact when those properties or authority
  are absent.

## 8. Evaluation Rules

Evaluate in this order:

1. load and hash the policy;
2. classify and hash the artifact;
3. load extracted text if required;
4. validate and bind the review record;
5. load and bind similarity reports;
6. load destination policy;
7. evaluate required categories;
8. evaluate similarity dispositions;
9. evaluate cross-cutting consistency;
10. determine the most restrictive decision;
11. render deterministic reports.

Decision composition:

```text
deny > hard_gate > require_review > allow
```

For this gate, unresolved required evidence maps to `hard_gate`, not
`require_review`, because public release is already the decision point.

Minimum hard-gate cases:

- missing review record;
- schema-invalid review;
- artifact or policy hash mismatch;
- missing required category;
- duplicate category;
- empty evidence for an asserted pass;
- undocumented not-applicable category;
- unresolved or failed check;
- missing human reviewer;
- missing similarity method or corpus limitations;
- material similarity candidate without a disposition;
- destination profile required but missing;
- raw PDF/HTML/binary artifact without trusted extracted text;
- required AI-use, prior-publication, conflict, funding, consent, or permission
  disclosure absent;
- detected private material or confidential-source misuse.

The evaluator may report `deny` only when the input review already records an
authorized human `deny` determination. Otherwise confirmed-looking automated
findings remain `hard_gate`.

## 9. Destination-Policy Profiles

Define an optional YAML schema supporting:

- destination name and policy version;
- artifact type;
- prior-publication and preprint rules;
- simultaneous-submission prohibition;
- AI disclosure requirements;
- word, figure, table, and supplement constraints;
- data and code availability requirements;
- conflict, funding, ethics, consent, and registration requirements;
- licenses and permission requirements;
- embargo or public-release timing;
- required human roles.

Profiles supplement ClaimWright; they cannot weaken ClaimWright hard gates.
Unknown or contradictory destination requirements must produce a hard gate.

## 10. Work Packages

### WP-0: Preserve and refactor the substrate checker

Tasks:

1. Add CLI parsing while retaining `claimwright check <repo-root>`.
2. Move existing substrate logic into `substrate.rs`.
3. Add regression integration tests for current pass and failure messages.
4. Keep the current success text unless intentionally versioned.

Acceptance:

- existing command and exit behavior are unchanged;
- current repository passes;
- a fixture missing a required file fails;
- formatting and tests pass.

### WP-1: Schema and semantic validation

Tasks:

1. Strengthen the publication review schema.
2. Add Rust models using `serde`.
3. Add JSON Schema validation.
4. Add semantic validation for exact category coverage, dispositions, hashes,
   rationales, decisions, and reviewer fields.
5. Add fixture review records.

Acceptance:

- valid fixture passes validation;
- every malformed or incomplete fixture fails with stable reason codes;
- no panic occurs for malformed JSON, invalid UTF-8 paths, or missing files.

### WP-2: Artifact binding and review initialization

Tasks:

1. Implement safe artifact type detection.
2. Compute SHA-256 for raw artifact and evaluated text.
3. Implement `publication init-review`.
4. Initialize all ten category records as `unresolved`.
5. Refuse accidental overwrite.

Acceptance:

- initialized review binds to the artifact and current policy;
- raw PDF or binary input records the need for extracted text;
- rerunning without `--force` preserves the existing review.

### WP-3: Policy evaluator and reports

Tasks:

1. Implement pure evaluation and decision composition.
2. Implement stable reason codes.
3. Implement human and JSON reports.
4. Implement exit-code contract.
5. Ensure report ordering is deterministic.

Acceptance:

- complete passing review exits `0`;
- unresolved review exits `1` with `hard_gate`;
- malformed input exits `2`;
- I/O failure exits `3`;
- repeated evaluation produces byte-identical JSON except when a caller opts
  into a current evaluation timestamp.

### WP-4: Similarity report import

Tasks:

1. Add similarity-candidate schema.
2. Parse one or more reports.
3. Verify artifact hashes.
4. Require tool/corpus limitations.
5. Hard-gate every material candidate lacking human disposition.
6. render bounded evidence without copying source documents.

Acceptance:

- zero candidates does not independently create a pass;
- unresolved material match hard-gates;
- accepted quotation, common phrase, properly attributed reuse, and corrected
  overlap dispositions are preserved distinctly;
- mismatched artifact hash fails input validation.

### WP-5: Offline local candidate generator

Tasks:

1. Add normalization with reversible offsets.
2. Add exact phrase and token n-gram matching.
3. Add near-duplicate candidate discovery.
4. Add configurable thresholds documented as discovery parameters.
5. emit generic candidate reports only.

Acceptance:

- seeded copied passages are found;
- quotation/reference handling remains visible;
- common boilerplate can be classified without being silently deleted;
- no test asserts that a score proves plagiarism;
- performance is measured on a documented corpus size.

### WP-6: Destination profiles and cross-checks

Tasks:

1. Add profile schema and parser.
2. Map destination requirements to review evidence.
3. Hard-gate missing requirements.
4. prevent profiles from weakening core policy.

Acceptance:

- a profile can require AI disclosure, preprint disclosure, ethics approval,
  conflict statements, and data availability;
- missing required fields hard-gate with stable reason codes;
- permissive profiles cannot suppress core findings.

### WP-7: CI and integration surfaces

Tasks:

1. Add a CI example that checks a prepared artifact and review.
2. Add versioned fixture responses for a future
   `claimwright.publication_gate` MCP surface.
3. Document consumption by GroundRecall, CiteGeist, and publication workflows.
4. Keep findings distinct from permission grants.

Acceptance:

- CI blocks on exit `1`, `2`, or `3`;
- machine report contains policy and artifact hashes;
- MCP fixture and local evaluator agree on representative decisions;
- no integration can silently convert `hard_gate` to `allow`.

## 11. Required Test Matrix

Create fixtures and tests for at least:

| Case | Expected result |
|---|---|
| complete reviewed artifact | `pass`, exit `0` |
| review missing | `hard_gate`, exit `1` |
| malformed review JSON | invalid input, exit `2` |
| missing required category | `hard_gate` |
| duplicate category ID | `hard_gate` |
| undocumented not applicable | `hard_gate` |
| asserted pass without evidence | `hard_gate` |
| artifact hash mismatch | invalid input or `hard_gate`, consistently documented |
| policy hash mismatch | `hard_gate` |
| missing human reviewer | `hard_gate` |
| zero similarity candidates | no independent pass |
| material match unresolved | `hard_gate` |
| material match disposition recorded | category remains evaluable |
| high score with legitimate quotation | no automatic plagiarism finding |
| low score with admitted unattributed idea use | `hard_gate` |
| self-overlap undisclosed | `hard_gate` |
| prior publication disclosed and allowed | category may pass |
| fabricated citation | `hard_gate` |
| AI use required but undisclosed | `hard_gate` |
| confidential manuscript sent to unauthorized adapter | `hard_gate` before network access |
| rights or consent missing | `hard_gate` |
| authorized human deny determination | `deny`, exit `1` |
| raw PDF without extracted text | `hard_gate` |
| destination profile cannot weaken core policy | core finding retained |
| deterministic repeated run | stable ordered findings |

Also retain all existing substrate and institutional-policy fixture tests.

## 12. Security And Privacy Requirements

- No network access in the core evaluator.
- Never log API keys, credentials, or full confidential text.
- Bound excerpts by characters and lines; make the limit configurable downward.
- Prefer source hashes and locations over copied source content.
- Write output atomically through a temporary file and rename.
- Create review/report files with user-only permissions where supported.
- Reject symlink traversal outside an explicitly supplied corpus root.
- Do not follow recursive directory links by default.
- Apply file-count, file-size, and total-corpus-size limits.
- Record skipped and unreadable files as coverage limitations.
- Treat parser failure as a gate, not as evidence that content is clean.

## 13. Documentation Deliverables

Update:

- `README.md` with CLI examples and the human-review limitation;
- `MOU.md` only if semantics change;
- `roles/publication-gatekeeper.md` with operational inputs and outputs;
- `roadmap/ROADMAP.md` as work packages land;
- `examples/full-path-public-safe-artifact/` with a complete passing review and
  at least one hard-gated review;
- `sources/academic-publication-integrity.md` when source guidance changes.

Document the difference among:

- textual similarity;
- unattributed overlap;
- text recycling;
- duplicate or redundant publication;
- fabrication and falsification;
- honest error;
- policy noncompliance;
- a formal misconduct determination.

## 14. Implementation Discipline For Coding Models

For each work package:

1. inspect the current dirty worktree and preserve unrelated changes;
2. implement only that package;
3. add or update tests in the same change;
4. run formatting, unit tests, integration tests, schema checks, and
   `claimwright check .`;
5. review `git diff --check` and the scoped diff;
6. report limitations and remaining hard gates;
7. do not commit, push, publish, upload a manuscript, or call a remote
   similarity service unless separately authorized.

Prefer small commits when publication is authorized later:

```text
refactor(cli): preserve substrate check behind command parser
feat(publication): validate integrity review records
feat(publication): bind reviews to artifact and policy hashes
feat(publication): evaluate hard gates and render reports
feat(similarity): import candidate reports
feat(similarity): add offline local candidate discovery
feat(publication): enforce destination profiles
docs(publication): add integrity-gate workflow
```

## 15. Definition Of Done

The implementation is complete when:

- every public-release artifact check is bound to exact artifact and policy
  hashes;
- all ten integrity categories are enforced;
- similarity candidates retain method and corpus limitations;
- every material match requires human disposition;
- automated code never declares misconduct or originality;
- absent, malformed, stale, or inconsistent evidence fails closed;
- reports are deterministic, versioned, and CI-consumable;
- confidential manuscripts remain local unless explicit authority and a
  reviewed adapter policy permit otherwise;
- all required tests pass;
- documentation explains limitations;
- final human publication approval remains a separate recorded action.

# Publication gate consumers

ClaimWright's `publication check` report is an offline evidence surface for
GroundRecall, CiteGeist, and accountable publication workflows. Consumers may
store and display findings, route unresolved work for human review, and retain
the report with the release record.

The `decision` is not a permission grant. In particular, consumers must never
convert `hard_gate` to `allow`, infer originality from a similarity score, or
treat a finding as a misconduct adjudication. A separate human publication
approval remains required after an integrity `pass`.

The versioned fixture at
`fixtures/groundrecall/publication_gate_mcp_responses.json` defines the planned
future `claimwright.publication_gate` MCP response shape. It is a fixture only;
no MCP server or network adapter is implemented by this roadmap.

CI should propagate ClaimWright exit status unchanged: 0 permits the next
review stage, while 1 (hard gate/deny), 2 (invalid input), and 3 (I/O failure)
stop the workflow.

## Running and validating the gate

Run commands from the repository root; the gate binds the current
`policies/academic_publication.yaml` file. Examples:

```sh
cargo test --offline --manifest-path tools/claimwright/Cargo.toml
cargo fmt --manifest-path tools/claimwright/Cargo.toml -- --check
cargo run --manifest-path tools/claimwright/Cargo.toml -- publication check \
  --artifact fixtures/publication/passing-artifact.txt \
  --review fixtures/publication/passing-review.json --format json
cargo run --manifest-path tools/claimwright/Cargo.toml -- publication similarity generate \
  --artifact draft.md --comparison-corpus reviewed-sources --output candidates.json
```

The passing fixture is synthetic test data, not a review of a real publication.
`init-review --similarity-report candidates.json` carries candidates requiring
classification into the initialized review. `check --similarity-report` binds
the candidate report to the artifact. Unknown materiality and unresolved material
matches keep the gate closed. Exact-match locations use zero-based token ranges;
near-match locations are document-level discovery hints. Unreadable corpus files
fail the scan instead of silently disappearing from its coverage.

Destination profiles use a strict, flat `key: value` format, with full-line
comments allowed. This is not a general YAML parser. Example:

```yaml
schema_version: claimwright.destination_policy.v1
destination: example-journal
policy_version: v1
require_ai_disclosure: true
require_prior_publication_disclosure: true
```

The review must name that destination and policy version. Unknown/duplicate keys,
unsupported schema versions, and malformed booleans are rejected. Requirements
can add findings but cannot weaken an existing denial or core-policy gate.

## Verification limits

The checker validates declared evidence and review metadata; it does not
authenticate a human signature, independently inspect every cited source, prove
originality, or determine whether a disclosure is substantively adequate.
Destination evidence checks are presence checks, not editorial judgments.
UTF-8 text is hash-bound as supplied (including HTML markup). Non-text artifacts
are identified and hashed but remain gated: this version cannot verify their
extracted text. Review timestamps support calendar dates and UTC offsets, with
whole-second future checks; leap-second notation is rejected.

Regression tests cover initialization, overwrite protection, pass/deny retention,
destination identity and version, unresolved/unknown similarity, stale hashes,
invalid arguments, I/O errors, and malformed timestamps. These tests establish
the tested behavior, not a security certification.

# Publication gate validation — 26 September 2026

Release preparation found and corrected decision downgrading, incomplete
destination binding, ignored similarity imports at initialization, unknown
similarity classifications passing the gate, malformed option handling, incorrect
I/O exit statuses, and incomplete timestamp checks. Text-hash mismatches and
non-text artifacts now remain gated. Destination profiles reject unknown and
duplicate keys, and nested review objects reject undeclared fields.

Validation on the prepared revision:

- `cargo test --offline --manifest-path tools/claimwright/Cargo.toml`: **23 passed**
  (11 unit tests, 3 existing CLI tests, 9 publication CLI integration tests).
- `cargo fmt --manifest-path tools/claimwright/Cargo.toml -- --check`: passed.
- `bash -n examples/publication-gate-ci.sh`: passed.
- `git diff --check`: passed.

The CLI integration tests exercise synthetic pass, hard-gate and deny records,
initialization and overwrite refusal, destination profiles, similarity imports
and generation, malformed arguments, missing files, artifact/text bindings,
non-text gating, calendar dates, offsets and reviewer placeholders. The CI
workflow repeats tests and formatting checks on pushes and pull requests.

This is software regression evidence, not a substantive review of a publication,
a security audit, or proof that human approval metadata is authentic. See
[consumer documentation](publication-gate-consumers.md) for the supported input
formats and verification limits. OpenAI Codex assisted inspection, fixes, tests
and documentation under the maintainer's instruction to test and publish the update.

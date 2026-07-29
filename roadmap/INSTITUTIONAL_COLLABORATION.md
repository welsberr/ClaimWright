# Institutional Collaboration Policy Roadmap

Date: 2026-07-29
Status: companion implementation plan

The canonical cross-repository implementation plan is GroundRecall's
`docs/institutional-federation-implementation-roadmap.md`. This file records the
parts owned by ClaimWright so that its policy work can be implemented and
reviewed in this repository without redefining GroundRecall's policy-plugin
contract.

GroundRecall is authoritative for decision points, request/decision schemas,
composition, release levels, enforcement behavior, and policy coverage.
ClaimWright supplies one configurable collaborative and institutional policy
stance that conforms to that contract.

## Intended Policy Modules

Add ClaimWright policy content as separable modules with stable IDs and
versions:

1. `collaboration.contribution`
   - attribution;
   - destination scope and release classification;
   - contribution intent;
   - separation of contribution and approval;
   - rejected/deferred contribution preservation.
2. `collaboration.review`
   - risk-based reviewer roles and quorum;
   - conflict of interest;
   - minority/dissent preservation;
   - appeal and re-review;
   - review receipt requirements.
3. `institutional.prior_work`
   - prior-work review before costly or durable initiatives;
   - preservation of negative and inconclusive results;
   - review-gated duplicate/related-work findings;
   - proportional override with rationale.
4. `institutional.discovery`
   - least-disclosure federation catalogs;
   - protected membership/topic inference;
   - subscription purpose and scope;
   - high-impact routing and notification proportionality.
5. `institutional.stewardship`
   - explicit stewardship rather than inference from activity;
   - custody handoff;
   - orphan escalation;
   - tenancy departure;
   - instance retirement;
   - confidentiality and attribution survival.
6. `institutional.privacy_and_fairness`
   - purpose limitation and minimization;
   - anti-surveillance controls for expertise and governance views;
   - correction and challenge mechanisms;
   - incident compartment handling.
7. `institutional.release`
   - license and attribution review;
   - provenance visibility;
   - redaction/declassification;
   - public defensibility;
   - withdrawal and supersession distinct from erasure.

Modules may be composed additively. Conservative GroundRecall composition
remains controlling: deny/hard-gate dominates, obligations and reviewers
accumulate, and the most restrictive release level wins.

## Stable Reason-Code Families

ClaimWright should emit structured reason codes for:

- `collaboration.missing_destination_scope`;
- `collaboration.unclassified_contribution`;
- `collaboration.contributor_reviewer_conflict`;
- `collaboration.insufficient_review_quorum`;
- `collaboration.minority_position_suppression`;
- `collaboration.negative_result_loss`;
- `institutional.missing_steward`;
- `institutional.unauthorized_custody_transfer`;
- `institutional.tenancy_departure_without_handoff`;
- `institutional.protected_catalog_disclosure`;
- `institutional.incident_compartment_leakage`;
- `institutional.expertise_view_surveillance_risk`;
- `institutional.stale_high_impact_knowledge`;
- `institutional.missing_attribution`;
- `institutional.missing_or_incompatible_license`.

Each reason code must map to:

- a default decision;
- one or more inspectable obligations;
- zero or more required reviewer roles;
- applicable GroundRecall decision points and action names;
- configurable scope/risk conditions;
- stable audit tags;
- tests and example fixtures.

## Role Cards

Add:

- `group-contributor`;
- `group-reviewer`;
- `prior-work-reviewer`;
- `scope-steward`;
- `records-custodian`;
- `tenancy-handoff-reviewer`;
- `incident-sanitization-reviewer`.

Each role card must state inputs, permitted actions, prohibited actions,
required handoffs, protected data, and human approval boundaries. Roles express
review obligations; they do not grant GroundRecall federation permissions.

## Checks

Add machine-readable checks for:

- contribution preflight and completion;
- prior-work review;
- multi-party promotion/adjudication;
- federation catalog publication;
- subscription creation and high-impact routing;
- custody transfer and tenancy departure;
- instance retirement;
- orientation/expertise/governance-view privacy;
- public/internal release-pack generation and withdrawal.

Checks must remain proportional. Low-risk private exploration should not inherit
institution-wide public-release ceremony.

## Compatibility Fixtures

For every institutional action in the GroundRecall roadmap:

- add a representative request fixture;
- add the expected ClaimWright decision fixture;
- include reason codes, obligations, required reviewers, release caps,
  redactions, confidence effects, and audit tags where applicable;
- include allow, review, hard-gate, and policy-composition cases;
- prove request metadata alone does not grant authority.

The ClaimWright checker must validate duplicate IDs, unknown referenced roles,
unknown reason codes, missing versions, and malformed action mappings.

## Implementation Sequence

Follow the GroundRecall work-package IDs:

1. `IF-00`: contract fixtures and checker validation.
2. `IF-01`: scope/work and negative-result policy.
3. `IF-02`: contribution, review, stewardship policy and roles.
4. `IF-03`: complete provider fixtures and institutional reason-code coverage.
5. `IF-04`: prior-work obligation and proportional overrides.
6. `IF-05`: catalog least-disclosure and inference-risk policy.
7. `IF-06`: subscription purpose, routing, and acknowledgement policy.
8. `IF-07`: quorum, independence, dissent, conflict, and appeal policy.
9. `IF-08`: custody, tenancy, retirement, retention, and erasure separation.
10. `IF-09`: institutional-view privacy, fairness, and correction policy.
11. `IF-10`: license, attribution, release, and withdrawal policy.
12. `IF-11`: MCP policy/check fixtures.
13. `IF-12`: conformance scenarios and paper evidence.

Do not implement ClaimWright action mappings ahead of the corresponding
GroundRecall contract fixture. Commit each ClaimWright slice with the matching
`IF-##` identifier and report its GroundRecall dependency.

### IF-06 Status

Status (2026-07-29): the collaboration policy now covers GroundRecall's
file-based incremental subscription transport. The policy requires scope-steward
review for subscription management, incremental export, and incremental import;
it records obligations for subscription purpose, bounded change scope, verified
acknowledgement, and prevention of silent auto-promotion from quarantined
bundles.

The corresponding GroundRecall dependency is `IF-06` in
`docs/institutional-federation-implementation-roadmap.md`, where signed,
cursor-bounded change bundles and replay-safe quarantine imports are
implemented.

### IF-07 Status

Status (2026-07-29): the collaboration policy now covers GroundRecall's
multi-party review and federation-feedback slice. The policy requires
scope-steward and group-reviewer review for group review, high-risk acceptance,
and federation feedback, with obligations for risk-based quorum, independent
high-risk review, dissent/minority-position preservation, conflict disclosure,
and appeal preservation.

The corresponding GroundRecall dependency is `IF-07` in
`docs/institutional-federation-implementation-roadmap.md`, where generalized
review receipts, quorum evaluation, signed feedback bundles, and unresolved
disagreement summaries are implemented as a partial slice.

### IF-08 Status

Status (2026-07-29): the collaboration policy now hard-gates custody transfer
and instance retirement with scope-steward, records-custodian, and
tenancy-handoff-reviewer review. The policy adds obligations for complete
custody handoff, least-necessary retention, confidentiality survival, erasure
separation, attribution/correction rights, and orphan escalation.

The corresponding GroundRecall dependency is `IF-08` in
`docs/institutional-federation-implementation-roadmap.md`, where orphan
stewardship reports, tenancy departure dry-runs, instance retirement dry-runs,
and release-broadening custody guards are implemented as a partial slice.

### IF-09 Status

Status (2026-07-29): the collaboration policy now covers institutional-view
privacy and fairness for scope orientation, stewardship views, and change-impact
reports. The policy requires purpose limitation, minimization, explicit
basis-labeling, activity-ranking suppression, and correction-path preservation.

The corresponding GroundRecall dependency is `IF-09` in
`docs/institutional-federation-implementation-roadmap.md`, where release-capped
orientation, impact, governance, and stewardship views are implemented as a
partial slice.

### IF-10 Status

Status (2026-07-29): the collaboration policy now hard-gates public/internal
release packs and withdrawal with publication-gatekeeper and scope-steward
review. The policy requires license compatibility, attribution preservation,
release authority, provenance visibility enforcement, and withdrawal review.

The corresponding GroundRecall dependency is `IF-10` in
`docs/institutional-federation-implementation-roadmap.md`, where deterministic
signed release packs and signed withdrawal notices are implemented as a partial
slice.

### IF-11 Status

Status (2026-07-29): ClaimWright now carries stable GroundRecall MCP policy
response fixtures. The fixture examples label policy findings as findings, not
permission grants, and include no-write contribution proposal, explicit
stewardship/no-ranking, and publication-gatekeeper scenarios.

The corresponding GroundRecall dependency is `IF-11` in
`docs/institutional-federation-implementation-roadmap.md`, where prior-work,
catalog discovery, subscription status, impact report, stewardship/orphan
review, and no-write contribution proposal MCP tools are implemented as a
partial slice.

## Acceptance

ClaimWright's institutional policy work is complete when:

- all policy files have stable IDs and versions;
- all role and reason references validate;
- representative fixtures conform to the GroundRecall contract;
- collaboration checks distinguish findings from permission grants;
- contribution, review, stewardship, discovery, continuity, privacy, and
  release concerns have tested policy coverage;
- policies remain configurable by risk and scope;
- no ClaimWright file claims to be authoritative for GroundRecall permissions
  or plugin schemas.

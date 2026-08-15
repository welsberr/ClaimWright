# Bounded Decision-Challenge Roadmap

**Status:** draft for cross-repository review  
**Policy owner:** ClaimWright  
**Primary contract consumer:** GroundRecall  
**Initial downstream consumers:** CiteGeist, Epistemap, Didactopus,
SciSiteForge, GenieHive, doclift, and ThreeGate

## Purpose

Substantial decisions should receive one deliberate challenge before they
become durable, public, costly, destructive, or authority-bearing:

> What are the most plausible ways this decision could be materially wrong?
> What evidence would discriminate those alternatives, and which check is
> worth performing now?

The useful part of this rule is directed search for disconfirming evidence. The
danger is exhaustive doubt, manufactured alternatives, repeated review of the
review, and latency that makes the policy unusable. This roadmap therefore
defines a bounded `decision_challenge` artifact, not a general instruction to
question every action indefinitely.

## Authority Boundaries

ClaimWright owns:

- the portable decision-challenge policy;
- trigger and review-level vocabulary;
- the versioned artifact schema;
- default bounds, stop reasons, and anti-regress rules;
- ClaimWright checker and policy-plugin findings.

ClaimWright does not own:

- GroundRecall's policy-plugin request or decision format;
- bibliographic identity or source-support decisions in CiteGeist;
- graph assessment or posterior calculations in Epistemap;
- learner mastery or instructional decisions in Didactopus;
- public-site deployment authority in SciSiteForge or site repositories;
- model routing, credentials, or budget enforcement in GenieHive;
- extraction truth or source permissions in doclift;
- execution approval or security boundaries in ThreeGate.

GroundRecall's `docs/policy-plugin-spec.md` remains authoritative for governed
memory operations. A ClaimWright decision challenge may produce obligations or
review findings through that interface, but it is not an independent
permission grant.

## Operational Rule

Before a configured substantial decision is committed, published, promoted,
or executed:

1. classify the decision's review level;
2. perform at most one challenge pass for that decision version;
3. name no more than three plausible, decision-relevant failure modes;
4. identify the cheapest available evidence that could discriminate each one;
5. check the highest-value item that fits the review budget;
6. record `proceed`, `revise`, `defer`, or `escalate` plus residual uncertainty;
7. stop unless new material evidence or a materially changed decision creates a
   new decision version.

A possible failure mode qualifies only when:

- there is a plausible basis for it;
- it could materially change the action, scope, release level, or conclusion;
- available evidence could distinguish it from the working assumption.

The rule does not require inventing remote possibilities or treating settled
evidence and speculative objections as equally strong.

## Review Levels

| Level | Intended use | Bound |
| --- | --- | --- |
| `none` | reversible, local, mechanical, read-only, or already deterministically checked | no challenge artifact required |
| `quick` | ordinary consequential but reversible decisions | one failure mode, one cheap check, nominal one-minute budget |
| `standard` | durable, public, costly, novel, or evidence-sensitive decisions | up to three failure modes, one or two checks, nominal five-minute budget |
| `escalated` | destructive, security/privacy/legal sensitive, high-blast-radius, or exceptional-risk decisions | independent reviewer or explicit human authority; repository policy sets the budget |

Repository profiles may tighten these defaults. They must not silently remove
the depth, count, or reopening limits.

## Trigger Codes

The initial stable trigger vocabulary should include:

- `difficult_to_reverse`;
- `destructive_action`;
- `durable_memory_change`;
- `public_release`;
- `release_scope_expansion`;
- `authority_or_capability_expansion`;
- `private_or_restricted_data`;
- `high_reputational_impact`;
- `high_resource_cost`;
- `novel_or_unfamiliar_path`;
- `thin_or_contested_evidence`;
- `claim_or_mastery_promotion`;
- `source_support_or_identity_decision`;
- `security_boundary_change`.

The classifier should be deterministic where request metadata is sufficient.
Model assistance may propose failure modes or checks, but must not decide its
own authority or convert missing metadata into permission.

## Proposed Artifact Contract

ClaimWright should add a JSON Schema for
`claimwright.decision_challenge.v1`. The minimum conceptual shape is:

```yaml
schema_version: claimwright.decision_challenge.v1
challenge_id: dc_example
decision_id: decision_example
decision_version: 1
subject_id: agent_or_human_id
action: publish
review_level: standard
trigger_codes: [public_release, difficult_to_reverse]
decision_summary: Publish the reviewed artifact.
key_assumptions:
  - Public export contains only reviewed public-safe records.
failure_modes:
  - failure_mode_id: fm_private_path
    hypothesis: A rendered artifact still exposes a local path.
    plausibility_basis: The build uses local source manifests.
    material_consequence: Public metadata leakage.
    decision_changing: true
    discriminating_evidence: Scan the staged public tree.
    cheapest_check: Run the repository public-surface scanner.
    check_status: completed
    result_ref: artifact:public-scan.json
outcome: proceed
residual_uncertainty:
  - Scanner coverage is limited to configured patterns.
stop_reason: one_pass_complete
review_state: reviewed
authority: Review evidence only; not an autonomous permission grant.
```

The schema should require bounded arrays, stable IDs, an explicit outcome,
review state, and an authority statement. It should store concise public
reasoning products, not hidden chain-of-thought. Private evidence should be
referenced by scoped artifact ID rather than copied into public findings.

Initial stop reasons:

- `not_triggered`;
- `no_plausible_decision_changing_failure_mode`;
- `highest_value_check_completed`;
- `one_pass_complete`;
- `budget_exhausted`;
- `evidence_unavailable`;
- `human_authority_required`;
- `decision_revised`;
- `decision_deferred`.

## Anti-Regress And Performance Invariants

1. **Depth one:** a decision challenge does not challenge itself.
2. **One pass per version:** repeated invocation returns the existing artifact
   unless the caller supplies a new decision version and material-change reason.
3. **Maximum three:** no more than three failure modes may be retained.
4. **Decision relevance:** checks with no plausible decision-changing outcome
   are recorded as unnecessary, not executed.
5. **Bounded work:** every non-escalated profile has a time, tool-call, or cost
   budget and a visible stop result.
6. **Status-quo symmetry:** retaining the current state may itself be the
   substantial decision; inaction receives no automatic evidential privilege.
7. **No manufactured doubt:** low-plausibility alternatives cannot force an
   evidence burden merely because they can be imagined.
8. **No authority laundering:** completing a checklist does not prove the
   decision correct or permit an action blocked by another policy.
9. **No hidden mutation:** challenge generation and evidence lookup are
   read-only unless a separately authorized action is invoked.
10. **No private-data copying:** findings carry scoped references and release
    levels rather than unrestricted evidence text.

## Repository Alteration Inventory

The paths below are implementation anchors, not permission to edit every file
listed. Each repository work package should confirm the smallest viable set
against its then-current tree and keep its domain authority intact.

| Repository | Planned alteration surface | Candidate implementation anchors | Candidate verification anchors |
| --- | --- | --- | --- |
| ClaimWright | Own the portable schema, policy vocabulary, trigger classifier, bounded planner, and validator. | `schemas/decision-challenge.schema.json`, `policies/principles.yaml`, `policies/enforcement.yaml`, `checks/pre_action.yaml`, `checks/post_action.yaml`, `tools/claimwright/src/` | Rust unit/CLI tests plus valid, invalid, replay, and recursion fixtures |
| GroundRecall | Adapt ClaimWright findings to the existing generic policy-plugin contract and retain release-scoped, idempotent receipts. | `src/groundrecall/policy.py`, lifecycle write/export surfaces, artifact schemas, and MCP inspection surfaces | `tests/test_policy_plugins.py` plus affected promotion, federation, export, and MCP tests |
| CiteGeist | Provide bounded bibliographic and claim-support evidence checks without promoting search results. | `src/citegeist/claim_support.py`, `sources.py`, `resolve.py`, `mcp.py`, and storage only if durable result references are needed | claim-support, resolution, source-plugin, batch, and MCP tests |
| Epistemap | Provide bounded alternative, sensitivity, contradiction, prior, and temporal analyses as evidence rather than policy decisions. | `src/epistemap/evidence.py`, `bayesian.py`, `temporal.py`, `epistemic.py`, `assessment_manifest.py`, and `mcp.py` | evidence-ledger, Bayesian, temporal, assessment-manifest, and MCP tests |
| Didactopus | Apply challenges only at mastery/source-support promotion, pack publication, and instructional-policy boundaries. | `src/didactopus/mastery_ledger.py`, `review_actions.py`, `notebook_promotion_pipeline.py`, `pack_validator.py`, `rule_policy.py`, and `mcp.py` | promotion, mastery, pack-validation, learner-session, and MCP tests; add focused fixtures if absent |
| SciSiteForge | Attach challenge references to public-surface and deployment preflight for scope-broadening actions. | existing public-surface guardrails, site configuration/CLI implementation, `docs/PUBLIC_SURFACE_GUARDRAILS.md`, and `docs/AGENTIC_OS.md` | `tests/test_scisiteforge.py` plus public/private scope and destructive-sync fixtures |
| GenieHive | Review provider, budget, credential, tenancy, model-swapping, and native-adapter policy changes, not routine approved routing. | `src/geniehive_control/providers.py`, `budgeting.py`, `auth.py`, `config.py`, `routing.py`, `request_policy.py`, and admin surfaces | provider, budget, authorization, config, audit, routing, and admin CLI tests |
| doclift | Review OCR/lossy-path activation, public copyright-sensitive export, and quality-threshold changes. | `src/doclift/convert.py`, `legacy_doc.py`, `wordperfect.py`, `inspect.py`, `okf_export.py`, and `mcp.py` | conversion, legacy-document, quality/fallback, export, and MCP tests |
| ThreeGate | Bind capability and security-boundary expansion to existing human approval without creating executable challenge artifacts. | tool-request/result schemas, execution policy/backend surfaces, `docs/change_management.md`, `docs/threat-model.md`, and external-function documentation | schema fixtures and adversarial tests for allowlist, external-function, and heavy-execution changes |

### Change-Set Discipline

- Land DC0 and DC1 in ClaimWright before any consumer freezes a local schema.
- Give each downstream repository its own reviewable commit or pull request;
  do not combine cross-repository runtime changes into one opaque handoff.
- Add a short local roadmap link when a repository begins its work package,
  while keeping this document authoritative for the shared contract.
- Pin the ClaimWright contract version in fixtures or adapter configuration;
  do not depend on an unversioned sibling checkout.
- Preserve existing local changes and branch state. A dirty repository must be
  isolated or reconciled before its work package begins.
- Require the portable DC5 fixtures to pass before enabling enforcement beyond
  shadow or advisory mode.
- Keep rollback simple: disabling the adapter restores the prior repository
  behavior while retaining already-written audit references.

### Dependency Flow

```text
ClaimWright DC0 contract
        |
        +--> ClaimWright DC1 policy/checker
        |            |
        |            +--> GroundRecall DC2 adapter/receipts
        |            |            |
        |            |            +--> durable-memory consumers
        |            |
        |            +--> direct local adapters where no durable receipt is needed
        |
        +--> CiteGeist and Epistemap DC3 evidence providers
                         |
                         +--> Didactopus and other domain consumers DC4

All paths converge on DC5 conformance before enforcement rollout.
```

This ordering prevents consumers from defining incompatible meanings for
`review_level`, `outcome`, or `stop_reason`, while allowing evidence-provider
prototypes to proceed in parallel once the DC0 schema is stable.

## Delivery Sequence

### DC0: Contract And Golden Fixtures

**Repository:** ClaimWright  
**Dependencies:** none

Tasks:

- add the versioned decision-challenge schema;
- add trigger, level, outcome, and stop-reason policy vocabularies;
- create golden fixtures for `none`, `quick`, `standard`, and `escalated`;
- include failure fixtures for recursive depth, four or more failure modes,
  missing stop reason, fabricated evidence claims, and unchanged-version replay;
- document the boundary between concise rationale and hidden chain-of-thought.

Acceptance:

- schema validation rejects unbounded and recursive records;
- every fixture states whether a separate policy grants or blocks action;
- no fixture treats review completion as correctness or permission.

### DC1: ClaimWright Policy And Checker

**Repository:** ClaimWright  
**Depends on:** DC0

Tasks:

- add a `bounded_decision_challenge` principle;
- add pre-action trigger classification and post-action outcome recording;
- add configurable review profiles to enforcement policy;
- extend the Rust checker from substrate presence to schema and cross-field
  validation for challenge fixtures;
- emit stable reason codes and obligations compatible with GroundRecall's
  bounded decision vocabulary;
- add CLI planning and validation commands that are side-effect free.

Initial policy mapping:

- missing `quick` challenge: advisory or `require_review`;
- missing `standard` challenge: `soft_gate` by default;
- missing `escalated` challenge or required independent review: `hard_gate`;
- malformed, recursive, or falsely completed evidence record: `hard_gate` at
  protected enforcement surfaces.

### DC2: GroundRecall Adapter And Durable Receipt

**Repository:** GroundRecall  
**Depends on:** DC0 and DC1

Tasks:

- carry challenge metadata through the existing ClaimWright directory adapter;
- map challenge findings into `reasons`, `obligations`,
  `required_reviewers`, and `audit_tags` without changing the generic plugin
  contract;
- add a durable, release-scoped decision-challenge receipt or reference from
  existing review receipts;
- enforce idempotency by `(decision_id, decision_version, policy_version)`;
- keep evidence artifacts private unless their release level permits export;
- expose read-only MCP inspection for challenge status without granting write
  or promotion authority.

Acceptance:

- policy composition remains conservative;
- hard gates block before memory mutation;
- repeated identical evaluation creates no duplicate durable receipt;
- public export omits private evidence while retaining inspectable reason codes.

### DC3: Evidence And Sensitivity Providers

**Repositories:** CiteGeist and Epistemap  
**Depends on:** DC0

CiteGeist tasks:

- accept a bounded evidence-check request containing a claim/source question,
  maximum query count, and allowed source routes;
- return accepted, rejected, unresolved, and negative search results with
  provenance;
- distinguish source identity, topical relevance, and actual claim support;
- never promote a candidate citation merely because it was returned by a
  decision challenge.

Epistemap tasks:

- accept bounded failure-mode or alternative identifiers as parameters for
  sensitivity, contradiction, prior-profile, or temporal-tenability analyses;
- return derived evidence summaries and limitations, not policy decisions;
- preserve effective sample size, interval width, prior sensitivity,
  provenance, and temporal scope;
- use `docs/performance-decision.md` as a reference implementation of explicit
  reopening evidence and stop criteria.

Acceptance:

- both providers honor caller budgets;
- missing or inconclusive evidence is a valid result;
- bibliographic proximity, graph centrality, and posterior support never become
  automatic source truth or permission.

### DC4: Domain Consumers

**Repositories:** Didactopus, SciSiteForge, GenieHive, doclift, and ThreeGate  
**Depends on:** DC0; DC2 or a local ClaimWright adapter where durable policy is
required

Didactopus:

- trigger standard review for learner-mastery promotion, source-support
  promotion, pack publication, and material instructional-policy changes;
- expose a learner-friendly challenge scaffold at qualified conclusions;
- do not challenge every mentor turn, practice response, or reversible hint;
- keep learner challenge work as draft evidence until reviewed.

SciSiteForge:

- trigger standard or escalated review for global publish scope, new public
  route classes, allowlist broadening, public/private reclassification, and
  destructive synchronization;
- attach the challenge reference to public-surface guardrail and deployment
  reports;
- retain existing deterministic scans as evidence checks rather than replacing
  them with prose review.

GenieHive:

- trigger review for provider enablement, cost-ceiling changes, credential or
  tenancy model changes, active model swapping, and native-provider adapter
  selection;
- exclude routine routing among already approved healthy targets;
- use existing audit, benchmark, health, and budget data as discriminating
  evidence before requesting new model work.

doclift:

- trigger review for OCR activation, lossy fallback selection, public export of
  copyright-sensitive material, and changes to confidence/quality thresholds;
- exclude ordinary conversions that remain within a reviewed profile and pass
  deterministic quality checks;
- preserve extraction uncertainty and source-permission limitations.

ThreeGate:

- require escalated review for capability expansion, allowlist broadening,
  policy changes, external functions, and TOOL-EXEC-Heavy enablement;
- reuse existing human approval rather than adding a duplicate prompt to every
  approved routine tool request;
- keep decision-challenge artifacts one-way and non-executable.

### DC5: Cross-Repository Conformance

**Repositories:** all participating repositories  
**Depends on:** DC1-DC4

Create one portable fixture pack covering:

- reversible local edit (`none`);
- bounded bibliography lookup (`quick`);
- durable memory promotion (`standard`);
- public site release (`standard`);
- high-cost provider enablement (`standard` or `escalated`);
- destructive synchronization (`escalated`);
- security-boundary expansion (`escalated`);
- unchanged decision replay (idempotent);
- new material evidence (new decision version);
- manufactured-doubt proposal (rejected as implausible or non-discriminating).

Acceptance:

- all consumers agree on schema version, trigger codes, review levels, outcome,
  and stop reason;
- domain repositories retain their own authority;
- no conformance path creates nested challenges;
- deterministic classification and validation remain fast enough for local CLI
  use without a model call;
- provider checks respect explicit query, time, and cost budgets.

## Rollout Plan

1. **Shadow mode:** classify and record what would have triggered without
   blocking. Run for at least 50 substantial decisions or two weeks, whichever
   is longer.
2. **Advisory mode:** show quick and standard findings; retain existing hard
   gates unchanged.
3. **Soft-gate mode:** require explicit disposition for missing standard
   challenges on selected durable and public actions.
4. **Hard-gate mode:** enable only for already protected action classes such as
   destructive operations, private-data publication, security-boundary
   expansion, and explicit exceptional-risk paths.
5. **Quarterly policy audit:** review trigger precision, latency, missed defects,
   and stale profiles. Do not recursively challenge individual completed
   challenge artifacts during this audit.

## Evaluation Metrics

Track by repository and review level:

- trigger rate and false-trigger disposition;
- median and tail review latency;
- model, tool-call, query, and compute cost;
- percentage of challenges that revise, defer, or escalate a decision;
- defects or policy violations caught before action;
- post-action defects that the challenge missed;
- inconclusive evidence-check rate;
- duplicate/replayed challenge rate;
- nested challenge count, which must remain zero;
- human override and independent-review burden;
- private/public leakage incidents associated with challenge artifacts.

The policy is successful only if it reduces consequential errors without
making ordinary work materially slower. If it adds ceremony without changing
decisions or catching failures, narrow the triggers or lower the review level;
do not respond by adding another review layer.

## Explicit Non-Goals

- exhaustive enumeration of every possible error;
- recursive review of review artifacts;
- mandatory model calls for trigger classification;
- automatic claim truth, citation support, learner mastery, or publication
  approval;
- replacement of deterministic tests, scanners, backups, or human authority;
- forcing all repositories to depend directly on ClaimWright runtime code;
- storing private evidence in public policy findings;
- using speculative objections to manufacture doubt around well-supported
  conclusions.

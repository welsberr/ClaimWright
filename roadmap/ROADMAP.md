# ClaimWright Roadmap

## Top Priority: Policy Plugin Compatibility

ClaimWright should be usable as one policy framework among many for governed
memory layers such as GroundRecall. The integration target is not bespoke
GroundRecall code that embeds ClaimWright semantics everywhere. The target is a
bounded GroundRecall-owned policy-plugin contract that lets ClaimWright answer
standard decision questions about read, propose, promote, publish, federate,
adjudicate, redact, delete, cite, and act operations.

GroundRecall is the source of truth for that interface. ClaimWright should
conform to GroundRecall's `docs/policy-plugin-spec.md` rather than defining a
parallel plugin format.

Near-term ClaimWright work should therefore make its policy content easier to
consume through adapters:

- assign stable IDs and versions to principles, claim states, enforcement
  defaults, checklists, role cards, and publication gates;
- expose machine-readable findings with decision values compatible with
  `allow`, `require_review`, `soft_gate`, `hard_gate`, and `deny`;
- distinguish policy findings from permission grants;
- express obligations, required reviewers, redactions, confidence effects,
  release caps, and audit tags explicitly;
- document how ClaimWright findings compose with organization, project,
  federation, legal/privacy, and publication policies.

GroundRecall now has an authoritative policy-plugin specification, schema
version validation, an initial generic policy-plugin boundary, and a
ClaimWright directory adapter. ClaimWright should treat that as the first
compatibility target while staying independent enough to serve other systems.

ClaimWright may be packaged as one plugin or as a collection of additive
plugins, for example:

- claim-state policy;
- citation-review policy;
- publication-gate policy;
- adversarial-review policy;
- private/public-boundary policy.

The collaborative group and institutional policy modules, stable reason-code
families, role cards, compatibility fixtures, and implementation order are
specified in
[`INSTITUTIONAL_COLLABORATION.md`](INSTITUTIONAL_COLLABORATION.md). Its
cross-repository work-package authority is GroundRecall's
`docs/institutional-federation-implementation-roadmap.md`.

## Implemented In This Draft

- IF-00 GroundRecall policy-compatibility fixture covering the institutional
  federation action map, with schema/action/duplicate-ID validation in the
  checker.
- IF-02 collaborative policy module, contribution checks, and group/stewardship
  role cards for scope, provenance, negative-result, review, and continuity
  discipline.
- IF-04 prior-work review policy rule requiring a bounded review query and
  preservation of negative or inconclusive results.
- IF-05 least-disclosure federation-catalog rule with receiver-side release
  caps and protected-topic inference safeguards.
- Human-readable MOU.
- Machine-readable principles, claim states, and enforcement defaults.
- Pre-action and post-action checklists.
- Initial agent role cards.
- Draft claim and citation schemas.
- Pennock scientific virtues source note.
- Minimal Rust policy-substrate checker.
- Public-safe artifact workflow example.

## Near-Term Capability

- Add deeper source notes for VERITIES and scientific-virtues RCR training modules.
- Add structured records for negative results and cross-disciplinary bridges.
- Add schema validation beyond presence checks.
- Add branch-comparison templates.
- Add stale-claim scanner interfaces for repositories and note stores.
- Add citation-library integration points for CiteGeist.
- Add a Model Context Protocol (MCP) adapter plan for assistant-facing
  ClaimWright checks.

## Later Capability

- Knowledge graph overlay for principle, claim, citation, and review interactions.
- Configurable enforcement engine with advisory, soft-gate, and hard-gate modes.
- Cost and assistant-availability estimator, including parallel work burden.
- Rust tools for fast local scanning.
- Integration with GroundRecall, CiteGeist, SciSiteForge, Didactopus, doclift, Epistemap, GenieHive, and llm-learning workflows.
- Full private-claim-to-public-artifact reference implementation.

## MCP Adapter Roadmap

Goal:
Make ClaimWright available to MCP-capable assistants as a policy/checking
surface without letting the adapter silently approve publication, promotion, or
high-risk actions.

Initial tools:

- `claimwright.load_policy`: return active principles, enforcement defaults,
  role cards, and claim-state definitions for the current workspace;
- `claimwright.pre_action_check`: classify a proposed action and return
  advisory, soft-gate, or hard-gate findings with required evidence;
- `claimwright.post_action_check`: inspect a draft artifact or changed-file set
  for unsupported claims, unresolved citations, private/public boundary issues,
  and missing review state;
- `claimwright.review_claims`: map claims to evidence classes and proposed
  lifecycle states;
- `claimwright.publication_gate`: produce a publication-readiness report that
  remains advisory unless the caller has explicit configured authority.

Design constraints:

- MCP results are policy findings, not autonomous permission grants.
- The adapter must preserve which policy file, role card, checklist, and schema
  version produced each finding.
- Hard-gate findings must be explicit and machine-readable so client agents can
  stop or escalate.
- The adapter must not read private repositories or note stores unless they are
  explicitly placed in scope by the caller.
- Integrations with GroundRecall and CiteGeist should exchange reviewed claim,
  citation, and source-support state by stable IDs rather than copying private
  evidence text unnecessarily.

Acceptance criteria:

- versioned MCP schemas and fixture responses exist for each initial tool;
- local checker results and MCP checker results agree on representative
  workspaces;
- tests cover public/private boundary findings, unsupported-claim findings,
  unresolved-citation findings, and hard-gate escalation;
- documentation explains how ClaimWright MCP findings should be consumed by
  GroundRecall, CiteGeist, Didactopus, and publication workflows.

# ClaimWright

ClaimWright is a human-agent research collaboration framework for grounded claims, adversarial review, and public-safe publication.

It helps researchers, writers, developers, and AI agents work from the same operating agreement: claims should be scoped, evidence should be traceable, citations should be reviewable, uncertainty should remain visible, and public artifacts should not outrun their grounding.

## Why It Exists

AI assistants can accelerate research and publication, but they can also create extra review work: unchecked claims, vague citations, hidden assumptions, overconfident summaries, and public-facing text that sounds better grounded than it is.

ClaimWright is designed to reduce that burden without lowering standards. It gives collaborators a shared structure for:

- turning private ideas into public-safe artifacts;
- keeping claims, citations, corrections, and negative results reviewable;
- making adversarial review part of normal work;
- protecting reputation when the work may be scrutinized closely;
- matching model, tool, and compute effort to task risk and complexity.

## What It Provides

ClaimWright combines a human-readable memorandum of understanding with machine-readable policy files and a small executable checker.

Current capabilities:

- **Collaboration MOU:** defines the working agreement between human researchers and AI assistants.
- **Claim lifecycle:** names claim states such as exploratory, supported, contested, stale, contradicted, public-safe, and private-only.
- **Confidence dimensions:** tracks source reliability, source access depth, claim-source fit, methodological strength, consensus status, adversarial robustness, currentness, and human review confidence.
- **Mixed enforcement model:** advisory for private exploration, soft gates for costly or durable actions, hard gates for public-release hazards.
- **Agent role cards:** defines responsibilities for claim auditing, adversarial review, citation review, synthesis mapping, knowledge-base maintenance, and publication gating.
- **Pre-action and post-action checks:** helps agents evaluate reversibility, evidence standards, public/private boundaries, capacity cost, assumptions, unresolved risks, and broader knowledge-base effects.
- **Citation review pattern:** keeps accepted, rejected, and unresolved citation candidates reviewable instead of silently discarding costly retrieval work.
- **Scientific virtues grounding:** incorporates Pennock-style scientific virtues as operational constraints on agent behavior.
- **Rust substrate checker:** verifies that the core policy files required by the framework are present.

## Use Cases

Use ClaimWright when AI-assisted work needs to be faster, but also more defensible.

Good first use cases:

- **Private claim to public artifact:** move a speculative or working claim through evidence review, adversarial challenge, citation review, and final human publication approval.
- **Citation-heavy research:** preserve why sources were accepted, rejected, or left unresolved, especially when relevance is not obvious from metadata alone.
- **Scientific or technical publishing:** prevent public claims from relying on stale, contradicted, unsupported, or private-only material.
- **AI-assisted literature synthesis:** require cross-field syntheses to show how knowledge from one domain does useful work in another.
- **Research memory maintenance:** mark related claims stale when new evidence appears and open follow-up tasks instead of silently letting weak premises propagate.
- **Agent workflow design:** give agents explicit roles, authority limits, review states, and escalation rules.
- **Responsible conduct training:** connect AI-assisted research practice to scientific virtues, provenance fairness, and evidence-based revision.

## Quick Start

1. Read the collaboration agreement:

   ```sh
   sed -n '1,220p' MOU.md
   ```

2. Inspect the machine-readable policy:

   ```sh
   sed -n '1,220p' policies/principles.yaml
   sed -n '1,220p' policies/claim_states.yaml
   sed -n '1,220p' policies/enforcement.yaml
   ```

3. Review the operational checks:

   ```sh
   sed -n '1,220p' checks/pre_action.yaml
   sed -n '1,220p' checks/post_action.yaml
   ```

4. Run the current policy-substrate check:

   ```sh
   cargo run --manifest-path tools/claimwright/Cargo.toml -- check .
   ```

Expected result:

```text
ClaimWright check passed: policy substrate is present.
```

## First Workflow To Try

Start with the full path from private claim to public-safe artifact:

```sh
sed -n '1,220p' examples/full-path-public-safe-artifact/README.md
```

That workflow exercises the core ClaimWright loop:

1. capture a private claim;
2. assign a claim state;
3. gather and review citations;
4. record confidence dimensions;
5. run adversarial review;
6. check stale or contradicted related claims;
7. choose a conservative, balanced, or expansive branch;
8. apply public/private and publication gates;
9. record final human approval.

## Repository Shape

| Path | Purpose |
|---|---|
| `MOU.md` | Human-readable collaboration memorandum |
| `policies/` | Machine-readable principles, enforcement defaults, and claim states |
| `roles/` | Agent role cards and authority boundaries |
| `checks/` | Pre-action and post-action review checks |
| `schemas/` | Draft schemas for structured records |
| `sources/` | Source notes grounding policy concepts |
| `tools/claimwright/` | Minimal Rust CLI for policy-substrate checks |
| `examples/` | Worked workflow examples |
| `roadmap/` | Implemented versus future capability |

## Enforcement Default

ClaimWright defaults to a mixed enforcement model:

- advisory for private exploratory work;
- soft gates for costly, long-running, durable, or capacity-threatening work;
- hard gates for public release, private-data exposure, fabricated or unverified citations, destructive irreversible actions, and stale or contradicted claims.

The default is meant to support serious work without turning every action into ceremony. Enforcement should become stricter as work approaches public release or reputational risk increases.

## Scientific Virtues

ClaimWright treats scientific virtues as operational constraints, not decorative values. The current policy is grounded in Robert T. Pennock's Scientific Virtues Project and _An Instinct for Truth_.

See:

- `sources/pennock-scientific-virtues.md`
- `policies/principles.yaml`
- `checks/pre_action.yaml`
- `checks/post_action.yaml`

## License

ClaimWright is licensed under the Apache License, Version 2.0. See `LICENSE`.

Copyright 2026 ClaimWright contributors.

## Current Status

ClaimWright is an initial framework, not a complete enforcement engine.

Implemented now:

- MOU;
- policy files;
- role cards;
- pre-action and post-action checks;
- draft schemas;
- source grounding notes;
- minimal Rust substrate checker;
- first public-safe artifact workflow.

Planned next:

- schema validation for claim and citation records;
- branch-comparison templates;
- stale-claim scanner interfaces;
- citation-library integration;
- configurable enforcement engine;
- cost and assistant-availability estimation;
- knowledge graph overlay for claims, citations, principles, and review states.

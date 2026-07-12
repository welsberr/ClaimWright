# ClaimWright

A human-agent research collaboration framework for grounded claims, adversarial review, and public-safe publication.

ClaimWright defines a memorandum of understanding for human-agent research work and a minimal executable policy substrate for applying that memorandum before and after non-trivial actions.

## Purpose

ClaimWright exists to reduce review burden without lowering standards. It treats claims, citations, corrections, negative results, and publication decisions as durable objects with provenance, review state, and explicit risk handling.

The first test case is the full path from a private claim to a public-safe artifact.

## Repository Shape

| Path | Purpose |
|---|---|
| `MOU.md` | Human-readable collaboration memorandum |
| `policies/` | Machine-readable principles, enforcement defaults, and claim states |
| `roles/` | Agent role cards and authority boundaries |
| `checks/` | Pre-action and post-action review checks |
| `schemas/` | Draft schemas for structured records |
| `sources/` | Source notes grounding policy concepts |
| `tools/claimwright/` | Minimal Rust CLI for policy substrate checks |
| `examples/` | Worked workflow examples |
| `roadmap/` | Implemented versus future capability |

## Enforcement Default

ClaimWright defaults to mixed enforcement:

- advisory for private exploratory work;
- soft gates for costly, long-running, or capacity-threatening work;
- hard gates for public release, private-data exposure, fabricated or unverified citations, destructive irreversible actions, and stale or contradicted claims.

## Source Status

This initial draft is derived from an interview on July 12, 2026. Pennock-style scientific virtues are grounded in Pennock's Scientific Virtues Project page and related MIT Press book metadata; see `sources/pennock-scientific-virtues.md`.

## Quick Check

```sh
cargo run --manifest-path tools/claimwright/Cargo.toml -- check .
```

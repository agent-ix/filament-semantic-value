---
id: SR-2734121003
title: "Code review — filament-semantic-value pin alignment"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-semantic-value; Cargo.toml, Cargo.lock, tests/clock_reference.rs, spec/functional/FR-135-clock-place-reference-in-declaration-admission-membership-and-closure.md"
review_set: subset
---

## Summary

Reviewed the exact `filament-exact` dependency pin and lockfile update for QSL-3312, along with the affected semantic-value requirement and test. The exact pin is consistent with the merged kernel commit, but it changes equality behavior while the local FR-135-AC-3 and its test still require cross-universe refusal; the recorded focused test fails.

## Verdict

**FAIL** — the pin cannot be consumed while it leaves the crate's owning requirement and focused test incompatible with the dependency's current behavior and the governing FR-829/FR-415 requirements.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Advancing the kernel pin changes equal-text cross-universe equality to `true`, while FR-135-AC-3 and `equality_follows_the_kernel_decision_for_clock_references` still require `ForeignReference`; the focused test fails and the local requirement contradicts current FR-415/FR-829. Align FR-135-AC-3 and its test with the governing equality behavior in this PR before merging the pin. | Cargo.toml:17, tests/clock_reference.rs:305, spec/functional/FR-135-clock-place-reference-in-declaration-admission-membership-and-closure.md:96 |

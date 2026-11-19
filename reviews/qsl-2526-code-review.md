---
id: SR-1794040450272172418986
title: "Code review — filament-semantic-value exact pin"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-semantic-value; Cargo.toml, Cargo.lock"
review_set: subset
---

## Summary

Reviewed QSL-2526 step 1, PR #114, at `b003ae551d3e6ff162f272067687b194863459fc` against `3fd65564216ad917db391caf03ead4cd90316d39`. The manifest and lockfile both pin the existing `filament-exact` dependency to merged main `b26a8e4c592503fccdc4a0e3d2cce23dd8487616`; targeted compile, tests, Clippy, deny, and duplicate-dependency checks passed.

## Verdict

**PASS** — the pin and lockfile are consistent, and the changed crate compiles and tests against the requested exact revision.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Review Notes

The PR changes only `Cargo.toml` and `Cargo.lock`; it does not change source, tests, dependency policy, or CI workflows. The `filament-exact` source in both files is the same immutable commit, `b26a8e4c592503fccdc4a0e3d2cce23dd8487616`, and `cargo tree --locked -d` reports no duplicates.

`cargo fmt --all -- --check` exits 1 on pre-existing formatting drift at `tests/admitted_equality.rs:286`, which is unchanged by this PR. `cargo deny check` passes with existing wildcard-dependency and unmatched-license-allowance warnings.

## Gates

- `cargo test --locked -p filament-semantic-value --test model_transition`: 12 passed.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: passed.
- `cargo check --locked -p filament-semantic-value --all-targets`: passed (coder report; independently repeated by Clippy's all-target compilation).
- `cargo tree --locked -d`: no duplicates.
- `cargo deny check`: passed; existing warnings only.
- `cargo fmt --all -- --check`: failed on unchanged `tests/admitted_equality.rs:286`.
- `git diff origin/main...HEAD --check`: passed.

---
id: SR-1794040450272172418997
title: "Code review — ordered relationship extent positions"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-semantic-value; src/relationship_extent.rs, tests/relationship_extents.rs; QSL-2958"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-semantic-value/FR-119
    type: reviews
---
## Summary

Reviewed PR #127 at frozen head 95760c6c4a878c68c3cb7760fe6c56f249ae906b, including the ordered extent implementation and tests. Position admission precedence and ordered reads appear correct; two required source-end test cases are not asserted, leaving FR-119-AC-21/23 evidence incomplete.

## Verdict

**CONDITIONAL** — two medium test coverage findings remain; no implementation defect was established by the scoped review.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No test asserts MissingPosition for an ordered multi-valued source end, leaving the source-side required-position branch without a failure oracle. | tests/relationship_extents.rs:611 |
| FND-002 | medium | The source-end read assertion is not repeated after reversing the instance list, so FR-119-AC-23's input-order independence is not directly checked. | tests/relationship_extents.rs:874 |

## Coverage

Inspected FR-119-AC-8 and AC-21/22/23, TC-919's ordered-position and read requirements, and downstream FSL FR-364-AC-15/16 as a boundary reference. The new refusal assertions compare exact typed variants and indices; mismatch and duplicate-triple precedence are checked before positions. Both ordered read paths sort by the opposite-end reference then numeric position and retain full-reference-key ordering for unordered ends. The changed Rust files were reviewed under the Rust API, implementation, safety, boundary, and testing checks. No CI workflow files changed. Filtered integration test passed (command in review report).

## Dispositions

Not applicable in the review pass.

Disposition pass 1 rechecked fix commit `1d526802865f41bb791e12369909ed041fdfb398`; both source-side coverage gaps are fixed.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Added an ordered source-end extent with no source position and exact `MissingPosition { relationship: "housing", extent_index: 0, instance_index: 0, side: Source }` assertion at `tests/relationship_extents.rs:796-813`. |
| FND-002 | fixed | Reversed the ordered source instance list, rebuilt the store, and asserted `sources("housing", t1)` equals the same expected `[p1, p2]` at `tests/relationship_extents.rs:925-947`. |

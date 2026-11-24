---
id: SR-1794040450272172418998
title: "Gap analysis — filament-semantic-value"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-semantic-value; spec/, src/, tests/; QSL-2958"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-semantic-value/spec
    type: references
---
## Summary

Ran the repository-wide computed matrix and source/test stub scans. The changed AC-21/22/23 each have a tagged integration test; eight other Test or Inspection criteria in the repository remain untagged, so the repository-wide gap verdict is FAIL. Plan completion: not assessed. Optional semantic review was performed for the changed FR-119 slice only.

## Verdict

**FAIL** — the computed matrix contains eight untagged criteria (none among the changed FR-119-AC-21/22/23). No TODO/FIXME/XXX or Rust todo/unimplemented placeholders were found in src/ or tests/.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-109-AC-6 has no tagged test evidence. | spec/functional/FR-109-bound-supplied-value-traversal.md |
| FND-002 | high | FR-121-AC-6 has no tagged inspection or analysis evidence. | spec/functional/FR-121-shared-property-and-fairness-values.md |
| FND-003 | high | FR-123-AC-6 has no tagged inspection or analysis evidence. | spec/functional/FR-123-step-simulation-record-and-obligation.md |
| FND-004 | high | FR-124-AC-6 has no tagged inspection or analysis evidence. | spec/functional/FR-124-identified-quantity-from-an-exact-conversion.md |
| FND-005 | high | FR-127-AC-14 has no tagged test evidence. | spec/functional/FR-127-map-and-bounded-quantity-in-declaration-admission-and-membership.md |
| FND-006 | high | FR-130-AC-1 has no tagged test evidence. | spec/functional/FR-130-share-input-path-admission-record.md |
| FND-007 | high | FR-130-AC-3 has no tagged test evidence. | spec/functional/FR-130-share-input-path-admission-record.md |
| FND-008 | high | FR-130-AC-4 has no tagged test evidence. | spec/functional/FR-130-share-input-path-admission-record.md |

## Coverage

`quoin matrix --repo . --json`: 196 criteria; 8 untagged; 0 tagged only by ignored tests. The changed FR-119-AC-21/22/23 each bind to `tests/relationship_extents.rs`. The matrix had no run evidence bound to criteria. Scanned Rust source and tests for TODO/FIXME/XXX/todo!/unimplemented!: none found. Reviewed the changed FR-119 tests and implementation for intent/test/code alignment. Plan completion: not assessed.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | Deferred to IR-1596. Adding evidence for FR-109-AC-6 is outside QSL-2958/PR #127, which changes relationship extent position behavior and does not modify FR-109 or its tests. |
| FND-002 | deferred | Deferred to IR-1596. Adding evidence for FR-121-AC-6 is outside QSL-2958/PR #127, which changes relationship extent position behavior and does not modify FR-121 or its inspection/analysis evidence. |
| FND-003 | deferred | Deferred to IR-1596. Adding evidence for FR-123-AC-6 is outside QSL-2958/PR #127, which changes relationship extent position behavior and does not modify FR-123 or its inspection/analysis evidence. |
| FND-004 | deferred | Deferred to IR-1596. Adding evidence for FR-124-AC-6 is outside QSL-2958/PR #127, which changes relationship extent position behavior and does not modify FR-124 or its inspection/analysis evidence. |
| FND-005 | deferred | Deferred to IR-1596. Adding evidence for FR-127-AC-14 is outside QSL-2958/PR #127, which changes relationship extent position behavior and does not modify FR-127 or its tests. |
| FND-006 | deferred | Deferred to IR-1596. Adding evidence for FR-130-AC-1 is outside QSL-2958/PR #127, which changes relationship extent position behavior and does not modify FR-130 or its tests. |
| FND-007 | deferred | Deferred to IR-1596. Adding evidence for FR-130-AC-3 is outside QSL-2958/PR #127, which changes relationship extent position behavior and does not modify FR-130 or its tests. |
| FND-008 | deferred | Deferred to IR-1596. Adding evidence for FR-130-AC-4 is outside QSL-2958/PR #127, which changes relationship extent position behavior and does not modify FR-130 or its tests. |

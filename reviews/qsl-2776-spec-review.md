---
id: SR-16709
title: "QSL-2776 filament-semantic-value PR 124 spec review: FR-135 and TC-932"
type: SpecReview
analysis: base
scope: "agent-ix/filament-semantic-value@9066d53421787890f5c5bde60197156a88ae0901 vs origin/main b1999c4; spec/functional/FR-135-clock-place-reference-in-declaration-admission-membership-and-closure.md, spec/test-cases/TC-932-clock-place-reference-in-admission-membership-and-closure.md"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-semantic-value/FR-135
    type: reviews
---

## Summary

Ticket: QSL-2776. Spec review of the new FR-135 and TC-932, with the EARS and
integrity sub-analyses folded in.

- Validation: `quire validate --scope . <FR-135> <TC-932> <FR-133>` with quire 0.52.0
  (engine 0.62.2) on terra passes with one warning (FND-001). On titan the resolved
  `~/.cargo/bin/quire` is 0.36.2 (engine 0.50.2), which reports "unknown type 'FR' (no
  archetype registered for it)" for every file; that is an outdated titan binary, not a
  spec defect.
- Numbering: no FR-135 or TC-932 existed on origin/main. Existing mentions are other
  repos' ids (QSL/FSL FR-135 in reviews/ and src/step_simulation.rs:17; quire-exact
  TC-932 in reviews/SR-13407). See FND-003 for the matrix side effect.
- FR-133 consistency: FR-133 forbids a clock type in this crate; FR-135 states a
  clock-place reference is a data value, not a clock, and leaves FR-133 unchanged.
  Consistent.
- FR-415 consistency: FR-135 defers equality, order, key and size to FR-415 and adds no
  rule. Behaviors 1-6 agree with FR-415 Behaviors 1, 5, 6, 7, 8 and 10.

## Reviewed units

- FR-135 Description (examined): "filament-semantic-value SHALL add no rule of its own for it: it SHALL admit ValueType::ClockReference wherever QExact FR-415 admits it and SHALL let it behave as QExact FR-415 states ... For object closure it adds no edge: a clock-place reference names a clock place, not an object, so it has no target object that could be present or dangling."
- FR-135 Behaviors 1-6 (examined).
- FR-135-AC-1 .. FR-135-AC-5 (examined).
- TC-932 Test Procedure and Expected Results 1-5 (examined).
- FR-133 Description (context_only): "A Clock arm in the kernel's ValueType would give the crate a type with no values to admit ..."
- filament-exact FR-415 Behavior 3 at 08a5a176 (context_only): "The rules that need the declared model are: ... that the owner exists in the admitted population ..."

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | EARS non-singular: the Description statement carries three SHALLs ("SHALL add no rule", "SHALL admit", "SHALL let it behave"); `quire validate` warns `[ears:non-singular]` at line 28. Split it, or move the deferral to Behavior and keep one SHALL. | spec/functional/FR-135-clock-place-reference-in-declaration-admission-membership-and-closure.md:28 |
| FND-002 | low | The no-edge rationale "has no target object that could be present or dangling" is not accurate: a place id names its owner object (component K in population P), and FR-415 Behavior 3 assigns "the owner exists in the admitted population" to the caller (FSL FR-230 / FR-106). As written, a reader can conclude owner existence is checked nowhere. State that the owner check belongs to the caller, so the closure adds no edge. The same wording is in the code comment. | spec/functional/FR-135-clock-place-reference-in-declaration-admission-membership-and-closure.md:33-35, src/object_closure.rs:263-264 |
| FND-003 | low | Allocating local FR-135 makes the existing cross-repo citation "FSL FR-135 `steps`" in `StepRowRef`'s doc resolve to this repo's FR-135 in `quire matrix`, which now warns `[trace-id-outside-marker] FR-135 ... at src/step_simulation.rs:17` (no FR-135 warning on main). Either qualify the citation as a full `ix://agent-ix/filament-spec-language/FR-135` reference or accept the warning. | src/step_simulation.rs:17 |

## Verdict

FR-135 and TC-932 are sound, consistent with FR-133 and FR-415, and validate under the
current quire. Three low wording/hygiene findings; none changes behaviour. Mergeable.

---
id: SR-16708
title: "QSL-2776 filament-semantic-value PR 124 gap analysis: FR-135 against TC-932 tests"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-semantic-value@9066d53421787890f5c5bde60197156a88ae0901 vs origin/main b1999c4; spec/functional/FR-135-*.md, spec/test-cases/TC-932-*.md, tests/clock_reference.rs, src/declaration.rs, src/declaration/membership.rs, src/object_closure.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-semantic-value/FR-135
    type: reviews
---

## Summary

Ticket: QSL-2776. Planless gap analysis of FR-135 against the computed matrix and the
tests. Plan completion: not assessed.

`quire matrix --scope .` (quire 0.52.0, terra) lists FR-135-AC-1 (tests/clock_reference.rs:129, 143),
AC-2 (:194), AC-3 (:265) and AC-4 (:324) as tagged. AC-5 is Inspection with no symbol
(method-without-symbol), which is expected for an Inspection criterion; inspection
confirms all 14 variant-listing matches name `ClockReference` explicitly.

Mutation by reading:

- AC-1: making any of `walk_type_refusal`, `contains_ieee`, `runtime_type` refuse or
  change ClockReference fails `clock_reference_is_an_admitted_leaf_in_every_declaration_position`.
- AC-2: moving ClockReference out of the `Kind` arm of `leaf_cause` fails the exact
  `SuppliedAdmissionError::Invalid { cause: Kind, locus: at }` asserts.
- AC-3: exact `Completed(false)`, `Completed(true)`, `Refused(ForeignReference)` and
  `TypeMismatch` asserts.
- AC-4: changing the `same_slots` arm to `true` fails `assert_ne!` (other place); to
  `false` fails `assert_eq!` (equal reference). Adding a dangling check fails the
  `expect` with no other object supplied.

No stubs, no `#[ignore]`, no tautological asserts. Tests are exact.

## Reviewed units

- FR-135-AC-1 (examined): "An environment declaring a record with fields r: ClockReference, o: Option<ClockReference>, s: Sequence<ClockReference> and m: Map<String, ClockReference>, an object type with attribute r: ClockReference, a tuple (ClockReference) and a union member with one ClockReference position admits; runtime_type(...) ...; contains_ieee is false for each."
- FR-135-AC-2 (examined): "admits(ClockReference, v) is true for a clock-place reference and false for an object Value::Reference, a String holding the same characters and an Integer; ... Supplied membership ... reports Kind at the value for each refused pair ..."
- FR-135-AC-3 (examined): "check_equality of two ClockReference operands succeeds and its evaluation returns false ..., true ... and ForeignReference for one text in two universes; ... against Reference(t) is refused TypeMismatch."
- FR-135-AC-4 (examined): "An ObjectClosure of one object whose attribute is a clock-place reference, ... equals a second closure built from an equal reference and differs from one built from a reference of another place."
- FR-135-AC-5 (examined): "Inspection finds an explicit ClockReference arm, with no wildcard, in every exhaustive variant-listing match ..."
- FR-135 Behavior 4 (examined): "The closure's own identity comparison SHALL compare two clock-place references by QExact FR-415 Behavior 7 (universe and text) and a clock-place reference with any other value kind as unequal."

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-135 Behavior 4 states the closure identity compares clock-place references by universe and text, but AC-4 and its test vary only the text in one universe. A mutation of `same_slots` to `a.text() == b.text()` (dropping the universe) passes every test. Add a closure pair built from the same text in two universes and assert unequal. | tests/clock_reference.rs:324-370, src/object_closure.rs:414 |
| FND-002 | low | FR-415 admits ClockReference as a Map key and as a Set element (FR-415 Behavior 1, AC-1, AC-5), and the arch rule says semantic-value admits it wherever FR-415 does, but FR-135 AC-1 and the tests cover only Map value and Sequence. The non-Sequence branch of `walk_type_refusal` (contains_map_walk / contains_ieee_walk over the element) is not exercised for `Set<ClockReference>`, and no declaration with a `Map<ClockReference, _>` field is admitted. Extend AC-1 and its test with a `Set<ClockReference>` field and a `Map<ClockReference, Integer>` field. | spec/functional/FR-135-clock-place-reference-in-declaration-admission-membership-and-closure.md:85, src/declaration.rs:3637-3643 |

## Verdict

Every AC has a tagged, exact test that fails if its arm changes. Two low coverage gaps
against stated behaviour (closure universe half; Set element and Map key positions).
Neither is a wrong result in the code. Mergeable; the two gaps are cheap to close in
this PR.

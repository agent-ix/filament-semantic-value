---
id: TC-932
title: "Clock-place reference in declaration admission, membership and object closure"
type: TC
relationships:
  - target: ix://agent-ix/filament-semantic-value/FR-135
    type: verifies
---
# TC-932: Clock-place reference in declaration admission, membership and object closure

## Description

Verify [FR-135](../functional/FR-135-clock-place-reference-in-declaration-admission-membership-and-closure.md):
`ValueType::ClockReference` is a leaf in declaration admission and membership,
the kernel decides its equality, and it adds no object-closure edge.

Scope: FR-135-AC-1 to FR-135-AC-5. The tests live in
`tests/clock_reference.rs` and need the `json-object` feature, which
`ClockPlaceReference::new` requires.

## Test Procedure

1. Admit the environment of FR-135-AC-1; call `runtime_type` and
   `contains_ieee` on the clock-reference types (FR-135-AC-1).
2. Call `admits` and supplied membership for a clock-place reference, an object
   reference, a String and an Integer against `ClockReference` and for the
   clock-place reference against `Reference(t)`; run a `Sequence` and an
   `Option` of clock-place references (FR-135-AC-2).
3. Check and evaluate equality of two `ClockReference` operands over texts
   differing in `K`, one text in one universe and one text in two universes;
   check `ClockReference` against `Reference(t)` (FR-135-AC-3).
4. Build object closures with a bare, `Option` and `Map` clock-place reference
   attribute, supplying no other object; compare closures (FR-135-AC-4).
5. Inspect the three modules for explicit arms (FR-135-AC-5).

## Expected Results

1. Admission succeeds; `runtime_type` returns the same type; `contains_ieee` is
   false.
2. Admitted for the clock-place reference only; `Kind` for each refused pair.
3. `false`, `true`, `ForeignReference`; `TypeMismatch` against `Reference(t)`.
4. Each closure admits, holds one object and reads the reference back; equal
   references give equal closures and another place gives an unequal one.
5. No wildcard arm in an exhaustive variant-listing match over `ValueType` or `Value` in those modules.

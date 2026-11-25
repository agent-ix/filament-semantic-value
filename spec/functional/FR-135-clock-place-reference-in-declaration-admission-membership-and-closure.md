---
id: FR-135
title: "Clock-place reference in declaration admission, membership and object closure"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/filament-exact/FR-415
    type: references
  - target: ix://agent-ix/filament-exact/FR-386
    type: references
  - target: ix://agent-ix/filament-exact/FR-096
    type: references
  - target: ix://agent-ix/filament-specification/FR-829
    type: references
  - target: ix://agent-ix/filament-semantic-value/FR-106
    type: references
  - target: ix://agent-ix/filament-semantic-value/FR-113
    type: references
  - target: ix://agent-ix/filament-semantic-value/FR-133
    type: references
---
# FR-135: Clock-place reference in declaration admission, membership and object closure

## Description

`filament-exact` carries a `Reference<Clock>` as the distinct kernel type
`ValueType::ClockReference` and value `Value::ClockReference` (QExact FR-415).
`filament-semantic-value` adds no rule of its own for it. The crate SHALL admit
`ValueType::ClockReference` wherever QExact FR-415 admits it. The kernel
decides equality, order, key use and the foreign-reference refusal as QExact
FR-415 states. For declaration admission and supplied membership it is a leaf,
like `ValueType::Reference`, with no object-type target to resolve. For object
closure it adds no edge: a clock-place reference names a clock place, not an
object, and the place id names its owner object (component K in population P),
whose existence QExact FR-415 Behavior 3 leaves to the caller (FSL FR-230 and
FR-106), so the closure has nothing to check for it.

A clock-place reference is a data value that names a clock place. It is not a
clock, so [FR-133](./FR-133-hold-the-data-part-of-clock-bearing-composites.md)
is unchanged: the crate still declares no clock type, and a clock member of a
composite is still projected away before declaration.

## Inputs

- Declarations whose member, element, payload or map-value type is
  `ValueType::ClockReference`, bare or inside `Option`, `Collection` or `Map`.
- `Value::ClockReference` values built by `ClockPlaceReference::new`.

## Outputs

- Admitted declarations, membership verdicts, equality results and object
  closures, with the kernel's own refusals for every other case.

## Behavior

1. **Declaration admission.** `ValueType::ClockReference` SHALL be an admitted
   member type of a record field, tuple position, union payload position and
   object attribute, and an admitted type link under `Option`, `Collection` and
   `Map`, and a `Map` key type and a `Set` element type. It names no
   declaration, so admission SHALL NOT refuse it
   `UnknownDeclaration` or an unknown object type, SHALL add no recursion edge,
   and `runtime_type` SHALL return it unchanged. It contains no `Float` and no
   `Map` link of its own.
2. **Membership.** `Value::ClockReference` SHALL be a member of
   `ValueType::ClockReference` exactly when the kernel `admits` it, and a value
   of any other kind at that type, or a clock-place reference at any other
   type, the crate SHALL refuse `InvalidMembershipCause::Kind` at the value.
   Membership SHALL add no conformance rule: an object `Value::Reference` is
   never a member of `ValueType::ClockReference`.
3. **Equality.** `check_equality` SHALL accept two `ValueType::ClockReference`
   operands as the same type and refuse a `ValueType::ClockReference` operand
   against a `ValueType::Reference` operand with `TypeMismatch`. Evaluation SHALL
   follow QExact FR-415 Behavior 5: references with equal place-id text SHALL
   compare equal (`true`) regardless of universe, and references with unequal
   place-id text SHALL compare unequal (`false`) in any universes. Universe
   remains readable payload and does not participate in identity. The crate
   SHALL NOT add an ordering
   operator; QExact FR-415 Behavior 6 orders keys only.
4. **Object closure.** A clock-place reference held in an object attribute,
   directly or inside an `Option`, `Collection` or `Map` value, SHALL add no
   closure edge and SHALL NOT make the object refuse `DanglingReference`.
   The closure's own identity comparison SHALL compare two clock-place
   references by QExact FR-415 Behavior 7 (place-id text only; universe
   remains readable payload) and a
   clock-place reference with any other value kind as unequal.
5. **Size.** The crate SHALL add no size accounting for it; the caller charges
   the byte length of its `text()` (QExact FR-415 Behavior 8).
6. **Explicit arms.** Every exhaustive match that lists the variants of
   `ValueType` or `Value` in declaration admission, membership and object
   closure SHALL name `ClockReference` explicitly and not through a wildcard
   arm.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-135-AC-1 | An environment declaring a record with fields `r: ClockReference`, `o: Option<ClockReference>`, `s: Sequence<ClockReference>`, `m: Map<String, ClockReference>`, `e: Set<ClockReference>` and `k: Map<ClockReference, Integer>`, an object type with attribute `r: ClockReference`, a tuple `(ClockReference)` and a union member with one `ClockReference` position admits; `runtime_type(ClockReference)` returns `ClockReference` and `runtime_type(Option<ClockReference>)` returns `Option<ClockReference>`; `contains_ieee` is false for each. | Test |
| FR-135-AC-2 | `admits(ClockReference, v)` is true for a clock-place reference and false for an object `Value::Reference`, a `String` holding the same characters and an `Integer`; `admits(Reference(t), clock_reference)` is false. Supplied membership of the same values reports `Kind` at the value for each refused pair and succeeds for the admitted ones, including a `Sequence<ClockReference>` of two references and an `Option<ClockReference>`. | Test |
| FR-135-AC-3 | `check_equality` of two `ClockReference` operands succeeds and its evaluation returns `false` for two place-id texts that differ in `K` in one universe and for unequal texts across universes, and `true` for equal place-id text in one or two universes; `check_equality` of `ClockReference` against `Reference(t)` is refused `TypeMismatch`. | Test |
| FR-135-AC-4 | An `ObjectClosure` of one object whose attribute is a clock-place reference, one whose attribute is `Option<ClockReference>` present, and one whose attribute is a `Map<String, ClockReference>` admits with no other object supplied, holds exactly one object, reads the reference back unchanged, and equals a second closure built from equal place-id text, including in another universe, and differs from one built from a reference with another place-id text. | Test |
| FR-135-AC-5 | Inspection finds an explicit `ClockReference` arm, with no wildcard, in every exhaustive variant-listing match over `ValueType` or `Value` in `declaration`, `declaration/membership` and `object_closure`, and finds no clock-specific equality, ordering or size rule of this crate. | Inspection |

## Dependencies

- QExact FR-415 defines the type, value, equality, key order, hash and size
  accounting this requirement defers to.
- [FR-106](./FR-106-object-closure-admission.md) owns object closure admission,
  unchanged except that a clock-place reference adds no edge.
- [FR-133](./FR-133-hold-the-data-part-of-clock-bearing-composites.md) owns the
  data-part rule: a clock-place reference is a data value, not a clock.

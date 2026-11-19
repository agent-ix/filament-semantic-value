---
id: FR-133
title: "Hold the data part of clock-bearing composites"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: references
  - target: ix://agent-ix/filament-specification/FR-415
    type: references
  - target: ix://agent-ix/filament-semantic-value/FR-113
    type: references
  - target: ix://agent-ix/filament-semantic-value/FR-115
    type: references
---
# FR-133: Hold the data part of clock-bearing composites

## Description

`filament-semantic-value` SHALL hold a record, tuple or union that has a clock
member as the **data part** that FSpec FR-322 "Data part of a clock-bearing
record, tuple or union" defines, and SHALL have no value type, value or slot for
a clock. A clock (FSpec FR-415) is timed-model state: it holds no data value,
has no type node in the checked package, and is read only by a clock constraint
and written only by a reset, which the timed model records by place. A
semantic value is a data value, so a clock member is not part of it. A `Clock`
arm in the kernel's `ValueType` would give the crate a type with no values to
admit, compare, hash or order, and every exhaustive walk over `ValueType` a
refusing arm, while adding nothing a data read can observe; the crate
therefore receives clock-bearing declarations already projected to their data
parts by the caller that lowers the checked package.

## Inputs

- Composite declarations whose shapes are data parts: a record's data fields in
  declaration order, a tuple's data positions in ascending source ordinal, and a
  union's members with a clock-payload member at payload arity 0.
- Values constructed against those declarations.

## Outputs

- Admitted record, tuple and union values of the data part, and the existing
  typed construction refusals for a value of any other shape.

## Behavior

1. **No clock type.** The crate SHALL declare no clock type; a declaration
   holds no clock. A clock-bearing composite reaches it as a
   `CompositeDeclaration` whose shape is the data part: `CompositeShape::Record`
   with the data fields only, `CompositeShape::Tuple` with the data positions
   only, and `CompositeShape::Union` with every case as a member.
2. **Tuple positions.** The tuple data part holds the data positions in
   ascending source ordinal; the data position at source ordinal `p` is
   position `d(p)`, the number of data positions whose ordinal is below `p`. The
   caller maps the ordinal of a `filament.op.tuple.project` to `d(p)` before it
   reads the crate's tuple; the crate reads and refuses by data index only. A
   tuple whose every position is a clock is the empty tuple `Tuple(vec![])`.
3. **Union members.** A case whose payload is a clock is a member with no
   payload position (`UnionMemberDeclaration` with empty `positions`), keeping
   its identifier, its `UnionMember` binding and its place in member order; it
   stays distinct from every other member, a nullary member included, and its
   value is the empty payload.
4. **Identity.** A data part takes the `NodeKey` of the declaration node of its
   type (FSpec FR-322 `model`/`record_value_type`, `model`/`tuple_value_type`
   or `model`/`variant_type`). The crate mints no key for a data part
   ([FR-060](./FR-060-the-crate-mints-no-identity.md)), and two declarations of
   equal data parts and distinct keys remain two types.
5. **Refusals.** The crate SHALL refuse `WrongArity` a tuple value whose
   position count is not the data part's arity (one carrying a slot for a clock
   position) and a value of a clock-payload member carrying a payload position,
   as it refuses any other arity mismatch. A
   value at a data position or field of the wrong type keeps its
   `TypeMismatch` refusal at that position.
6. **Equality.** The crate SHALL compare data parts with its unchanged
   structural equality. The checker refuses `=` and `!=` over a clock-bearing
   tuple or union (FSpec FR-322), so no checked package asks for that
   comparison, and the crate adds no clock-specific refusal.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-133-AC-1 | With the data parts of FSpec FR-322-AC-475 declared (tuple `Slot` as `Tuple([Integer])`, `Trio` as `Tuple([Integer, Boolean])`, `Ticks` as `Tuple([])`, variant `Phase` as a sealed union with members `Armed` and `Idle` each with no positions, record `Timer` with the single field `n: Integer`), environment admission succeeds; `tuple` builds a `Slot` from one `Integer`, a `Trio` from `4` and `true`, a `Ticks` from no value, `union` builds `Phase::Armed` and `Phase::Idle` with empty payloads and `record` builds a `Timer` from `n`; each value is admitted by `admits` at its declared composite type, reads back the supplied positions, payload, member and field, and equals a second construction from the same inputs. `Phase::Armed` and `Phase::Idle` are unequal and name distinct members. | Test |
| FR-133-AC-2 | `tuple` of `Trio` with three values (`4`, any value for the clock ordinal, `true`), `tuple` of `Slot` with two values, and `union` of `Phase::Armed` with one payload value are each refused `WrongArity`; `tuple` of `Trio` with `true` then `4` is refused `TypeMismatch` at position `0`; a `Timer` built with a field named `x` is refused `UndeclaredField`. | Test |
| FR-133-AC-3 | Inspection finds no `Clock` arm in the kernel `ValueType` reached by this crate, no clock type, marker, placeholder value or absent slot for a clock in the crate's declarations, values or admission, and no clock-specific refusal in its equality. | Inspection |

## Dependencies

- FSpec FR-322 "Data part of a clock-bearing record, tuple or union" defines
  the data part, the ordinal-to-data-index rule and the clock-payload member.
- The caller that lowers a checked package to `CompositeDeclaration`s projects
  every clock-bearing declaration to its data part and maps tuple ordinals to
  data indices; FSpec FR-415 and the timed model own the clocks themselves.
- [FR-115](./FR-115-stage-and-seal-union-declarations.md) owns union staging and
  sealing, unchanged for a clock-payload member.

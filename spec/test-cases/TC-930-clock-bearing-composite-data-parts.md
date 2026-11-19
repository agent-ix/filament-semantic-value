---
id: TC-930
title: "Clock-bearing composites are held as their data parts"
type: TC
relationships:
  - target: ix://agent-ix/filament-semantic-value/FR-133
    type: verifies
---
# TC-930: Clock-bearing composites are held as their data parts

## Description

Verify [FR-133](../functional/FR-133-hold-the-data-part-of-clock-bearing-composites.md):
the data parts of clock-bearing tuples, unions and records admit, construct and
read back like any composite, and a value shaped with a clock slot or a clock
payload is refused by the existing arity check.

Scope: FR-133-AC-1 to FR-133-AC-3.

## Test Procedure

1. Admit one environment declaring the data parts of FSpec FR-322-AC-475:
   `Slot` = `Tuple([Integer])`, `Trio` = `Tuple([Integer, Boolean])`,
   `Ticks` = `Tuple([])`, `Phase` a sealed union with members `Armed` and
   `Idle` each with no positions, and record `Timer` with the single field
   `n: Integer`. Build `Slot(7)`, `Trio(4, true)`, `Ticks()`, `Phase::Armed`,
   `Phase::Idle` and `Timer { n: 3 }`; check each with `admits` at its
   composite type, read back its positions, member or field, build it again
   from the same inputs and compare; compare `Phase::Armed` with `Phase::Idle`
   (FR-133-AC-1).
2. Build `Trio` from three values, `Slot` from two, `Phase::Armed` with one
   payload value, `Trio(true, 4)` and `Timer { x: ..., n: 3 }` (FR-133-AC-2).
3. Inspect the crate and the reached kernel `ValueType` for a clock arm, a
   clock marker or placeholder, and a clock-specific equality refusal
   (FR-133-AC-3).

## Expected Results

1. Admission succeeds; every value is admitted, reads back what was supplied
   and equals its second construction; `Phase::Armed` and `Phase::Idle` are
   unequal with distinct members.
2. `WrongArity` for the three arity cases, `TypeMismatch` at position `0` for
   `Trio(true, 4)`, `UndeclaredField` for the `Timer` with `x`.
3. None found.

---
id: FR-111
title: "Admit native UUID and Timestamp as distinct semantic-value leaves"
type: FR
relationships:
  - target: ix://agent-ix/filament-exact/FR-370
    type: references
---
# FR-111: Admit native UUID and Timestamp as distinct semantic-value leaves

## Description

`filament-semantic-value` SHALL accept the kernel's native UUID and Timestamp
types as terminal declaration types and their matching values as terminal
object attributes. It SHALL retain their distinct value kinds through
admission and object-closure lookup. The kernel owns native payload parsing
and value-kind identity; FSV owns its declaration and closure walks.

## Use case

A caller declares one required UUID attribute and one required Timestamp
attribute on a model object type, then admits an object carrying matching
native values. The caller can read those values from the closed object
without supplying an object that either value references.

## Behavior

1. FSV SHALL accept `ValueType::Uuid` and `ValueType::Timestamp` as terminal
   declared types, including object attributes. Neither contains an IEEE
   value or a composite-recursion edge. FSV's type admission SHALL admit a
   native value only under its matching native type.
   FSV's type admission SHALL refuse a crossed native value or an Integer value.
   FSV's equality-conversion table SHALL NOT convert a bounded Integer source
   to either native target.
2. FSV's `ObjectClosure` SHALL admit required native attributes whose values
   match their declared types and retain their native kinds and payloads on
   attribute lookup. FSV's `ObjectClosure` SHALL treat both native values as
   terminal during reference closure and SHALL NOT resolve either as an object
   reference. FSV's `ObjectClosure` SHALL refuse a crossed native value or
   Integer in either native attribute as that field's `TypeMismatch` before
   returning a closure.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-111-AC-1 | In an empty type environment, `check_type` accepts each native type and `contains_ieee` is false. `admits` accepts a UUID value only for `ValueType::Uuid` and a Timestamp value only for `ValueType::Timestamp`; crossed native values and Integer values refuse. A bounded Integer source does not admit equality conversion to either native target. | Test (TC-910) |
| FR-111-AC-2 | Given an object type with required UUID and Timestamp attributes, an object with matching native values closes without another referenced object, and attribute lookup returns the same native payloads. Replacing either attribute with the other native kind or an Integer refuses with `Attribute(TypeMismatch)` naming that field. | Test (TC-910) |

## Status

Implemented.

## Dependencies

- [Filament Exact FR-370](ix://agent-ix/filament-exact/FR-370) owns the distinct
  native kernel types, values, constructors and payload contract. FSV uses
  those values and does not parse a caller's tagged wire form.

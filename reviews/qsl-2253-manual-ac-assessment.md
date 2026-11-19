---
id: SR-1794040450272172418988
title: "Manual AC-to-tests assessment — QSL-2253 PR116"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-semantic-value; tests/clock_data_shapes.rs; proposed PR115 FR-133 and TC-930; src/declaration.rs; tests/common/mod.rs; pinned filament-exact value types"
review_set: subset
---

## Summary

Ticket: QSL-2253. Independent review of PR116 frozen head 3b01b714e45be68d675dda33f8484fe61e3cddba against base 1df71c27e6db56c8118cf017689aa7fab6156cde. Manual assessment replaces production gap-analysis because this diff adds only a Rust integration test. No computed matrix or repository-wide coverage conclusion is claimed.

## Verdict

**FAIL** — one statically established high finding prevents execution of the Phase acceptance assertions. Focused runtime validation is pending.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The Phase fixture declares Idle before Armed, but the union test expects Armed at position 0; declaration-order lookup returns 1, so the test fails before its union construction, reconstruction, equality, admission and WrongArity assertions. | tests/clock_data_shapes.rs:47; tests/clock_data_shapes.rs:108 |

## Evidence and required correction

CompositeDeclaration::new stores the supplied shape unchanged (src/declaration.rs:1152). Environment preparation records each union member's input position (2586–2635) and stores that declaration (2683–2689). union_member_named enumerates retained members (3158–3171); it does not sort them. Consequently Armed returns position 1, while expected_position is 0 on the first iteration. This violates the proposed FR-133 behavior 3 caller obligation and blocks both union cases. Supply members as Armed, Idle, keeping the expected order assertion; do not add sorting to production or weaken the assertion. Re-run only the already authorized name-filtered tests after correction. This is the same root defect as the code-review artifact, recorded here for this method's independent evidence; it is one fix, not an extra fix round.

## Acceptance assessment

| Unit | Role | Assessment |
| --- | --- | --- |
| FR-133-AC-1 Slot | examined | Lines 164–201 construct Slot(7), assert one present Integer slot and declaration key, extract/reconstruct via tuple(), call admits(), and evaluate typed equality. |
| FR-133-AC-1 Trio | examined | Same loop constructs (4,true), checks both ordered kinds/values and exact slot count, reconstructs and admits, then evaluates equality. No Clock ordinal slot is passed. |
| FR-133-AC-1 Ticks | examined | Empty positions force exact zero slots, empty reconstruction, admits(), and equality; the empty zip alone is not the evidence. |
| FR-133-AC-1 Phase | examined | Intended assertions at 109–142 cover arity 0, retained key/member/identifier, payload, occurrence count, reconstruction, admission, positive equality and cross-member inequality. Line 108 fails first: these assertions are unreachable on this candidate. |
| FR-133-AC-1 Timer | examined | Lines 237–260 require exactly one present Integer(3) slot, supplied key, retained field name n, record reconstruction, admits(), and equality. |
| FR-133-AC-2 | examined | Lines 203–226 assert both WrongArity counts/components and swapped Trio TypeMismatch at Position(0). Lines 261–271 assert Timer x as UndeclaredField at Field(x). Phase payload WrongArity at 143–153 is blocked by FND-001. |
| FR-133-AC-3 | examined | Inspection of the pinned exact ValueType/Value and semantic declaration/admission/equality paths found no Clock type/marker, clock slot or clock-specific equality refusal. Inspection evidence only; no automated test binding claimed. |
| TC-930 procedures/results 1–3 | examined | Mapped all forms and refusal cases above; runtime execution remains pending. |
| FR-133 behavior 1–6 | context_only | Caller supplies projected data shapes, maps source ordinals, supplies FSpec member order, retains keys, and semantic-value uses unchanged structural equality/refusals. This PR does not test model lowering or wire conversion. |

These are appropriate ordinary data-only semantic API shapes under FR-133, rather than an attempted Clock ValueType. Assertions inspect production outputs and typed refusal causes; no doubles or test-only implementation path is used. Source-ordinal projection, checked-package admission, timed clock places and checker equality refusal are caller/FSL responsibilities and are not established here. Equality is the semantic check_equality()/CheckedEquality::evaluate() path, not Rust derived equality alone. Admission is asserted on reconstructed values; exact reconstruction and equality relate those values to original construction.

## Examined excerpts

```yaml
{
  "scope": [
    {
      "id": "FR-133-AC-1",
      "path": "spec/functional/FR-133-hold-the-data-part-of-clock-bearing-composites.md",
      "role": "examined",
      "excerpt": "With the data parts of FSpec FR-322-AC-475 declared (tuple `Slot` as `Tuple([Integer])`, `Trio` as `Tuple([Integer, Boolean])`, `Ticks` as `Tuple([])`, variant `Phase` as a sealed union with members `Armed` and `Idle` each with no positions, record `Timer` with the single field `n: Integer`), environment admission succeeds; `tuple` builds a `Slot` from one `Integer`, a `Trio` from `4` and `true`, a `Ticks` from no value, `union` builds `Phase::Armed` and `Phase::Idle` with empty payloads and `record` builds a `Timer` from `n`; each value is admitted by `admits` at its declared compos"
    },
    {
      "id": "FR-133-AC-2",
      "path": "spec/functional/FR-133-hold-the-data-part-of-clock-bearing-composites.md",
      "role": "examined",
      "excerpt": "`tuple` of `Trio` with three values (`4`, any value for the clock ordinal, `true`), `tuple` of `Slot` with two values, and `union` of `Phase::Armed` with one payload value are each refused `WrongArity`; `tuple` of `Trio` with `true` then `4` is refused `TypeMismatch` at position `0`; a `Timer` built with a field named `x` is refused `UndeclaredField`."
    },
    {
      "id": "FR-133-AC-3",
      "path": "spec/functional/FR-133-hold-the-data-part-of-clock-bearing-composites.md",
      "role": "examined",
      "excerpt": "Inspection finds no `Clock` arm in the kernel `ValueType` reached by this crate, no clock type, marker, placeholder value or absent slot for a clock in the crate's declarations, values or admission, and no clock-specific refusal in its equality."
    },
    {
      "id": "TC-930",
      "path": "spec/test-cases/TC-930-clock-bearing-composite-data-parts.md",
      "role": "examined",
      "excerpt": "Scope: FR-133-AC-1 to FR-133-AC-3."
    }
  ],
  "bindings": [
    {
      "test_id": "clock_union_data_shape_round_trips_without_a_payload_placeholder",
      "ac_id": "FR-133-AC-1",
      "trace": "correct"
    },
    {
      "test_id": "clock_union_data_shape_round_trips_without_a_payload_placeholder",
      "ac_id": "FR-133-AC-2",
      "trace": "correct"
    },
    {
      "test_id": "clock_tuple_data_shape_round_trips_only_its_ordered_data_slots",
      "ac_id": "FR-133-AC-1",
      "trace": "correct"
    },
    {
      "test_id": "clock_tuple_data_shape_round_trips_only_its_ordered_data_slots",
      "ac_id": "FR-133-AC-2",
      "trace": "correct"
    },
    {
      "test_id": "clock_record_data_shape_round_trips_only_its_named_data_field",
      "ac_id": "FR-133-AC-1",
      "trace": "correct"
    },
    {
      "test_id": "clock_record_data_shape_round_trips_only_its_named_data_field",
      "ac_id": "FR-133-AC-2",
      "trace": "correct"
    }
  ]
}
```

## Validation limits

No cargo command was started by this reviewer. The existing queued Clippy log still showed lock contention; Clippy completion and coder name-filtered clock_ tests are pending. No full suite, workspace sweep, aggregate or gate was run. Direct rustfmt --edition 2021 --check tests/clock_data_shapes.rs and git diff --check against the frozen base both exited 0; rustfmt warned that imports_granularity and group_imports require nightly. Static control-flow analysis establishes FND-001; no runtime failure is claimed.

PR115's inspected diff (head db597fe3d70f5b37ed31917bc4e48d3145af8081) is used as the proposed owning contract. Its API query returned a state different from the brief's OPEN status. No merge or integration into the frozen PR116 base is assumed; lead must reconcile dependency status. Core-data PR360 is the completed upstream admission work per the brief.


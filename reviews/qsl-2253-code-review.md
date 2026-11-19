---
id: SR-1794040450272172418987
title: "Code review with Rust lane — QSL-2253 PR116"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-semantic-value; tests/clock_data_shapes.rs; proposed PR115 FR-133 and TC-930; src/declaration.rs; tests/common/mod.rs; pinned filament-exact value types"
review_set: subset
---

## Summary

Ticket: QSL-2253. Independent review of PR116 frozen head 3b01b714e45be68d675dda33f8484fe61e3cddba against base 1df71c27e6db56c8118cf017689aa7fab6156cde. Code-review ran with rust-review folded into this artifact; rust-style and its testing/implementation references, local CLAUDE.md and lint/formatter policies were loaded. No production, CI or policy files changed; no applicable AssuranceProfile was found under spec/.

## Verdict

**FAIL** — one statically established high finding prevents execution of the Phase acceptance assertions. Focused runtime validation is pending.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The Phase fixture declares Idle before Armed, but the union test expects Armed at position 0; declaration-order lookup returns 1, so the test fails before its union construction, reconstruction, equality, admission and WrongArity assertions. | tests/clock_data_shapes.rs:47; tests/clock_data_shapes.rs:108 |

## Evidence and required correction

CompositeDeclaration::new stores the supplied shape unchanged (src/declaration.rs:1152). Environment preparation records each union member's input position (2586–2635) and stores that declaration (2683–2689). union_member_named enumerates retained members (3158–3171); it does not sort them. Consequently Armed returns position 1, while expected_position is 0 on the first iteration. This violates the proposed FR-133 behavior 3 caller obligation and blocks both union cases. Supply members as Armed, Idle, keeping the expected order assertion; do not add sorting to production or weaken the assertion. Re-run only the already authorized name-filtered tests after correction.

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

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1: inspected pushed head 1193b513280c51a92936e947a12b96f0b70e5611. Phase is supplied as Armed, Idle at tests/clock_data_shapes.rs:47, matching the preserved expected-position assertion at :106–108. Production declaration-order lookup remains unchanged. |

## Disposition round 1 evidence

**Current verdict: PASS; the coder's focused clock_ tests passed, and merge is review-clear subject to custody of these normalized artifacts and the final custody diff-check.** All findings without prior outcomes were checked: FND-001 is fixed. No new defects were found; no additional finding IDs were allocated. The original Findings and review-pass prose above remain historical and unchanged apart from whitespace normalization.

Fixed source excerpt, tests/clock_data_shapes.rs:47:

```rust
                CompositeShape::Union(vec![member("Armed"), member("Idle")]),
```

The iteration over ["Armed", "Idle"] and assert_eq!(position, expected_position) at lines 106–108 are intact. The production lookup still enumerates the retained declaration vector (src/declaration.rs:3158–3171); no sorting was introduced. Static inspection now reaches both Phase cases' retained identifier/binding/key, empty payload, reconstruction, admits(), positive equality, cross-member inequality and WrongArity assertions without the original index mismatch. Runtime success is not inferred from this inspection. Slot, Trio, Ticks and Timer bodies and all typed refusal assertions are unchanged from the reviewed candidate.

PR115 is verified MERGED at e7f477ce6e6ec1f0fb6e8b6a8b05428c29a61d7d, mergedAt 2026-11-19T16:24:21Z. That commit is the current PR116 base and an ancestor in the local history, so FR-133 and TC-930 are now integrated owning specifications rather than proposed context. The earlier dependency uncertainty above is superseded.

The committed SR copies were compared byte-for-byte with the original scratch artifacts and matched. For this authoritative disposition copy, trailing spaces were removed and the extra EOF blank was removed; no original finding wording or other original content changed. The lead/coder must copy this updated file into reviews/ in the final evidence commit.

Validation observed during this disposition:
- PR116 API head and local HEAD both resolve to 1193b513280c51a92936e947a12b96f0b70e5611; the sole working tree is clean.
- Test diff against original review head changes only member("Idle"), member("Armed") to member("Armed"), member("Idle"). Relative to the merged spec base, only the new test and two original SRs are present.
- Direct rustfmt --edition 2021 --check tests/clock_data_shapes.rs exits 0, with stable-channel warnings for imports_granularity/group_imports.
- Existing original Clippy log contains successful completion. The affected /Users/peter/dev/_lead-bin/qsl-2253-clippy-round1.log now contains Checking filament-semantic-value and Finished dev profile in 0.32s. This is observed coder-owned log evidence, not a reviewer-executed cargo check.
- The later /Users/peter/dev/_lead-bin/qsl-2253-test-round1.log completed: clock_record_data_shape_round_trips_only_its_named_data_field, clock_union_data_shape_round_trips_without_a_payload_placeholder, and clock_tuple_data_shape_round_trips_only_its_ordered_data_slots all passed; result 3 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out. This is coder-owned focused test evidence, not a reviewer cargo execution. No targeted test remains pending. The old pre-mutation queued test was cancelled per the lead's brief; it is not recorded as a semantic test failure.
- Current committed git diff --check reports only the original SR trailing space and extra EOF blank lines; those defects are removed in this scratch copy. Copying these authoritative artifacts and the final custody diff-check remain lead/coder work.
- No cargo command, duplicate queue entry, full suite, workspace sweep, aggregate, gate, merge or product/test edit was performed by this reviewer.

Code-review continues to include the Rust lane and loaded rust-style conventions. Manual AC-to-tests assessment remains the appropriate replacement for production gap-analysis on this test-only change. No computed-matrix or wire/model-lowering coverage claim is added.

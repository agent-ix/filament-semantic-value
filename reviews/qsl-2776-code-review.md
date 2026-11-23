---
id: SR-16707
title: "QSL-2776 filament-semantic-value PR 124 code review: ValueType::ClockReference leaf arms"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-semantic-value@9066d53421787890f5c5bde60197156a88ae0901 vs origin/main b1999c4; src/declaration.rs, src/declaration/membership.rs, src/object_closure.rs, tests/clock_reference.rs, Cargo.toml, Cargo.lock"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-semantic-value/FR-135
    type: reviews
---

## Summary

Ticket: QSL-2776. Code review with the rust-review lane folded in, scoped to
`git diff origin/main...9066d53`. The PR adds an explicit `ValueType::ClockReference`
or `Value::ClockReference` arm to 14 exhaustive matches, pins filament-exact to
08a5a176 (on filament-exact main, #195) and adds `tests/clock_reference.rs`.

Each site was checked against filament-exact FR-415 at 08a5a176:

- declaration.rs:2662 (union member position walk), 3072 and 3122 (`runtime_type`
  descent and wrapper rebuild), 3431 (`contains_ieee`), 3511 (`contains_map_walk`),
  3589 (`contains_ieee_walk`), 3840 (recursion edges): leaf, no declaration key, no
  Float, no Map link, no edge. Correct.
- declaration.rs:3665 (`walk_type_refusal`): `Reference(key)` is refused only through
  its guarded arm for an unknown object type; the unguarded arm admits. ClockReference
  has no target, so admitting it unconditionally matches FR-415 Behavior 1. Population
  stays refused. Correct.
- declaration.rs:5717 (`top_level_unit`), 5748 (`bounded_read`), 5856
  (`integer_source_admits`): no unit, no bounded read, no integer conversion row.
  Correct (FR-415 Behavior 10: no conversion).
- membership.rs:185 (`leaf_cause`): a failed leaf admission at ClockReference is
  `Kind`, which is the only possible failure (the type has no payload domain). Correct.
- object_closure.rs:265 (`check_closed`): no edge, no dangling check. Matches the
  binding arch rule.
- object_closure.rs:414 and 435 (`same_slots`, the `PartialEq for ObjectClosure`
  structural identity): `a == b` is the derived (universe, text) equality of FR-415
  Behavior 7, and a cross-kind pair is false. This is Rust value identity, which must
  be total and cannot carry a refusal. Language equality goes through `check_equality`
  to the kernel plan, which returns `ForeignReference` (tested in AC-3). So `==` hides
  nothing; it matches the existing `Reference` arm.

The wildcard pair matches that remain (`same_type` 3028 `_ => left == right`, `narrows`
4432, `check_equality` 5570, operand admission 6375) are not variant-listing matches;
each gives ClockReference the right result (same type, narrows to itself, `Plan`
schedule, `admits_or_abort`). No other module in `src/` special-cases `Reference`.

Cargo.lock diff is one line (filament-exact source rev); no other dependency drifted.
The pin is on filament-exact origin/main. The json-object feature gate on the test file
matches the kernel's gate on `ClockPlaceReference::new`.

Focused runs on titan, target in the worktree, jobs 3:
`cargo test --locked --features json-object --test clock_reference` 5 passed;
`--lib object_closure` 16 passed; `--test map_and_bounded_quantity --test clock_data_shapes`
3 + 23 passed; `cargo clippy --locked --features json-object --lib --test clock_reference -- -D warnings` clean.

## Reviewed units

- src/declaration.rs:2662, 3072, 3122, 3431, 3511, 3589, 3665, 3840, 5717, 5748, 5856 (examined): `| ValueType::ClockReference` added beside `| ValueType::Reference(_)` in each leaf arm.
- src/declaration/membership.rs:185 (examined): `| ValueType::ClockReference` in the `leaf_cause` Kind arm.
- src/object_closure.rs:263-266 (examined): `Value::ClockReference(_) | Value::Reference(_) | ... => {}` in `check_closed`.
- src/object_closure.rs:414, 435 (examined): `(Value::ClockReference(a), Value::ClockReference(b)) => a == b` and `| (Value::ClockReference(_), _)` in `same_slots`.
- Cargo.toml:17, Cargo.lock:76 (examined): filament-exact rev a58474cc -> 08a5a176.
- tests/clock_reference.rs (examined): five tests, TC-932.
- filament-exact FR-415 at 08a5a176 (context_only): Behaviors 1, 5, 6, 7, 8.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. Every one of the 14 sites is correct as a `Reference(_)`-like leaf, none resolves
a target, orders, hashes or computes bounds for the new variant, and the closure identity
comparison is the right (structural) equality. No panic path, no wildcard added. The
pin moves only filament-exact. Mergeable from the code side.

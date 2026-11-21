---
id: SR-17952591800001
title: code-review of QSL-2173 PR120 oracle correction
type: SpecReview
analysis: code-review
scope: agent-ix/filament-semantic-value; tests/fallible_type_check.rs;
  FR-132-AC-1; existing guard and FR-149 as context
review_set: subset
---

## Summary

Ticket: QSL-2173. Independent review of PR120 at exact 8417f3cc2141980117c145a0a1be9af8ad74fb3f, separate from PR119. One changed test file, 1 insertion and 6 deletions. No applicable AssuranceProfile exists under spec/. Rust-style, rust-review, code-review and the testing reference were applied. No spec/plan edit warrants spec-review. Review-only artifacts are in scratch; source and branch are unchanged.

FR-149 admits float-bearing Set, Bag and OrderedSet members at every nested depth. The production guard refuses only a non-Sequence element that contains both Map and IEEE content. Fixtures 1, 2 and 3 contain an IEEE float through record, tuple-to-record and recursive union-to-tuple-to-record paths, with no Map; fixture 4 is an option-recursive union without a float. The retained 1..=4 loop crosses all four fixtures with all three collection kinds (12 direct public-API assertions), plus each bare composite. These assertions execute without conditional bypass or mocks and would fail under the previous broad IEEE refusal. The recursive cases still exercise the contains_map walk and its visited handling; this change does not claim to test equality evaluation or all IEEE discovery internals.

The same selected test retains bare-map acceptance and Set<Map<optional Float64>> refusal. The separate Population(None), Option<Population(None)>, unknown-composite and non-object-reference controls are untouched. The Population controls were inspected but not executed by the supplied one-test run. No assertion, fixture, trace tag, iteration, test attribute, negative control, implementation, spec, dependency or lockfile was deleted or changed beyond the stale expected-result conditional.

## Examined Scope

```yaml
clean: true
scope:
  - id: ieee_discovery_preserves_record_tuple_union_and_cycle_behavior
    path: tests/fallible_type_check.rs
    role: examined
    excerpt: >-
      assert_eq!(
                      environment.try_check_type(&collection(kind, composite.clone())),
                      Ok(())
                  );
  - id: record fixture
    path: tests/fallible_type_check.rs
    role: examined
    excerpt: |-
      CompositeShape::Record(vec![FieldDeclaration::new(
                          "value",
                          float.clone(),
                          Presence::Required,
                      )])
  - id: tuple fixture
    path: tests/fallible_type_check.rs
    role: examined
    excerpt: CompositeShape::Tuple(vec![ValueType::Composite(common::node(1))])
  - id: recursive union fixture
    path: tests/fallible_type_check.rs
    role: examined
    excerpt: vec![ValueType::option(ValueType::Composite(common::node(3)))]
  - id: non-float cycle fixture
    path: tests/fallible_type_check.rs
    role: examined
    excerpt: vec![ValueType::option(ValueType::Composite(common::node(4)))]
  - id: collection coverage
    path: tests/fallible_type_check.rs
    role: examined
    excerpt: |-
      for number in 1..=4 {
              let composite = ValueType::Composite(common::node(number));
  - id: collection kinds
    path: tests/fallible_type_check.rs
    role: examined
    excerpt: |-
      for kind in [
                  CollectionKind::Set,
                  CollectionKind::Bag,
                  CollectionKind::OrderedSet,
              ]
  - id: Map-plus-IEEE negative control
    path: tests/fallible_type_check.rs
    role: examined
    excerpt: |-
      assert_eq!(
              environment.try_check_type(&collection(CollectionKind::Set, map)),
              refused(IllTypedCause::OperatorIneligible)
          );
  - id: Population negative control
    path: tests/fallible_type_check.rs
    role: context_only
    excerpt: |-
      for ty in [
              ValueType::Population(None),
              ValueType::option(ValueType::Population(None)),
          ]
  - id: FR-132-AC-1
    path: spec/functional/FR-132-check-external-types-with-fallible-storage.md
    role: context_only
    excerpt: "Public `try_check_type` returns success for admitted leaves,
      references, composites, sealed union handles, nested options, sequences,
      maps and IEEE-bearing set/bag/ordered-set elements without a nested `Map`;
      unknown composites and non-object references return `Refused(IllTyped {
      cause: TypeMismatch })`. Populations and non-sequence collections whose
      element type contains both a `Map` and an IEEE float at any depth return
      `Refused(IllTyped { cause: OperatorIneligible })`."
  - id: FR-149 member float identity
    path: /Users/peter/dev/filament-specification/spec/functional/type-model/FR-149-apply-complete-equality-matrix.md
    role: context_only
    excerpt: "That relation is this matrix with exactly one change: a `Float32` or
      `Float64` leaf, at any depth inside the member, is compared by FR-148
      `bitIdentical`."
  - id: walk_type_refusal guard
    path: src/declaration.rs
    role: context_only
    excerpt: >-
      if collection.kind() != CollectionKind::Sequence
                              && self.contains_map_walk(collection.element(), charge, storage)?
                              && self.contains_ieee_walk(collection.element(), charge, storage)?
findings: []
bindings:
  - test_id: ieee_discovery_preserves_record_tuple_union_and_cycle_behavior
    ac_id: FR-132-AC-1
    trace: correct
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Rust review is folded into this code-review artifact. The edit corrects a stale oracle against settled semantics and the existing implementation; it does not alter product behavior or weaken a valid refusal.

## Validation

Supplied lead execution evidence was read directly: /private/tmp/qsl-2173-sv-main-one-oracle.log records the selected test failing on main 74b334 with left Ok(()) versus right OperatorIneligible, 0 passed / 1 failed / 3 filtered. /private/tmp/qsl-2173-sv-oracle-after.log records the same selected test passing, 1 passed / 0 failed / 3 filtered. These are supplied execution results, not reviewer reruns. git diff --check against origin/main exits 0. No additional Cargo, build, test or Python command was run. No full-suite pass is claimed.

## Verdict

CLEAR for PR120. No new finding in this test-only oracle correction. This verdict makes no repository-wide assurance claim.

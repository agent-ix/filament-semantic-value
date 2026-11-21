---
id: SR-17952584400002
title: spec-review/base review of QSL-2173 documentation
type: SpecReview
analysis: base
scope: agent-ix/filament-semantic-value; src/declaration.rs doc comment;
  spec/functional/FR-127-map-and-bounded-quantity-in-declaration-admission-and-membership.md;
  spec/functional/FR-132-check-external-types-with-fallible-storage.md
review_set: subset
---

## Summary

Ticket: QSL-2173. Independent review of filament-semantic-value#119 at dde27e0d38eb7c72bfdbbe3786a99aa489afb999. Scope is the three-file docs/spec diff against origin/main. Review set: base plus EARS and dependency; no structural/domain-object changes warrant integrity/object lenses. No applicable AssuranceProfile exists under spec/. No enforced review policy applies. Gap-analysis does not apply because no executable production code changes. Review-only scratch artifacts; no source edits or branch mutation.

The revised semantic claims reconcile FR-127 and FR-132 with the existing guard and authoritative FR-149. IDs and links validate. No new hash, digest, pin, tracking ceremony, copied matrix or alternate equality algorithm is introduced. The changed criteria remain concrete and falsifiable. The matrix marks FR-127-AC-2 and FR-132-AC-1 tagged; tags alone do not establish correct assertions.

## Examined Scope

```yaml
clean: true
scope:
  - id: try_check_type documentation
    path: src/declaration.rs
    role: examined
    excerpt: IEEE-bearing set, bag and ordered-set elements are admitted unless the
      element type also contains a `Map` at any depth.
  - id: FR-127 Behavior 2
    path: spec/functional/FR-127-map-and-bounded-quantity-in-declaration-admission-and-membership.md
    role: examined
    excerpt: The declaration walk SHALL retain `OperatorIneligible` for a
      non-`Sequence` collection whose element type contains both a `Map` and an
      IEEE float at any depth.
  - id: FR-127-AC-2
    path: spec/functional/FR-127-map-and-bounded-quantity-in-declaration-admission-and-membership.md
    role: examined
    excerpt: Declaring a record field of type `Map<Text, Float64>`,
      `Option<Map<Text, Float32>>` and `Sequence<Map<Text, Float64>>` is
      accepted in plain and metered declaration admission, while `Set<Map<Text,
      Float64>>` is refused `OperatorIneligible` because its element type
      contains both a `Map` and an IEEE float.
  - id: FR-132 Behavior 4
    path: spec/functional/FR-132-check-external-types-with-fallible-storage.md
    role: examined
    excerpt: FSV SHALL admit IEEE-bearing set, bag and ordered-set elements under
      QSpec FR-149, except that FSV SHALL refuse a non-sequence collection whose
      element type contains both a `Map` and an IEEE float at any depth with
      `OperatorIneligible`.
  - id: FR-132-AC-1
    path: spec/functional/FR-132-check-external-types-with-fallible-storage.md
    role: examined
    excerpt: "Public `try_check_type` returns success for admitted leaves,
      references, composites, sealed union handles, nested options, sequences,
      maps and IEEE-bearing set/bag/ordered-set elements without a nested `Map`;
      unknown composites and non-object references return `Refused(IllTyped {
      cause: TypeMismatch })`. Populations and non-sequence collections whose
      element type contains both a `Map` and an IEEE float at any depth return
      `Refused(IllTyped { cause: OperatorIneligible })`."
  - id: FR-132 FR-149 reference
    path: spec/functional/FR-132-check-external-types-with-fallible-storage.md
    role: examined
    excerpt: "- [QSpec FR-149](ix://agent-ix/filament-specification/FR-149) admits
      float-bearing collections and defines `bitIdentical` float-leaf comparison
      within set, bag and ordered-set members."
  - id: FR-149 membership relation
    path: /Users/peter/dev/filament-specification/spec/functional/type-model/FR-149-apply-complete-equality-matrix.md
    role: context_only
    excerpt: "That relation is this matrix with exactly one change: a `Float32` or
      `Float64` leaf, at any depth inside the member, is compared by FR-148
      `bitIdentical`."
  - id: walk_type_refusal guard
    path: src/declaration.rs
    role: context_only
    excerpt: if collection.kind() != CollectionKind::Sequence &&
      self.contains_map_walk(collection.element(), charge, storage)? &&
      self.contains_ieee_walk(collection.element(), charge, storage)?
findings: []
bindings: []
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Validation and Baseline

Baseline observations: tests/fallible_type_check.rs:241-253 still expects OperatorIneligible for float-bearing record/tuple/union elements with no Map. This untouched oracle contradicts the existing guard and settled FR-149; it is separately routed by the lead and is not a PR119 regression. FR-127-AC-14 is untagged in the computed matrix, also outside the changed criterion. Vale reports four unchanged US-spelling warnings at FR-127:215,217,219,236. No Cargo, build or test commands were run under the dispatch restriction. docs_lint.py was not run because Python is prohibited; structure and phrasing were inspected manually. No runtime pass is claimed.

Changed specs validated at explicit paths with quire 0.52.0 (engine 0.62.2), exit 0. git diff --check exits 0. The review artifacts are validated at their explicit scratch paths.

## Verdict

CLEAR for the changed documentation/specification contract. The revised semantic claims reconcile FR-127 and FR-132 with the existing guard and authoritative FR-149. IDs and links validate. No new hash, digest, pin, tracking ceremony, copied matrix or alternate equality algorithm is introduced. The changed criteria remain concrete and falsifiable. The matrix marks FR-127-AC-2 and FR-132-AC-1 tagged; tags alone do not establish correct assertions.

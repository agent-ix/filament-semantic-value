---
id: SR-1794040450272172418995
title: "QSL-2734 PR118 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-semantic-value; src/, tests/, spec/; QSL-2734 PR118 at 9f898af6a0178869b05fc7b74e8183c1b45d3be2"
review_set: subset
---

## Summary

Independent gap-analysis of frozen PR118. Ticket: QSL-2734. FR-134 and TC-931 are landed requirements. No applicable AssuranceProfile was found. The engine binds all seven FR-134 criteria, but two acceptance branches lack the specified behavioral evidence. Inherited repository matrix gaps are listed separately below.

## Verdict

FAIL for the repository audit (eight inherited untagged criteria); CONDITIONAL for the PR-specific gap slice until its two medium findings are resolved. No new reverse requirement gap or source/test stub was identified.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The tight-limit loop only sets each ceiling to zero and matches Outcome::Incomplete(_). It never verifies the limit kind, first denied charge, record, atomic counters, or the just-sufficient and one-less boundaries required by AC-6/TC-931 step 6. Injected-denial prefix tests do not establish actual resource-limit behavior. | tests/dimension_equivalence_conversion.rs:637-678 |
| FND-002 | medium | The tagged test covers a base/derived pair only. No test constructs two distinct derived dimensions with equal normalized maps, verifies direct declared conversion refuses before charging without membership, then admits explicit equivalence and verifies exact converted value/target identity. This independent AC-7/TC-931 step-7 branch is untested. | tests/dimension_equivalence_conversion.rs:599-633 |

## Evidence and correction

AC-6 injection tests already compare admission prefixes and atomic counters well. Extend that evidence to real work/unit-edge/integer-bit/occurrence/result ceilings, including enough and one-less boundaries and exact denial records. AC-7 needs a separate equal-map derived/derived test; the Time/Dur base/derived fixture exercises a different case.

## Coverage

- Reconciliation: quoin matrix (quoin 0.36.7, quire 0.52.0 / engine 0.62.2), plus quire coverage. No run evidence: 187 criteria.
- Plan completion: not assessed
- Matrix: 187 criteria; 160 tagged, 8 untagged, 19 method-without-symbol; all 7 FR-134 criteria tagged, none ignored-only.
- Inherited untagged criteria: FR-109-AC-6, FR-121-AC-6, FR-123-AC-6, FR-124-AC-6, FR-127-AC-14, FR-130-AC-1, FR-130-AC-3, FR-130-AC-4. These predate this API diff and remain assurance backlog, not claims that this PR fixes them.
- Coverage census: 324 Rust candidates, 312 tagged, 300 bound; 240 untracked symbols and 265 diagnostics across the repository, including external FSpec ids and existing marker issues. Not all are absent local requirements.
- Source inventory: 27 Rust files, module/public-surface inventory checked against README ownership and requirement references. New public behavior: graph projection plus typed refusal/cause, all owned by FR-134. New untraced behaviors: 0. Source TODO/unimplemented scans found no stubs; new test stubs: 0.
- Core paths examined: class row precedence, least structural representatives, mixed base/derived map normalization, unchanged nominal preimages, class root/path projection, declared/compound conversion admission, source/reverse arithmetic schedules, affine conversion, unchanged distinct-unit refusal, same/different overlay reprojection.
- Semantic gap fan-out: skipped; ordinary code-test contract checks above are part of code review, not an optional repository-wide semantic campaign.
- No new Cargo execution. Lead evidence is 9 filtered new tests and 2 existing handoff regressions passing; this review does not claim to have rerun them. Owner forbids full gates and whole-crate/workspace runs. No CI or lint policy changed.

## Examined scope and findings data

```yaml
{
  "clean": false,
  "scope": [
    {
      "id": "FR-134",
      "path": "spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md",
      "role": "examined",
      "excerpt": "When a caller supplies admitted dimension-class memberships, FSV SHALL construct an equivalence-aware runtime unit graph whose existing quantity operations traverse the identification edges under the caller's `Meter`."
    },
    {
      "id": "FR-134-AC-1",
      "path": "spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md",
      "role": "examined",
      "excerpt": "Valid scalar classes and an empty overlay return the specified graph; each malformed-row cause in Behavior 1 is independently reached, names the offending dimension, returns no graph and leaves the original graph unchanged; multiple defects follow the stated precedence."
    },
    {
      "id": "FR-134-AC-2",
      "path": "spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md",
      "role": "examined",
      "excerpt": "Forward and reverse results include the requested identified target unit."
    },
    {
      "id": "FR-134-AC-3",
      "path": "spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md",
      "role": "examined",
      "excerpt": "Injected accounting observes the existing forward/reverse rational-event order, even for scale one and offset zero."
    },
    {
      "id": "FR-134-AC-4",
      "path": "spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md",
      "role": "examined",
      "excerpt": "A formed single-root compound can be converted to a declared unit of the same normalized dimension and handed off through `identified_quantity`."
    },
    {
      "id": "FR-134-AC-5",
      "path": "spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md",
      "role": "examined",
      "excerpt": "Related distinct-unit addition, subtraction, equality and order refuse `DistinctUnits`; unrelated conversion refuses `IncompatibleDimensions`; affine arithmetic retains its existing refusal."
    },
    {
      "id": "FR-134-AC-6",
      "path": "spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md",
      "role": "examined",
      "excerpt": "Tight work, unit-edge, integer-bit, occurrence and result limits, and injected denial at each conversion charge point, produce the existing typed incomplete/refusal record at the first denied charge, with atomic counters and no completed result; ordinary same-dimension conversion has unchanged values and schedule under an empty overlay."
    },
    {
      "id": "FR-134-AC-7",
      "path": "spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md",
      "role": "examined",
      "excerpt": "unrelated equal-map derived dimensions still need explicit equivalence for direct declared conversion."
    }
  ],
  "findings": [
    {
      "fnd": "FND-001",
      "severity": "medium",
      "confidence": "high",
      "method": "gap-analysis",
      "check": "coverage",
      "artifact_id": "FR-134-AC-6",
      "related_ids": [
        "TC-931"
      ],
      "path": "tests/dimension_equivalence_conversion.rs",
      "lines": "637-678",
      "excerpt": "Tight work, unit-edge, integer-bit, occurrence and result limits, and injected denial at each conversion charge point, produce the existing typed incomplete/refusal record at the first denied charge, with atomic counters and no completed result; ordinary same-dimension conversion has unchanged values and schedule under an empty overlay.",
      "finding": "The tight-limit loop only sets each ceiling to zero and matches Outcome::Incomplete(_). It never verifies the limit kind, first denied charge, record, atomic counters, or the just-sufficient and one-less boundaries required by AC-6/TC-931 step 6. Injected-denial prefix tests do not establish actual resource-limit behavior."
    },
    {
      "fnd": "FND-002",
      "severity": "medium",
      "confidence": "high",
      "method": "gap-analysis",
      "check": "coverage",
      "artifact_id": "FR-134-AC-7",
      "related_ids": [
        "TC-931"
      ],
      "path": "tests/dimension_equivalence_conversion.rs",
      "lines": "599-633",
      "excerpt": "unrelated equal-map derived dimensions still need explicit equivalence for direct declared conversion.",
      "finding": "The tagged test covers a base/derived pair only. No test constructs two distinct derived dimensions with equal normalized maps, verifies direct declared conversion refuses before charging without membership, then admits explicit equivalence and verifies exact converted value/target identity. This independent AC-7/TC-931 step-7 branch is untested."
    }
  ],
  "bindings": []
}
```

## Dispositions

### Round 1

Fix head ec4e3c7ce7ae1f4245aefca087fd78c6977f497b inspected directly. Actual resource ceilings now test exact sufficient and one-less bounds across five limit kinds, the first denied charge, full typed record and atomic counter prefix. A new public-API test covers distinct equal-map derived dimensions, unchanged meter on refusal, explicit membership, exact converted value and target identity.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Inspected the fix commit and its real public-API assertions. |
| FND-002 | fixed | Inspected the fix commit and its real public-API assertions. |

MERGEABLE for this PR's scoped implementation. No new findings. Original findings and the original verdict remain historical; these outcomes supersede them. Eight inherited repository matrix gaps remain separate assurance backlog; this disposition claims no repository-wide assurance completion. Lead reports 10 equivalence tests and 2 handoff regressions passing, plus focused Clippy; no Cargo command was run by this reviewer.

Artifact custody: commit this updated file as reviews/qsl-2734-pr118-gap-analysis.md. The PR-qualified path avoids overwriting PR117's existing same-basename artifact; the SR id is unchanged.

```yaml
{
  "dispositions": [
    {
      "fnd": "FND-001",
      "outcome": "fixed",
      "after_excerpt": "Outcome::Incomplete(Incomplete {\n    limit_kind: kind,\n    limit: bound,\n    consumed: prefix_consumed(&capture.events()[..index], kind),\n    next_charge: event_request(event, kind),\n    charge_point: event.point\n})"
    },
    {
      "fnd": "FND-002",
      "outcome": "fixed",
      "after_excerpt": "fn dimension_equivalence_conversion_equal_map_derived_nodes_require_explicit_membership() {"
    }
  ]
}
```



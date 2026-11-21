---
id: SR-1794040450272172418994
title: "QSL-2734 PR118 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-semantic-value; src/, tests/, spec/; QSL-2734 PR118 at 9f898af6a0178869b05fc7b74e8183c1b45d3be2"
review_set: subset
---

## Summary

Independent code-review of frozen PR118. Ticket: QSL-2734. FR-134 and TC-931 are landed requirements. No applicable AssuranceProfile was found. Rust-review and rust-style are folded into this artifact. One high duplication finding blocks merge.

## Verdict

FAIL; not mergeable until FND-001 is resolved.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | nominal_path reimplements the admitted unit-to-root traversal, affine edge composition and path construction already owned by unit_paths at src/unit.rs:677-690. Keep one path-composition implementation or retain nominal paths for reprojection; do not maintain two copies. | src/unit/equivalence.rs:221-249 |

## Evidence and correction

Searched src/unit.rs and src/unit/equivalence.rs for unit path construction. Existing unit_paths walks target links, composes canonical.followed_by(edge, UnmeteredAffine), appends edges and returns the terminal root. The new nominal_path repeats all of that against nominal metadata. Reprojection needs the original paths, but should consume one shared implementation or retained nominal results. No need to change runtime metered conversion.

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
      "severity": "high",
      "confidence": "high",
      "method": "code-review",
      "check": "other",
      "artifact_id": "FR-134",
      "related_ids": [
        "FR-134-AC-7"
      ],
      "path": "src/unit/equivalence.rs",
      "lines": "221-249",
      "excerpt": "When a caller supplies admitted dimension-class memberships, FSV SHALL construct an equivalence-aware runtime unit graph whose existing quantity operations traverse the identification edges under the caller's `Meter`.",
      "finding": "nominal_path reimplements the admitted unit-to-root traversal, affine edge composition and path construction already owned by unit_paths at src/unit.rs:677-690. Keep one path-composition implementation or retain nominal paths for reprojection; do not maintain two copies."
    }
  ],
  "bindings": []
}
```


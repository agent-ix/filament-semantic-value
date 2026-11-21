---
id: SR-1794040450272172418991
title: "QSL-2734 PR117 spec-review/integrity"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-semantic-value; FR-134 and TC-931; spec-only PR117"
review_set: subset
---

## Summary

Ticket: QSL-2734. Independent review of frozen cd4d0fccb55633cbab981e86580023c8b5c8e32a. The new overlay contract contradicts its owning FSpec rule for admissible mixed base/derived classes.

No applicable AssuranceProfile was found under spec/. Review set selected by lead: base plus applicable integrity, EARS and dependency analyses. No enforced run is required. No branch edits or builds were performed.

## Verdict

FAIL for this method. Overall PR117 is not mergeable while the integrity finding remains open.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Behavior 1 rejects all mixed base/derived equivalence classes, but FSpec FR-142 admission step 6 allows a pair not both base when normalized maps and base-implied root ratios agree; a valid Time ~ Dur=Time class would be refused. | spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md:92-94 |

## Evidence and correction

FSpec FR-142, Dimension equivalence admission step 6, applies to every pair whose nodes are not both base; it permits equal normalized maps with the base-implied ratio. For example, base Time and derived Dur=Time may be explicitly related at scale one. FR-134 currently rejects this before constructing its graph. Remove the unconditional mixed-class refusal, preserve FSpec's distinct base-class normalization versus general conversion classes, and add a positive mixed base/derived fixture plus incompatible-map refusal. Caller-admitted derived ratios remain caller-owned; do not add a second declaration admission engine.

## Examined scope

```yaml
{
  "clean": false,
  "scope": [
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
      "excerpt": "With Time representative root `s`, Beat root `beat`, `bar = 4 beat`, `ms = s/1000`, and Beat root size `1/4`, exact conversions yield `1 beat -> 1/4 s`, `3 s -> 12 beat`, and `1 bar -> 1000 ms`; at size one, `1 beat -> 1 s`. Forward and reverse results include the requested identified target unit."
    },
    {
      "id": "FR-134-AC-3",
      "path": "spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md",
      "role": "examined",
      "excerpt": "A root-to-representative conversion traverses one identification edge, a conversion between two nonrepresentative roots traverses two, and bar-to-ms traverses its two ordinary edges plus the one Beat identification edge. Injected accounting observes the existing forward/reverse rational-event order, even for scale one and offset zero."
    },
    {
      "id": "FR-134-AC-4",
      "path": "spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md",
      "role": "examined",
      "excerpt": "At Beat size `1/4`, `6 s / 3 beat` completes with magnitude `8` and the empty dimensionless compound unit, equal to `6 s / (3/4 s)`; multiplying or squaring equivalent root operands yields the same representative-root compound identity. A formed single-root compound can be converted to a declared unit of the same normalized dimension and handed off through `identified_quantity`."
    },
    {
      "id": "FR-134-AC-5",
      "path": "spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md",
      "role": "examined",
      "excerpt": "Related distinct-unit addition, subtraction, equality and order refuse `DistinctUnits`; unrelated conversion refuses `IncompatibleDimensions`; affine arithmetic retains its existing refusal. Each leaves meter counters unchanged. A valid affine conversion with an ordinary offset edge and a class identification edge still completes with its exact expected value."
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
      "excerpt": "The overlay and declared table retain original nominal keys, nominal dimension references and preimages. Equivalent root products normalize their runtime dimensions and compound terms without changing declared ids or conflating declared and compound identity arms; unrelated equal-map derived dimensions still need explicit equivalence for direct declared conversion."
    },
    {
      "id": "FR-134",
      "path": "spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md",
      "role": "examined",
      "excerpt": "When a caller supplies admitted dimension-class memberships, FSV SHALL construct\nan equivalence-aware runtime unit graph whose existing quantity operations\ntraverse the identification edges under the caller's `Meter`."
    },
    {
      "id": "FR-134 Behavior 1",
      "path": "spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md",
      "role": "examined",
      "excerpt": "representative scale not one (`NonIdentityRepresentative`), or a class\n   mixing base and derived dimensions or unequal normalized derived maps\n   (`IncompatibleClass`)."
    },
    {
      "id": "TC-931",
      "path": "spec/test-cases/TC-931-metered-dimension-equivalence.md",
      "role": "examined",
      "excerpt": "Verify [FR-134](../functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md)\nusing FSV's public graph, unit-table and quantity APIs."
    }
  ],
  "findings": [
    {
      "fnd": "FND-001",
      "severity": "high",
      "confidence": "high",
      "method": "spec-review/integrity",
      "check": "soundness",
      "artifact_id": "FR-134",
      "related_ids": [
        "FR-134-AC-1",
        "FR-134-AC-7",
        "TC-931",
        "FSpec-FR-142"
      ],
      "path": "spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md",
      "lines": "92-94",
      "excerpt": "representative scale not one (`NonIdentityRepresentative`), or a class\n   mixing base and derived dimensions or unequal normalized derived maps\n   (`IncompatibleClass`).",
      "finding": "Behavior 1 rejects all mixed base/derived equivalence classes, but FSpec FR-142 admission step 6 allows a pair not both base when normalized maps and base-implied root ratios agree; a valid Time ~ Dur=Time class would be refused."
    }
  ],
  "bindings": []
}
```

## Validation limits

quire 0.52.0 (engine 0.62.2), quoin 0.36.7. Changed documents validate grammar-clean. Full spec validation exits0 with seven pre-existing warnings in FR-110/121/132. Computed matrix inspected: FR-134 AC1-7 each has no runtime binder yet; implementation evidence remains pending by design. No tests, full gate or human-acceptance workflow was run or claimed.


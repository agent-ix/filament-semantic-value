---
id: TC-931
title: "Dimension equivalence preserves exact values, identities and meter boundaries"
type: TC
relationships:
  - target: ix://agent-ix/filament-semantic-value/FR-134
    type: verifies
---
# TC-931: Dimension equivalence preserves exact values, identities and meter boundaries

## Description

Verify [FR-134](../functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md)
using FSV's public graph, unit-table and quantity APIs. Construct structural
keys whose ordering makes Time the class representative, and a Time root `s`,
unit `ms = s/1000`, Beat root `beat`, unit `bar = 4 beat`, and a third member
root `pulse`. Use exact rational scales, not floating-point tolerances.

## Test Procedure

1. Project a class with Time self-row scale one, Beat scale `1/4`, and Pulse
   scale `1/2`. Test an empty overlay and every malformed-row cause separately,
   then selected simultaneous defects to verify precedence. Snapshot the
   original graph before each call and compare it afterwards (AC-1).
2. Run exact beat-to-second, second-to-beat and bar-to-millisecond conversions,
   then a separate Beat scale-one class. Check the exact values and both
   members returned by `Conversion::identified_quantity` (AC-2).
3. Observe named charge points through the existing meter injection seam for
   conversions Beat-to-Time, Beat-to-Pulse, and bar-to-ms, in both directions.
   Check one, two and three total traversed edges respectively, all edge
   charges before rational events, and forward multiply/add versus reverse
   subtract/divide events. Include a scale-one identification edge (AC-3).
4. Evaluate `6 s / 3 beat` and `6 s / (3/4 s)`, compare their magnitude `8`
   and empty compound terms. Compare products and powers formed from the two
   class roots, then convert a newly formed single-root compound to a declared
   target and resolve the identified result in `UnitScope` (AC-4).
5. Attempt add, subtract and each comparison between seconds and beats, and
   convert an unrelated Length quantity to seconds. Exercise the existing
   affine arithmetic refusal. Add an ordinary affine unit `shifted` targeting
   `beat` with scale one and offset two: exact `1 shifted -> 3/4 s` must still
   complete via its ordinary affine edge and the zero-offset class edge
   (AC-5).
6. Starting from a successful conversion's measured schedule, reduce work,
   unit-edge, integer-bit, occurrence and result ceilings at their individual
   boundaries. Inject denial at identity-read, edge, rational-arithmetic,
   target-domain and result-retain. Compare the returned record and counters
   with the attempted charge and assert absence of a completed conversion.
   Compare an empty overlay's existing same-dimension conversion with the
   original graph's result and schedule (AC-6).
7. Compare graph and table nominal preimages before and after projection,
   along with `dimension_node()` and declared unit ids. Check representative
   compound terms, combined and cancelled exponents, and declared/compound
   identity separation. Create two derived dimensions of equal maps without
   a membership row and assert their direct declared conversion is refused;
   add admitted scale-one membership and assert it is admitted (AC-7).

## Expected Results

- Overlay refusals match the named cause/key and leave the original graph
  unchanged. The successful star graph charges its actual projected edges,
  independently of the number of equivalence declarations used by the caller.
- Exact scalar and compound values, requested target units and structural
  identities match the independent rational and root-term expectations.
- Distinct-unit and incompatible-dimension refusals occur before charging;
  ordinary affine conversion remains valid, while affine arithmetic refuses.
- Every denied charge preserves its atomic accounting record and returns no
  completed result. No FSL-specific callback or unmetered runtime arithmetic
  is needed to obtain any expected value.
- Source/package admission and BareWorld source-to-result tests remain the
  FSL lane's separate acceptance evidence for QSL-2734.

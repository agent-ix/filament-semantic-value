---
id: SR-2734121001
title: 'Code review of FSV PR #121: derived-root scale under dimension equivalences'
type: SpecReview
analysis: code-review
scope: 'agent-ix/filament-semantic-value@2563bb773ff2; PR #121 diff origin/main...HEAD: src/unit/equivalence.rs, tests/dimension_equivalence_conversion.rs; ticket QSL-2734'
review_set: subset
date: '2026-11-21'
relationships:
- target: ix://agent-ix/filament-semantic-value/FR-134
  type: reviews
---

## Summary

Ticket: QSL-2734. PR filament-semantic-value#121, reviewed at head 2563bb773ff289d8e7c5c5cc6376727ee7aef164 (code-review with the rust-review lane). The scale direction, edge order, affine members, unlisted bases and derived-of-derived refusals were checked and are clean. Three findings were raised and all three are fixed in fix round 1.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| coherent_scales and the coherence check | src/unit/equivalence.rs | examined |
| coherence edge append | src/unit/equivalence.rs | examined |
| derived-root tests | tests/dimension_equivalence_conversion.rs | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The contradiction check never fires for a derived representative of a mixed class, so refusal depended on node-key order. Fixed: every derived member is checked against the least base member (or the representative's coherent scale when the class has no base member); test `dimension_equivalence_derived_representative_must_agree_with_its_base_members` builds both key orders. | FR-134, FR-142 |
| FND-002 | medium | The coherence factor was a charged unit edge, adding edges to same-dimension derived conversions that FR-142 does not count. Fixed per arch ruling (b): the factor is stored on the unit and applied uncharged at the compound pivot only; paths and edge counts are as before the PR. | FR-134-AC-3, FR-142 |
| FND-003 | low | Coherence was compared before every row was validated, so a malformed base row could be masked by an IncompatibleClass on an earlier derived row. Fixed: the comparison is a second loop after the per-row loop. | FR-134 |
| FND-004 | low | The FND-003 regression assertion put the malformed base row before the derived rows, so a single-loop check passed it too. Fixed in fix round 2: the malformed base row is moved after the derived rows and must still report NonPositiveScale. | FR-134 |

## Verdict

Disposition pass 1 at f382896da0efc98aef54129a0c626323547c5044: FND-001, FND-002 and FND-003 fixed; FND-004 raised and fixed in fix round 2.

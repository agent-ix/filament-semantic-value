---
id: SR-2734121002
title: 'Spec review of FSV PR #121: FR-134 derived-root coherence paragraph'
type: SpecReview
analysis: integrity
scope: 'agent-ix/filament-semantic-value@2563bb773ff2; FR-134 behavior 3 paragraph added by PR #121 against FSpec FR-142 derived roots and step 6; ticket QSL-2734'
review_set: subset
date: '2026-11-21'
relationships:
- target: ix://agent-ix/filament-semantic-value/FR-134
  type: reviews
---

## Summary

Ticket: QSL-2734. Review of the FR-134 behavior 3 paragraph that PR #121 adds, compared with FSpec FR-142 derived roots and step 6. Two findings were raised and both are fixed in fix round 1.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-134 behavior 3 | spec/functional/FR-134-meter-dimension-equivalence-through-the-unit-graph.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The refusal sentence covered only non-representative derived members. Fixed: it now states the check against the least base member, or the representative's coherent scale when the class has no base member. | FR-142, FR-134-AC-7 |
| FND-002 | medium | The paragraph made the coherence step a charged edge, which FR-142 does not count. Fixed: the paragraph states the factor is applied at the compound pivot and is not a charged edge. | FR-142, FR-134-AC-3 |

## Verdict

All findings fixed in fix round 1; re-review pending.

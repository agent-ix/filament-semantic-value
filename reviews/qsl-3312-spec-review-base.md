---
id: SR-2734121004
title: "Base spec review — QSL-3312 FR-135 clock-reference update"
type: SpecReview
analysis: base
scope: "agent-ix/filament-semantic-value; FR-135 Behaviors 3-4 and AC-3/4, TC-932, tests/clock_reference.rs"
review_set: subset
---

## Summary

Checked the changed FR-135 criteria, TC-932 procedure and expected results, and their tagged test binders. The revised equality rule now matches FR-415/FR-829, but the `Option` and `Map` closure tests do not verify that the returned universe payload is preserved. No applicable AssuranceProfile appears in this repository's spec tree.

## Verdict

**CONDITIONAL** — one medium coverage gap remains for FR-135-AC-4's unchanged-read obligation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-135-AC-4 requires the bare, `Option`, and `Map` closure forms to read their clock reference back unchanged. The test checks the universe payload for the bare form only; after identity changed to place-id text alone, closure equality cannot prove that the `Option` and `Map` forms preserve the readable universe. Read each nested value back and assert its place-id text and universe. | FR-135-AC-4, tests/clock_reference.rs:374, tests/clock_reference.rs:391 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The closure test now reads the `Option` payload and the `Map` entry value, checking both text and universe for each. After excerpt: `actual.text() == place_text("c/d") && actual.universe() == &one`. |

## Verdict after disposition pass 1

**PASS** — the only base-review coverage finding is fixed.

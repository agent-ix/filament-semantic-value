---
id: SR-2734121003
title: "Code review — filament-semantic-value pin alignment"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-semantic-value; Cargo.toml, Cargo.lock, tests/clock_reference.rs, spec/functional/FR-135-clock-place-reference-in-declaration-admission-membership-and-closure.md, spec/test-cases/TC-932-clock-place-reference-in-admission-membership-and-closure.md"
review_set: subset
---

## Summary

Reviewed the exact `filament-exact` dependency pin and lockfile update for QSL-3312, along with the affected semantic-value requirement and test. The exact pin is consistent with the merged kernel commit, but it changes equality behavior while the local FR-135-AC-3 and its test still require cross-universe refusal; the recorded focused test fails.

## Verdict

**FAIL** — the pin cannot be consumed while it leaves the crate's owning requirement and focused test incompatible with the dependency's current behavior and the governing FR-829/FR-415 requirements.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Advancing the kernel pin changes equal-text cross-universe equality to `true`, while FR-135-AC-3 and `equality_follows_the_kernel_decision_for_clock_references` still require `ForeignReference`; the focused test fails and the local requirement contradicts current FR-415/FR-829. Align FR-135-AC-3 and its test with the governing equality behavior in this PR before merging the pin. | Cargo.toml:17, tests/clock_reference.rs:305, spec/functional/FR-135-clock-place-reference-in-declaration-admission-membership-and-closure.md:96 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-135-AC-3 and its traced equality test now require equal place-id text to compare `true` across universes, matching FR-415/FR-829. After excerpt: `check_equality` of two `ClockReference` operands succeeds and its evaluation returns `false` for two place-id texts that differ in `K` in one universe and for unequal texts across universes, and `true` for equal place-id text in one or two universes; `check_equality` of `ClockReference` against `Reference(t)` is refused `TypeMismatch`. |
| FND-002 | fixed | The Option and Map closure cases now read back the nested clock reference and assert both its text and universe payload. After excerpt: `actual.text() == place_text("c/d") && actual.universe() == &one` (asserted for `option.payload()` and `map_value.entries()`). |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | FR-135-AC-4 requires each bare, `Option`, and `Map` closure to read the supplied clock reference back unchanged, including its readable universe payload. The test checks the returned universe only for the bare form; `ObjectClosure` equality now uses place-id text alone, so the `Option` and `Map` equality assertions can pass even if their nested universe payload is lost. Assert the returned text and universe for both nested forms. | tests/clock_reference.rs:374, tests/clock_reference.rs:391, spec/functional/FR-135-clock-place-reference-in-declaration-admission-membership-and-closure.md:100 |

## Verdict after disposition pass 1

**CONDITIONAL** — FND-001 is fixed; the new medium FND-002 remains open for the nested closure readback payload.

## Verdict after disposition pass 2

**PASS** — FND-001 and FND-002 are fixed; no new code-review finding remains.

---
id: SR-1794040450272172418996
title: "Code review of semantic-value WorkScope exact pin"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-semantic-value; Cargo.toml, Cargo.lock; QSL-2158"
review_set: subset
---

## Summary

Ticket QSL-2158, PR122. Independent Codex review of frozen head 2cb2a87a96cdd4064b34e6351b17306f3dbc2545 against fa5e78aa71fcea5418492b67f13a33124af84459. The complete diff is two additions and two removals in Cargo.toml and Cargo.lock, replacing only the filament-exact revision b26a8e4c592503fccdc4a0e3d2cce23dd8487616 with accepted a58474cc105a86f3307a11def3fc001e2459679f.

## Verdict

**PASS** — no defects found in the scoped pin change. The manifest revision, lock query and resolved fragment agree. There is one exact package record, retaining version 0.3.0. Byte comparison after revision normalization proves all other manifest and lock bytes, dependencies, package records, features and pins unchanged. json-object and test-support remain intact; the derived-root implementation, tests and specification remain intact. No source, version, CI or gate configuration changes occur.

## Scope

Code-review with the rust-review lane and rust-style baseline applied to Rust dependency configuration. Repository AGENTS.md, CLAUDE.md, Rust toolchain/lint/deny/formatter configuration were read as context. No applicable AssuranceProfile was found in spec/. Source-specific API, panic, concurrency, wire and test-change checks are not applicable because there is no source or test diff. No copying or vendoring is introduced.

```yaml
{
  "clean": true,
  "scope": [
    {
      "id": "manifest-exact-pin",
      "path": "Cargo.toml",
      "role": "examined",
      "excerpt": "filament-exact = { git = \"https://github.com/agent-ix/filament-exact\", rev = \"a58474cc105a86f3307a11def3fc001e2459679f\" }"
    },
    {
      "id": "lock-exact-source",
      "path": "Cargo.lock",
      "role": "examined",
      "excerpt": "source = \"git+https://github.com/agent-ix/filament-exact?rev=a58474cc105a86f3307a11def3fc001e2459679f#a58474cc105a86f3307a11def3fc001e2459679f\""
    },
    {
      "id": "feature-contract",
      "path": "Cargo.toml",
      "role": "examined",
      "excerpt": "json-object = [\"filament-exact/json-object\"]\n# The test-only reservation permit of `SuppliedRun`. Only a `[dev-dependencies]`\n# entry enables it.\ntest-support = []"
    }
  ],
  "findings": [],
  "bindings": []
}
```

## Dependency provenance

GitHub exact main resolved to a58474cc105a86f3307a11def3fc001e2459679f. The GitHub comparison from b26a8e4c592503fccdc4a0e3d2cce23dd8487616 to a58474cc105a86f3307a11def3fc001e2459679f reports ahead by one, behind by zero, with the previous pin as merge base. Its complete changed-file list contains no Cargo.toml or Cargo.lock, proving no upstream dependency-manifest delta; the accepted Cargo.toml declares filament-exact 0.3.0. The local exact checkout lacks the accepted object; provenance was checked against the owner repository API rather than relying on stale local refs.

## Validation evidence

The dispatching lead supplies actual Titan results at 2cb2a87a96cdd4064b34e6351b17306f3dbc2545: identified_quantity 11 plus dimension_equivalence_conversion 15, totaling 26 passed, 0 failed, 0 ignored; cargo build --locked --target thumbv7em-none-eabi exit 0; cargo check --locked --all-targets --all-features exit 0. These are supplied execution evidence, not local reviewer executions. The brief prohibits local Cargo; fmt, clippy, full tests and cargo-deny were therefore not run by this reviewer. git diff --check passed. The lead retains the final rebased exact-head all-target gate before merge.

## Method applicability

Gap-analysis: N/A for this pin-only diff; no production source or acceptance criteria change and no new implementation obligation. Spec-review and every spec sub-analysis: N/A; spec/ and plan/ are unchanged. No full matrix/spec campaign was launched. This review does not implement or accept FSL cumulative accounting and does not close QSL-2158.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Tracker record

One private QSL-2158 review comment posted: b6d85eec-5cb2-4634-8998-29bf7b4d70f9. The initial comment page contained no matching marker for this newly allocated owner-repository SR id. The one permitted readback found the saved comment id and matched the body byte-for-byte; the JSON-compatible YAML parses with clean true, three scope entries, and empty findings/bindings. Scope excerpts were checked verbatim against the frozen files before posting. No GitHub findings or excerpts were posted; no issue status or label mutation was made.

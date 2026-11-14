---
id: FR-130
title: "Share one input-path admission record below FSL and filament-eval"
type: FR
relationships:
  - target: ix://agent-ix/filament-semantic-value/FR-126
    type: depends_on
  - target: ix://agent-ix/filament-spec-language/FR-106
    type: references
---
# FR-130: Share one input-path admission record below FSL and filament-eval

## Description

`filament-semantic-value` SHALL hold the one shared input-path admission record
and the canonical generic observation-admission result that FSL and
`filament-eval` can both name without either depending on the other. The record
describes the diagnostic; the result preserves its admission disposition. FSL
[FR-106](ix://agent-ix/filament-spec-language/FR-106) continues to own observation
admission, its check order, and the decision to admit, refuse, or return
incomplete.

## Outputs

- A public, `no_std`-compatible `AdmissionRecord` containing a `CatalogCode`
  whose `code()` and `cause()` return the exact catalog pair; a
  `BTreeMap<&'static str, String>` of named input fields; an ordered
  `Vec<PathStep>`; and an `Option<filament_exact::ConstructionCause>` named
  `kernel`.
- A public `PathStep` with exactly `Field(String)`, `OptionValue`, and
  `Index(usize)` variants. Steps run outermost first from the checked field,
  parameter, or result to the nested value. An empty path names that checked
  value itself. A field called `"0"` remains `Field("0")`, distinct from
  `Index(0)`.
- A public, `no_std`-compatible, generic `ObservationAdmission` with
  `Admitted`, `Refused`, `Incomplete`, and `Fault` arms. It is generic over the
  admitted authority; any fault payload that cannot be expressed using FSV's
  own `InternalFault` SHALL be a type parameter, never a dependency on
  `filament-eval`.

## Behavior

1. FSV SHALL preserve the exact catalog code and cause, each named field and
   value, each typed path step in order, and the kernel construction cause when
   building or returning an `AdmissionRecord`. It SHALL NOT render the typed
   path into a string or discard the kernel cause.
2. `filament-eval` SHALL carry this FSV record in its call-admission failure.
   `filament-eval` SHALL NOT declare a second copy of its code, cause, fields, path, or kernel
   fields. Its `CallAdmissionLocus` SHALL distinguish population nodes from
   node, member, and binder loci, naming the population at which the fault was
   observed. This locus identifies the evaluator's population context; it
   does not replace the record's input path.
3. The observation admission implementation SHALL populate the FSV record at
   the FSL FR-106 decision site where the original defect is classified. A
   call admission implementation SHALL populate the same record shape at its
   own `filament-eval` admission decision site. Passing either result through
   another layer SHALL retain the original record and failure kind, without
   reconstructing it from a rendered message or a catalog code alone.
4. An observation API reporting admission SHALL return FSV's typed result with
   `Admitted`, `Refused`, `Incomplete`, and `Fault` arms. `Admitted` SHALL
   carry the admitted authority. `Refused` and `Incomplete` SHALL each carry
   the original input-path admission record and remain distinct. The `Fault`
   arm SHALL carry the original `InternalFault` and remain distinct from both
   input-failure arms. A query for a missing successful authority SHALL
   return the original refusal, incomplete result, or internal fault rather
   than only `None`. `filament-eval` SHALL use and publicly re-export this FSV
   result without redefining a parallel result or changing its existing
   consumer-facing arm shapes.
5. FSV SHALL own only the shared record, path, and result vocabulary. FSV SHALL
   NOT depend on FSL or `filament-eval`. FSV SHALL NOT implement FSL FR-106 admission
   behavior. `filament-eval` SHALL NOT depend on FSL to carry the record or result.
   FSL's layer-4 `fsl-package` SHALL import the FSV result without depending on
   `filament-eval`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-130-AC-1 | A shared record built with `invalid_runtime_input`/`invalid-value`, fields for document, population, object and field, path `[Field("0"), OptionValue, Index(2)]`, and `Some(ConstructionCause::TypeMismatch)` reads back the same code, cause, fields, path and kernel variant. `Field("0")` remains distinct from `Index(0)`. | Test |
| FR-130-AC-2 | FSL and `filament-eval` both compile against FSV's one public record and path types; FSV and `filament-eval` have no dependency on FSL, and `filament-eval` defines no parallel input-path record fields. | Inspection |
| FR-130-AC-3 | For a population-level admission defect, `CallAdmissionLocus` names the population and the call failure retains the original FSV record. Existing node, member and binder loci still identify their respective nodes. | Test |
| FR-130-AC-4 | An observation refused at a nested value returns a typed `Refused` arm with its original code/cause, named input fields, typed path and kernel cause. An incomplete observation returns a distinct `Incomplete` arm retaining its record; an admission invariant failure returns a distinct `Fault` arm retaining the original `InternalFault`. A successful observation returns an `Admitted` arm with its authority. The missing-authority query exposes each original non-admitted arm rather than only `None`. | Test |
| FR-130-AC-5 | `ObservationAdmission` is one public generic FSV type with the four arms in AC-4. `filament-eval` uses and publicly re-exports that type without a second enum or changed consumer-facing arm shapes. FSV has no FSL or `filament-eval` dependency, and FSL layer-4 `fsl-package` can use the type without a `filament-eval` dependency. | Inspection |

## Dependencies

- [FR-126](./FR-126-own-the-evaluation-result-vocabulary.md) owns
  `CatalogCode`.
- FSL [FR-106](ix://agent-ix/filament-spec-language/FR-106) owns the observation
  admission semantics and QSpec FR-207 defines the typed nested path.

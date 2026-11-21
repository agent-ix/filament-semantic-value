---
id: FR-134
title: "Meter dimension equivalence through the unit graph"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-142
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-093
    type: references
  - target: ix://agent-ix/filament-semantic-value/FR-124
    type: references
  - target: ix://agent-ix/filament-semantic-value/FR-129
    type: references
---
# FR-134: Meter dimension equivalence through the unit graph

## Description

When a caller supplies admitted dimension-class memberships, FSV SHALL construct
an equivalence-aware runtime unit graph whose existing quantity operations
traverse the identification edges under the caller's `Meter`.

This is the shared API dependency of QSL-2734. FSpec FR-142, "Dimension
equivalence / Effect", owns the equivalence semantics. FSL owns declaration
admission, source diagnostics, checker integration, package/BareWorld
reconstruction and its dependency update. This requirement owns the FSV graph
projection and its quantity evaluation, not those consumer changes.

## Inputs

- An admitted `UnitGraph` and rows `(dimension, representative, root_scale)`.
- Every participating dimension, including its representative, appears once.
  `root_scale` is the exact positive size of that dimension's canonical root in
  canonical roots of its representative. The representative row has scale one.
- Quantity operands, targets and formed compound units from the same projected
  graph, and the existing quantity-operation target and meter.

The caller has admitted the equivalence declarations, their cycle consistency
and derived-dimension ratios. The rows describe those admitted classes; they
do not constitute another equivalence-declaration language.

## Outputs

- An owned projected `UnitGraph`, or a typed overlay refusal without a graph.
- Existing `convert_quantity`, `result_unit`, `evaluate_quantity_unit` and
  `compare_quantity` results and refusal/outcome types.

## Public API inventory

The new items are in `filament_semantic_value::unit`:

```rust
impl UnitGraph {
    pub fn with_dimension_equivalences(
        &self,
        members: &[(NodeKey, NodeKey, Rational)],
    ) -> Result<Self, DimensionEquivalenceRefusal>;
}

pub struct DimensionEquivalenceRefusal {
    pub dimension: NodeKey,
    pub cause: DimensionEquivalenceCause,
}

pub enum DimensionEquivalenceCause {
    UnknownDimension,
    DuplicateDimension,
    MissingRoot,
    MissingRepresentative,
    NonCanonicalRepresentative,
    NonPositiveScale,
    NonIdentityRepresentative,
    IncompatibleClass,
}
```

The refusal and cause implement `Clone`, `Debug`, `Eq`, `PartialEq` and
`thiserror::Error`. Existing quantity-function signatures remain unchanged.
This API computes graph metadata at admission time; it returns no converted
quantity. Runtime arithmetic remains in the existing metered quantity core.

## Behavior

1. FSV SHALL validate rows in supplied order before constructing an overlay.
   For each row, the first applicable cause wins: unknown member or
   representative dimension (`UnknownDimension`, naming that unknown key),
   repeated member (`DuplicateDimension`), either dimension lacking its own
   canonical root (`MissingRoot`, naming that dimension), representative
   missing a self-representative row (`MissingRepresentative`), representative
   not the least structural dimension key of the class
   (`NonCanonicalRepresentative`), scale not positive (`NonPositiveScale`),
   representative scale not one (`NonIdentityRepresentative`), or a class
   containing a derived member whose normalized map differs from another
   member's map after base-class normalization (`IncompatibleClass`). A mixed
   base/derived class with equal normalized maps is valid; the caller has
   already admitted its base-implied root ratios. Otherwise, FSV SHALL return
   the overlay. FSV SHALL
   leave the input graph unchanged on refusal. When the row slice is empty,
   FSV SHALL return an equivalent graph.
2. FSV SHALL retain every nominal node key, declared unit id,
   `Unit::dimension_node()` and nominal preimage unchanged in the projected
   graph and its `UnitTable::declared` projection. FSV SHALL maintain a separate
   conversion-class identity, with an unlisted dimension its own class.
   FSV SHALL normalize base-dimension keys using only the base members of each
   admitted class, choosing their least structural base-dimension key,
   combining exponents and removing zero terms. FSV SHALL keep that
   base-class normalization distinct from the general conversion-class
   representative: a derived member cannot replace a base key in a dimension
   map, even when it is the least member of the general conversion class.
3. For every nonrepresentative dimension, FSV SHALL append exactly one
   identification edge from its original canonical root to its class root,
   with scale `root_scale` and offset zero. The class root is the original
   canonical root of the representative dimension. FSV SHALL retain existing
   unit edges in source-to-root order before that edge. FSV SHALL add no
   identification edge to representative dimensions. FSV SHALL NOT substitute
   an original declaration-pair path for this star graph. A derived
   dimension's canonical root is coherent with the product of its base-class
   roots at their exponents, whether or not the dimension has a membership
   row: FSV SHALL append one coherence edge, with offset zero, to every unit
   of a class whose root dimension is derived, after any identification
   edge. Its scale is the product, over that dimension's base terms, of each
   base class's root scale (relative to the base-class representative) raised
   to the term's exponent, computed exactly with negative exponents and
   rational scales. FSV SHALL add no coherence edge at scale one. FSV SHALL
   refuse `IncompatibleClass` for a listed derived member whose root scale
   differs from its coherent scale divided by its representative's.
4. When direct conversion is requested between declared units, FSV SHALL
   admit it exactly when their original dimension nodes are equal or their
   conversion-class identities are equal. For a conversion with a compound
   side, FSV SHALL retain the existing normalized-dimension-map rule. FSV SHALL
   traverse the full source-to-class-root path followed by the reversed
   target path. FSV SHALL charge every edge before every rational event as in
   the existing conversion schedule. FSV SHALL use the existing
   multiply-then-add schedule for forward identification edges, including
   the zero offset. FSV SHALL use subtract-then-divide for reverse
   identification edges. FSV SHALL NOT
   replace these charged events by unmetered multiplication of composed
   scales or by a consumer callback.
5. FSV SHALL use class roots when `result_unit` and quantity evaluation form
   products, quotients and integer powers. A declared operand's runtime
   value SHALL reach that same class root through its charged edge path.
   Compound terms SHALL combine by representative root, so equal roots cancel
   in a quotient and like terms combine in a product or power. The existing
   structural identity and bounded exponent handling of
   [FR-129](FR-129-bound-compound-unit-exponent-reservations.md) SHALL apply.
   An already formed compound value SHALL NOT be relabeled to another graph's
   roots without the corresponding metered value conversion. Consumers SHALL
   form or reconstruct a quantity and its units under one graph context.
6. FSV SHALL preserve identical-unit requirements for addition, subtraction,
   equality and ordering. Related but distinct units SHALL refuse
   `DistinctUnits` before meter charges; unrelated dimensions SHALL refuse
   `IncompatibleDimensions` before charges. Existing affine arithmetic
   refusals SHALL remain unchanged. Existing affine conversions SHALL remain
   admitted where conversion is otherwise valid, traversing and charging
   their ordinary offset edges; the zero-offset restriction applies to the
   identification edge, not every declared unit in a class.
7. FSV SHALL pass each existing meter refusal or incomplete record through
   unchanged at identity-read, edge, rational-arithmetic, target-domain and
   result-retain boundaries. A denied charge SHALL retain its existing atomic
   accounting behavior and return no completed conversion. Conversion target
   admission, rounding/loss behavior and the identified-quantity handoff of
   [FR-124](FR-124-identified-quantity-from-an-exact-conversion.md) SHALL remain
   unchanged. An overlay with no class memberships SHALL preserve existing
   result values, identities and charge schedules.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-134-AC-1 | Valid scalar classes and an empty overlay return the specified graph; each malformed-row cause in Behavior 1 is independently reached, names the offending dimension, returns no graph and leaves the original graph unchanged; multiple defects follow the stated precedence. | Test |
| FR-134-AC-2 | With Time representative root `s`, Beat root `beat`, `bar = 4 beat`, `ms = s/1000`, and Beat root size `1/4`, exact conversions yield `1 beat -> 1/4 s`, `3 s -> 12 beat`, and `1 bar -> 1000 ms`; at size one, `1 beat -> 1 s`. Forward and reverse results include the requested identified target unit. | Test |
| FR-134-AC-3 | A root-to-representative conversion traverses one identification edge, a conversion between two nonrepresentative roots traverses two, and bar-to-ms traverses its two ordinary edges plus the one Beat identification edge. Injected accounting observes the existing forward/reverse rational-event order, even for scale one and offset zero. | Test |
| FR-134-AC-4 | At Beat size `1/4`, `6 s / 3 beat` completes with magnitude `8` and the empty dimensionless compound unit, equal to `6 s / (3/4 s)`; multiplying or squaring equivalent root operands yields the same representative-root compound identity. A formed single-root compound can be converted to a declared unit of the same normalized dimension and handed off through `identified_quantity`. | Test |
| FR-134-AC-5 | Related distinct-unit addition, subtraction, equality and order refuse `DistinctUnits`; unrelated conversion refuses `IncompatibleDimensions`; affine arithmetic retains its existing refusal. Each leaves meter counters unchanged. A valid affine conversion with an ordinary offset edge and a class identification edge still completes with its exact expected value. | Test |
| FR-134-AC-6 | Tight work, unit-edge, integer-bit, occurrence and result limits, and injected denial at each conversion charge point, produce the existing typed incomplete/refusal record at the first denied charge, with atomic counters and no completed result; ordinary same-dimension conversion has unchanged values and schedule under an empty overlay. | Test |
| FR-134-AC-7 | The overlay and declared table retain original nominal keys, nominal dimension references and preimages. Equivalent root products normalize their runtime dimensions and compound terms without changing declared ids or conflating declared and compound identity arms; unrelated equal-map derived dimensions still need explicit equivalence for direct declared conversion. Base Time and derived Dur=Time with root units s and dsec form an admitted mixed class at scale one and convert 3 s to exact 3 dsec; a Time/Area=Time^2 class refuses IncompatibleClass. Base dimension maps retain their base representative even when a derived member is the least conversion-class key. | Test |

## Dependencies

- [FSpec FR-142](ix://agent-ix/filament-specification/FR-142), especially
  AC-23/24/26, defines class graphs, compound normalization and charge semantics.
- [FSL FR-093](ix://agent-ix/filament-spec-language/FR-093), AC-46, owns the
  source-level scalar and quotient cases. QSL-2734's FSL lane owns checker,
  evaluator and BareWorld wiring and pin adoption after this shared API lands.
- [TC-931](../test-cases/TC-931-metered-dimension-equivalence.md) defines the
  shared-library fixtures; it does not claim consumer integration passes.

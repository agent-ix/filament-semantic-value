// SPDX-License-Identifier: AGPL-3.0-or-later
//! FR-134 / TC-931 public graph projection and metered quantity evaluation.
mod common;

use filament_exact::{
    rational_arithmetic_bits, ChargeAdmission, ChargeEvent, ChargePoint, ComparisonOperator,
    IllTypedCause, Incomplete, InjectedDenial, Integer, LimitKind, Meter, NodeKey, Outcome,
    Quantity, Rational, RationalArithmetic, ScalarLimits, UnitId,
};
use filament_semantic_value::quantity::{
    compare_quantity, convert_quantity, evaluate_quantity_unit, result_unit, Conversion,
    QuantityOperation, QuantityTarget, QuantityUnit, UnitOperation, UnitScope, UnitTable,
};
use filament_semantic_value::unit::{
    DimensionEquivalenceCause as Cause, DimensionNode, NominalDeclaration, UnitGraph, UnitNode,
};
use ix_trace_rs::trace;
use std::collections::BTreeMap;
use std::num::NonZeroU64;

fn key(n: u64) -> NodeKey {
    common::node(n)
}
fn rat(n: i64, d: i64) -> Rational {
    Rational::new(Integer::from(n), Integer::from(d)).unwrap()
}
fn limits() -> ScalarLimits {
    ScalarLimits {
        integer_bits: 1024,
        decimal_digits: 1024,
        scale_expansion: 1024,
        text_input_bytes: 1024,
        text_scalars: 1024,
        normalized_scalars: 1024,
        unit_edges: 1024,
        value_occurrences: 1024,
        work_units: 1024,
        result_units: 1024,
    }
}
fn meter() -> Meter {
    Meter::new(limits())
}
fn unit(d: u64, target: Option<u64>, n: i64, denominator: i64, offset: i64) -> UnitNode {
    UnitNode::checked(
        key(d),
        target.map(key),
        rat(n, denominator),
        rat(offset, 1),
        NominalDeclaration::default(),
    )
    .unwrap()
}
fn graph() -> UnitGraph {
    UnitGraph::from_checked_nodes(
        &BTreeMap::from([
            (
                key(1),
                DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap(),
            ),
            (
                key(2),
                DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap(),
            ),
            (
                key(3),
                DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap(),
            ),
            (
                key(4),
                DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap(),
            ),
        ]),
        &BTreeMap::from([
            (key(11), unit(1, None, 1, 1, 0)),
            (key(12), unit(1, Some(11), 1, 1000, 0)),
            (key(21), unit(2, None, 1, 1, 0)),
            (key(22), unit(2, Some(21), 4, 1, 0)),
            (key(23), unit(2, Some(21), 1, 1, 2)),
            (key(31), unit(3, None, 1, 1, 0)),
            (key(41), unit(4, None, 1, 1, 0)),
        ]),
    )
    .unwrap()
}
fn rows() -> Vec<(NodeKey, NodeKey, Rational)> {
    vec![
        (key(1), key(1), rat(1, 1)),
        (key(2), key(1), rat(1, 4)),
        (key(3), key(1), rat(1, 2)),
    ]
}
fn projected() -> UnitGraph {
    graph().with_dimension_equivalences(&rows()).unwrap()
}
fn q(table: &UnitTable, n: i64, d: i64, id: u64) -> Quantity {
    let id = UnitId::declared(key(id));
    assert!(table.get(id.clone()).is_some());
    Quantity::new(rat(n, d), id)
}
fn declared(table: &UnitTable, id: u64) -> &QuantityUnit {
    table.get(UnitId::declared(key(id))).unwrap()
}
fn convert(
    table: &UnitTable,
    source: &Quantity,
    target: u64,
    meter: &mut Meter,
) -> Outcome<Conversion> {
    convert_quantity(
        table.resolve(source).unwrap(),
        declared(table, target),
        &QuantityTarget::Exact,
        meter,
    )
    .unwrap()
}
fn exact(outcome: Outcome<Conversion>) -> Conversion {
    let Outcome::Completed(value) = outcome else {
        panic!("conversion completes: {outcome:?}")
    };
    value
}

#[trace("TC-931", "FR-134-AC-1")]
#[test]
fn dimension_equivalence_conversion_validates_rows_in_order_without_mutating_the_graph() {
    let original = graph();
    let snapshot = original.clone();
    let cases = vec![
        (
            vec![(key(99), key(1), rat(0, 1))],
            key(99),
            Cause::UnknownDimension,
        ),
        (
            vec![(key(1), key(99), rat(1, 1))],
            key(99),
            Cause::UnknownDimension,
        ),
        (
            vec![(key(1), key(1), rat(1, 1)), (key(1), key(1), rat(0, 1))],
            key(1),
            Cause::DuplicateDimension,
        ),
        (
            vec![(key(2), key(1), rat(1, 1))],
            key(1),
            Cause::MissingRepresentative,
        ),
        (
            vec![(key(2), key(2), rat(1, 1)), (key(1), key(2), rat(1, 1))],
            key(2),
            Cause::NonCanonicalRepresentative,
        ),
        (
            vec![(key(1), key(1), rat(1, 1)), (key(2), key(1), rat(0, 1))],
            key(2),
            Cause::NonPositiveScale,
        ),
        (
            vec![(key(1), key(1), rat(1, 1)), (key(2), key(1), rat(-1, 1))],
            key(2),
            Cause::NonPositiveScale,
        ),
        (
            vec![(key(1), key(1), rat(2, 1))],
            key(1),
            Cause::NonIdentityRepresentative,
        ),
    ];
    for (rows, dimension, cause) in cases {
        let failed = original.with_dimension_equivalences(&rows).unwrap_err();
        assert_eq!((failed.dimension, failed.cause), (dimension, cause));
        assert_eq!(original, snapshot);
    }
    assert_eq!(original.with_dimension_equivalences(&[]).unwrap(), original);
    let empty = UnitGraph::from_checked_nodes(
        &BTreeMap::from([(
            key(1),
            DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap(),
        )]),
        &BTreeMap::new(),
    )
    .unwrap();
    let missing = empty
        .with_dimension_equivalences(&[(key(1), key(1), rat(1, 1))])
        .unwrap_err();
    assert_eq!(
        (missing.dimension, missing.cause),
        (key(1), Cause::MissingRoot)
    );
}

#[trace("TC-931", "FR-134-AC-2", "FR-134-AC-7")]
#[test]
fn dimension_equivalence_conversion_preserves_exact_scalars_and_nominal_preimages() {
    let original = graph();
    let projected = projected();
    let before = UnitTable::declared(&original);
    let table = UnitTable::declared(&projected);
    for (source, n, d, target, expected) in [
        (21, 1, 1, 11, rat(1, 4)),
        (11, 3, 1, 21, rat(12, 1)),
        (22, 1, 1, 12, rat(1000, 1)),
        (23, 1, 1, 11, rat(3, 4)),
    ] {
        let converted = exact(convert(
            &table,
            &q(&table, n, d, source),
            target,
            &mut meter(),
        ));
        let (quantity, identified) = converted.identified_quantity().unwrap();
        assert_eq!(quantity.magnitude(), &expected);
        assert_eq!(quantity.unit(), &UnitId::declared(key(target)));
        assert_eq!(quantity.unit(), &identified.id());
    }
    for id in [1, 2, 3, 4, 11, 12, 21, 22, 23, 31, 41] {
        assert_eq!(table.nominal_node(&key(id)), before.nominal_node(&key(id)));
    }
    for (unit, old) in projected.units().zip(original.units()) {
        assert_eq!(unit.id(), old.id());
        assert_eq!(unit.dimension_node(), old.dimension_node());
    }
    let same = original
        .with_dimension_equivalences(&[(key(1), key(1), rat(1, 1)), (key(2), key(1), rat(1, 1))])
        .unwrap();
    let table = UnitTable::declared(&same);
    assert_eq!(
        exact(convert(&table, &q(&table, 1, 1, 21), 11, &mut meter()))
            .identified_quantity()
            .unwrap()
            .0
            .magnitude(),
        &rat(1, 1)
    );
    assert_eq!(
        projected.with_dimension_equivalences(&rows()).unwrap(),
        projected
    );
    assert_eq!(
        projected.with_dimension_equivalences(&[]).unwrap(),
        projected
    );
}

#[trace("TC-931", "FR-134-AC-3", "FR-134-AC-6")]
#[test]
fn dimension_equivalence_conversion_charges_star_edges_and_each_arithmetic_event() {
    let table = UnitTable::declared(&projected());
    for (source, target, edges) in [(21, 11, 1), (21, 31, 2), (22, 12, 3), (12, 22, 3)] {
        let quantity = q(&table, 1, 1, source);
        let mut observed = meter();
        observed.enable_charge_capture(100, 200);
        assert!(matches!(
            convert(&table, &quantity, target, &mut observed),
            Outcome::Completed(_)
        ));
        let capture = observed.disable_charge_capture().unwrap().unwrap();
        let expected_points = std::iter::once(ChargePoint::UnitIdentityRead)
            .chain(std::iter::repeat_n(
                ChargePoint::UnitEdge,
                usize::try_from(edges).unwrap(),
            ))
            .chain(std::iter::repeat_n(
                ChargePoint::UnitRationalArithmetic,
                usize::try_from(2 * edges).unwrap(),
            ))
            .chain([ChargePoint::UnitTargetDomain, ChargePoint::UnitResultRetain]);
        assert_eq!(
            capture
                .events()
                .iter()
                .map(|event| event.point)
                .collect::<Vec<_>>(),
            expected_points.collect::<Vec<_>>()
        );
        for (point, count) in [
            (ChargePoint::UnitIdentityRead, 1),
            (ChargePoint::UnitEdge, edges),
            (ChargePoint::UnitRationalArithmetic, 2 * edges),
            (ChargePoint::UnitTargetDomain, 1),
            (ChargePoint::UnitResultRetain, 1),
        ] {
            for occurrence in 1..=count {
                let mut denied = meter().with_injected_denial(InjectedDenial {
                    point,
                    occurrence: NonZeroU64::new(occurrence).unwrap(),
                });
                denied.enable_charge_capture(100, 200);
                let Outcome::Incomplete(record) = convert(&table, &quantity, target, &mut denied)
                else {
                    panic!("denial {point:?} {occurrence} fires")
                };
                assert_eq!(record.charge_point, point);
                let denied_events = denied.disable_charge_capture().unwrap().unwrap();
                let index = capture
                    .events()
                    .iter()
                    .enumerate()
                    .filter(|(_, event)| event.point == point)
                    .nth(usize::try_from(occurrence - 1).unwrap())
                    .unwrap()
                    .0;
                assert_eq!(denied_events.events().len(), index + 1);
                assert_eq!(&denied_events.events()[..index], &capture.events()[..index]);
                assert_eq!(
                    denied_events.events()[index].admission,
                    ChargeAdmission::Denied
                );
                assert_prefix_counters(&denied, &capture.events()[..index]);
            }
            let mut absent = meter().with_injected_denial(InjectedDenial {
                point,
                occurrence: NonZeroU64::new(count + 1).unwrap(),
            });
            assert!(matches!(
                convert(&table, &quantity, target, &mut absent),
                Outcome::Completed(_)
            ));
        }
        for bound in 0..edges {
            let mut limits = limits();
            limits.unit_edges = bound;
            assert!(matches!(
                convert(&table, &quantity, target, &mut Meter::new(limits)),
                Outcome::Incomplete(_)
            ));
        }
    }
}

fn assert_prefix_counters(meter: &Meter, admitted: &[ChargeEvent]) {
    assert_eq!(
        meter.admission_count(),
        u64::try_from(admitted.len()).unwrap()
    );
    for kind in LimitKind::ALL {
        let expected = match kind {
            LimitKind::WorkUnits => admitted
                .iter()
                .map(|event| event.work_units.to_u64().unwrap())
                .sum(),
            LimitKind::ResultUnits => admitted
                .iter()
                .map(|event| event.result_units.to_u64().unwrap())
                .sum(),
            _ => admitted
                .iter()
                .flat_map(|event| &event.sizes)
                .filter(|(found, _)| *found == kind)
                .map(|(_, value)| value.to_u64().unwrap())
                .max()
                .unwrap_or(0),
        };
        assert_eq!(meter.consumed(kind), expected, "atomic {kind:?}");
    }
}

#[trace("TC-931", "FR-134-AC-3")]
#[test]
fn dimension_equivalence_conversion_forward_and_reverse_preserve_arithmetic_order() {
    let table = UnitTable::declared(&projected());
    let one = rat(1, 1);
    let quarter = rat(1, 4);
    let zero = rat(0, 1);
    let forward = [
        rational_arithmetic_bits(RationalArithmetic::Multiply(&one, &quarter)),
        rational_arithmetic_bits(RationalArithmetic::Add(&quarter, &zero)),
    ];
    let reverse = [
        rational_arithmetic_bits(RationalArithmetic::Subtract(&one, &zero)),
        rational_arithmetic_bits(RationalArithmetic::Divide(&one, &quarter)),
    ];
    for (source, target, sizes) in [(21, 11, forward), (11, 21, reverse)] {
        let mut meter = meter();
        meter.enable_charge_capture(16, 32);
        assert!(matches!(
            convert(&table, &q(&table, 1, 1, source), target, &mut meter),
            Outcome::Completed(_)
        ));
        let capture = meter.disable_charge_capture().unwrap().unwrap();
        let actual = capture
            .events()
            .iter()
            .filter(|event| event.point == ChargePoint::UnitRationalArithmetic)
            .map(|event| event.sizes.clone())
            .collect::<Vec<_>>();
        assert_eq!(
            actual,
            sizes.map(|size| vec![(LimitKind::IntegerBits, size)])
        );
    }
}

#[trace("TC-931", "FR-134-AC-3")]
#[test]
fn dimension_equivalence_conversion_scale_one_still_traverses_identification_events() {
    let graph = graph()
        .with_dimension_equivalences(&[(key(1), key(1), rat(1, 1)), (key(2), key(1), rat(1, 1))])
        .unwrap();
    let table = UnitTable::declared(&graph);
    let source = q(&table, 5, 1, 21);
    let mut meter = meter();
    let converted = exact(convert(&table, &source, 11, &mut meter));
    assert_eq!(
        converted.identified_quantity().unwrap().0.magnitude(),
        &rat(5, 1)
    );
    assert_eq!(meter.consumed(LimitKind::UnitEdges), 1);
    assert_eq!(meter.admission_count(), 6);
}

#[trace("TC-931", "FR-134-AC-4", "FR-134-AC-7")]
#[test]
fn dimension_equivalence_conversion_compound_products_and_quotients_use_class_roots() {
    let graph = projected();
    let table = UnitTable::declared(&graph);
    let seconds = q(&table, 6, 1, 11);
    let beats = q(&table, 3, 1, 21);
    let equivalent = q(&table, 3, 4, 11);
    let run = |right: &Quantity| {
        evaluate_quantity_unit(
            QuantityOperation::Divide(
                table.resolve(&seconds).unwrap(),
                table.resolve(right).unwrap(),
            ),
            &mut meter(),
        )
        .unwrap()
    };
    let (Outcome::Completed(actual), unit) = run(&beats) else {
        panic!("quotient completes")
    };
    let (Outcome::Completed(expected), other) = run(&equivalent) else {
        panic!("quotient completes")
    };
    assert_eq!(actual, expected);
    assert_eq!(unit, other);
    assert_eq!(actual.magnitude(), &rat(8, 1));
    assert_eq!(
        unit.unit(),
        &QuantityUnit::Compound(graph.compound_unit(&[]).unwrap())
    );
    let second = declared(&table, 11);
    let beat = declared(&table, 21);
    assert_eq!(
        result_unit(UnitOperation::Multiply, second, beat).unwrap(),
        result_unit(UnitOperation::Multiply, second, second).unwrap()
    );
    let a = q(&table, 2, 1, 11);
    let b = q(&table, 8, 1, 21);
    let power = Integer::from(2_i64);
    assert_eq!(
        evaluate_quantity_unit(
            QuantityOperation::Power(table.resolve(&a).unwrap(), &power),
            &mut meter()
        )
        .unwrap(),
        evaluate_quantity_unit(
            QuantityOperation::Power(table.resolve(&b).unwrap(), &power),
            &mut meter()
        )
        .unwrap()
    );
    let compound =
        QuantityUnit::Compound(graph.compound_unit(&[(key(11), Integer::one())]).unwrap());
    let source = q(&table, 1, 1, 21);
    let Outcome::Completed(converted) = convert_quantity(
        table.resolve(&source).unwrap(),
        &compound,
        &QuantityTarget::Exact,
        &mut meter(),
    )
    .unwrap() else {
        panic!("compound conversion completes")
    };
    let (formed, identified) = converted.identified_quantity().unwrap();
    let mut scope = UnitScope::new(&table);
    scope.form(identified);
    let Outcome::Completed(back) = convert_quantity(
        scope.resolve(&formed).unwrap(),
        second,
        &QuantityTarget::Exact,
        &mut meter(),
    )
    .unwrap() else {
        panic!("declared conversion completes")
    };
    assert_eq!(
        back.identified_quantity().unwrap().0.magnitude(),
        &rat(1, 4)
    );
    assert_ne!(compound.id().unwrap(), second.id().unwrap());
}

#[trace("TC-931", "FR-134-AC-5")]
#[test]
fn dimension_equivalence_conversion_preserves_uncharged_typing_refusals() {
    let table = UnitTable::declared(&projected());
    let a = q(&table, 1, 1, 11);
    let b = q(&table, 1, 1, 21);
    let affine = q(&table, 1, 1, 23);
    let unrelated = q(&table, 1, 1, 41);
    for operation in [
        QuantityOperation::Add(table.resolve(&a).unwrap(), table.resolve(&b).unwrap()),
        QuantityOperation::Subtract(table.resolve(&a).unwrap(), table.resolve(&b).unwrap()),
    ] {
        let mut meter = meter();
        let before = meter.clone();
        assert_eq!(
            evaluate_quantity_unit(operation, &mut meter).unwrap_err(),
            filament_semantic_value::quantity::QuantityRefusal::IllTyped(
                filament_exact::IllTyped {
                    cause: IllTypedCause::DistinctUnits
                }
            )
        );
        assert_eq!(meter, before);
    }
    for operator in ComparisonOperator::ALL {
        let mut meter = meter();
        let before = meter.clone();
        assert_eq!(
            compare_quantity(
                operator,
                table.resolve(&a).unwrap(),
                table.resolve(&b).unwrap(),
                &mut meter
            )
            .unwrap_err()
            .cause,
            IllTypedCause::DistinctUnits
        );
        assert_eq!(meter, before);
    }
    let mut meter = meter();
    let before = meter.clone();
    assert_eq!(
        convert_quantity(
            table.resolve(&unrelated).unwrap(),
            declared(&table, 11),
            &QuantityTarget::Exact,
            &mut meter
        )
        .unwrap_err()
        .cause,
        IllTypedCause::IncompatibleDimensions
    );
    assert_eq!(meter, before);
    let mut meter = Meter::new(limits());
    let before = meter.clone();
    assert_eq!(
        evaluate_quantity_unit(
            QuantityOperation::Multiply(
                table.resolve(&affine).unwrap(),
                table.resolve(&a).unwrap()
            ),
            &mut meter
        )
        .unwrap_err(),
        filament_semantic_value::quantity::QuantityRefusal::IllTyped(filament_exact::IllTyped {
            cause: IllTypedCause::AffineUnitArithmetic
        })
    );
    assert_eq!(meter, before);
}

fn mixed_graph(base_first: bool, exponent: i64) -> (UnitGraph, u64, u64) {
    let (base, derived) = if base_first { (1, 2) } else { (2, 1) };
    (
        UnitGraph::from_checked_nodes(
            &BTreeMap::from([
                (
                    key(base),
                    DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap(),
                ),
                (
                    key(derived),
                    DimensionNode::checked_derived(
                        vec![(key(base), Integer::from(exponent))],
                        NominalDeclaration::default(),
                    )
                    .unwrap(),
                ),
            ]),
            &BTreeMap::from([
                (key(11), unit(base, None, 1, 1, 0)),
                (key(21), unit(derived, None, 1, 1, 0)),
            ]),
        )
        .unwrap(),
        base,
        derived,
    )
}

#[trace("TC-931", "FR-134-AC-1", "FR-134-AC-7")]
#[test]
fn dimension_equivalence_conversion_mixed_classes_preserve_base_maps_and_nominal_admission() {
    for base_first in [true, false] {
        let (graph, base, derived) = mixed_graph(base_first, 1);
        let rows = vec![(key(1), key(1), rat(1, 1)), (key(2), key(1), rat(1, 1))];
        let before = UnitTable::declared(&graph);
        assert!(!declared(&before, 11).converts_to(declared(&before, 21)));
        let projected = graph.with_dimension_equivalences(&rows).unwrap();
        let table = UnitTable::declared(&projected);
        assert_eq!(
            exact(convert(&table, &q(&table, 3, 1, 11), 21, &mut meter()))
                .identified_quantity()
                .unwrap()
                .0
                .magnitude(),
            &rat(3, 1)
        );
        assert_eq!(
            projected
                .dimension(key(derived))
                .unwrap()
                .exponents()
                .collect::<Vec<_>>(),
            vec![(key(base), &Integer::one())]
        );
    }
    let (graph, _, _) = mixed_graph(true, 2);
    let before = graph.clone();
    let failure = graph
        .with_dimension_equivalences(&[(key(1), key(1), rat(1, 1)), (key(2), key(1), rat(1, 1))])
        .unwrap_err();
    assert_eq!(failure.cause, Cause::IncompatibleClass);
    assert_eq!(graph, before);
}

#[trace("TC-931", "FR-134-AC-6")]
#[test]
fn dimension_equivalence_conversion_empty_overlay_preserves_schedule_and_tight_limits_stop() {
    let graph = graph();
    let original = UnitTable::declared(&graph);
    let empty = UnitTable::declared(&graph.with_dimension_equivalences(&[]).unwrap());
    let quantity = q(&original, 2, 1, 12);
    let mut before = meter();
    let mut after = meter();
    assert_eq!(
        convert(&original, &quantity, 11, &mut before),
        convert(&empty, &quantity, 11, &mut after)
    );
    assert_eq!(before, after);
    let table = UnitTable::declared(&projected());
    let quantity = q(&table, 3, 1, 22);
    let mut observed = meter();
    observed.enable_charge_capture(64, 128);
    let expected = exact(convert(&table, &quantity, 12, &mut observed));
    let capture = observed.disable_charge_capture().unwrap().unwrap();
    for kind in [
        LimitKind::WorkUnits,
        LimitKind::UnitEdges,
        LimitKind::IntegerBits,
        LimitKind::ValueOccurrences,
        LimitKind::ResultUnits,
    ] {
        let sufficient = observed.consumed(kind);
        assert!(sufficient > 0);
        let mut exact_meter = Meter::new(limit_only(kind, sufficient));
        assert_eq!(
            convert(&table, &quantity, 12, &mut exact_meter),
            Outcome::Completed(expected.clone())
        );
        assert_prefix_counters(&exact_meter, capture.events());
        let bound = sufficient - 1;
        let mut denied = Meter::new(limit_only(kind, bound));
        denied.enable_charge_capture(64, 128);
        let actual = convert(&table, &quantity, 12, &mut denied);
        let index = capture
            .events()
            .iter()
            .enumerate()
            .position(|(index, event)| {
                let before = prefix_consumed(&capture.events()[..index], kind);
                let next = event_request(event, kind).to_u64().unwrap();
                match kind {
                    LimitKind::WorkUnits | LimitKind::ResultUnits => before + next > bound,
                    _ => next > bound,
                }
            })
            .unwrap();
        let event = &capture.events()[index];
        assert_eq!(
            actual,
            Outcome::Incomplete(Incomplete {
                limit_kind: kind,
                limit: bound,
                consumed: prefix_consumed(&capture.events()[..index], kind),
                next_charge: event_request(event, kind),
                charge_point: event.point
            })
        );
        assert_prefix_counters(&denied, &capture.events()[..index]);
        let actual_events = denied.disable_charge_capture().unwrap().unwrap();
        assert_eq!(&actual_events.events()[..index], &capture.events()[..index]);
        assert_eq!(actual_events.events().len(), index + 1);
        let mut expected_denied = event.clone();
        expected_denied.admission = ChargeAdmission::Denied;
        assert_eq!(actual_events.events()[index], expected_denied);
    }
}

fn limit_only(kind: LimitKind, bound: u64) -> ScalarLimits {
    match kind {
        LimitKind::WorkUnits => ScalarLimits {
            work_units: bound,
            ..limits()
        },
        LimitKind::UnitEdges => ScalarLimits {
            unit_edges: bound,
            ..limits()
        },
        LimitKind::IntegerBits => ScalarLimits {
            integer_bits: bound,
            ..limits()
        },
        LimitKind::ValueOccurrences => ScalarLimits {
            value_occurrences: bound,
            ..limits()
        },
        LimitKind::ResultUnits => ScalarLimits {
            result_units: bound,
            ..limits()
        },
        _ => panic!("fixture measures five quantity limits"),
    }
}

fn event_request(event: &ChargeEvent, kind: LimitKind) -> Integer {
    match kind {
        LimitKind::WorkUnits => event.work_units.clone(),
        LimitKind::ResultUnits => event.result_units.clone(),
        _ => event
            .sizes
            .iter()
            .filter(|(found, _)| *found == kind)
            .map(|(_, amount)| amount)
            .max()
            .cloned()
            .unwrap_or_else(Integer::zero),
    }
}

fn prefix_consumed(events: &[ChargeEvent], kind: LimitKind) -> u64 {
    let amounts = events
        .iter()
        .map(|event| event_request(event, kind).to_u64().unwrap());
    match kind {
        LimitKind::WorkUnits | LimitKind::ResultUnits => amounts.sum(),
        _ => amounts.max().unwrap_or(0),
    }
}

#[trace("TC-931", "FR-134-AC-7")]
#[test]
fn dimension_equivalence_conversion_equal_map_derived_nodes_require_explicit_membership() {
    let base = DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap();
    let derived = DimensionNode::checked_derived(
        vec![(key(1), Integer::from(2_i64))],
        NominalDeclaration::default(),
    )
    .unwrap();
    let graph = UnitGraph::from_checked_nodes(
        &BTreeMap::from([(key(1), base), (key(2), derived.clone()), (key(3), derived)]),
        &BTreeMap::from([
            (key(21), unit(2, None, 1, 1, 0)),
            (key(31), unit(3, None, 1, 1, 0)),
        ]),
    )
    .unwrap();
    let table = UnitTable::declared(&graph);
    assert_eq!(
        declared(&table, 21).dimension(),
        declared(&table, 31).dimension()
    );
    let quantity = q(&table, 7, 1, 31);
    let mut untouched = meter();
    let snapshot = untouched.clone();
    assert_eq!(
        convert_quantity(
            table.resolve(&quantity).unwrap(),
            declared(&table, 21),
            &QuantityTarget::Exact,
            &mut untouched
        )
        .unwrap_err()
        .cause,
        IllTypedCause::IncompatibleDimensions
    );
    assert_eq!(untouched, snapshot);
    let graph = graph
        .with_dimension_equivalences(&[(key(2), key(2), rat(1, 1)), (key(3), key(2), rat(1, 1))])
        .unwrap();
    let table = UnitTable::declared(&graph);
    let mut charged = meter();
    let (converted, identified) = exact(convert(&table, &quantity, 21, &mut charged))
        .identified_quantity()
        .unwrap();
    assert_eq!(converted.magnitude(), &rat(7, 1));
    assert_eq!(converted.unit(), &UnitId::declared(key(21)));
    assert_eq!(converted.unit(), &identified.id());
    assert_eq!(charged.consumed(LimitKind::UnitEdges), 1);
}

fn derived(terms: Vec<(u64, i64)>) -> DimensionNode {
    DimensionNode::checked_derived(
        terms
            .into_iter()
            .map(|(base, exponent)| (key(base), Integer::from(exponent)))
            .collect(),
        NominalDeclaration::default(),
    )
    .unwrap()
}
fn base_dimension() -> DimensionNode {
    DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap()
}
fn graph_of(dimensions: Vec<(u64, DimensionNode)>, units: Vec<u64>) -> UnitGraph {
    UnitGraph::from_checked_nodes(
        &dimensions.into_iter().map(|(k, d)| (key(k), d)).collect(),
        &units
            .into_iter()
            .map(|d| (key(d * 10 + 1), unit(d, None, 1, 1, 0)))
            .collect(),
    )
    .unwrap()
}

/// Beat 1, Time 2 (1 s = 4 beat), Length 7, Foot 8; unlisted derived Time^2
/// (3), Time^-1 (4), Time * Foot^-1 (5), Beat^2 (9), Beat^-1 (10) and
/// Beat * Length^-1 (12).
fn derived_graph() -> UnitGraph {
    graph_of(
        vec![
            (1, base_dimension()),
            (2, base_dimension()),
            (3, derived(vec![(2, 2)])),
            (4, derived(vec![(2, -1)])),
            (5, derived(vec![(2, 1), (8, -1)])),
            (7, base_dimension()),
            (8, base_dimension()),
            (9, derived(vec![(1, 2)])),
            (10, derived(vec![(1, -1)])),
            (12, derived(vec![(1, 1), (7, -1)])),
        ],
        vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 12],
    )
}
fn base_rows() -> Vec<(NodeKey, NodeKey, Rational)> {
    vec![
        (key(1), key(1), rat(1, 1)),
        (key(2), key(1), rat(4, 1)),
        (key(7), key(7), rat(1, 1)),
        (key(8), key(7), rat(3, 2)),
    ]
}
fn coherence(graph: &UnitGraph, unit: u64) -> Rational {
    graph.unit(key(unit)).unwrap().coherence().clone()
}
fn canonical_scale(graph: &UnitGraph, unit: u64) -> Rational {
    graph.unit(key(unit)).unwrap().canonical().scale().clone()
}

/// Convert 1 of declared unit `from` to the compound of `terms`, then to the
/// declared unit `to`; returns the magnitudes of both steps and the edges
/// charged.
fn pivot(graph: &UnitGraph, from: u64, terms: &[(u64, i64)], to: u64) -> (Rational, Rational, u64) {
    let table = UnitTable::declared(graph);
    let terms: Vec<_> = terms
        .iter()
        .map(|(unit, exponent)| (key(*unit), Integer::from(*exponent)))
        .collect();
    let compound = QuantityUnit::Compound(graph.compound_unit(&terms).unwrap());
    let source = q(&table, 1, 1, from);
    let mut charged = meter();
    let Outcome::Completed(first) = convert_quantity(
        table.resolve(&source).unwrap(),
        &compound,
        &QuantityTarget::Exact,
        &mut charged,
    )
    .unwrap() else {
        panic!("compound conversion completes")
    };
    let (formed, identified) = first.identified_quantity().unwrap();
    let mut scope = UnitScope::new(&table);
    scope.form(identified);
    let Outcome::Completed(second) = convert_quantity(
        scope.resolve(&formed).unwrap(),
        declared(&table, to),
        &QuantityTarget::Exact,
        &mut charged,
    )
    .unwrap() else {
        panic!("declared conversion completes")
    };
    (
        formed.magnitude().clone(),
        second.identified_quantity().unwrap().0.magnitude().clone(),
        charged.consumed(LimitKind::UnitEdges),
    )
}

#[trace("TC-931", "FR-134-AC-7")]
#[test]
fn dimension_equivalence_derived_root_factor_is_the_product_of_base_class_scales() {
    let projected = derived_graph()
        .with_dimension_equivalences(&base_rows())
        .unwrap();
    // 1 s = 4 beat, so 1 s2 = 16 beat2 though Time^2 has no membership row.
    assert_eq!(coherence(&projected, 31), rat(16, 1));
    assert_eq!(coherence(&projected, 41), rat(1, 4));
    // Two classes, rational class scale: 4 * (3/2)^-1.
    assert_eq!(coherence(&projected, 51), rat(8, 3));
    for unit in [11, 21, 71, 81, 91, 101, 121] {
        assert_eq!(coherence(&projected, unit), rat(1, 1), "unit {unit}");
    }
    // The factor is not a path edge: derived unit paths are as before.
    let nominal = derived_graph();
    for unit in [31, 41, 51, 91] {
        assert_eq!(
            projected.unit(key(unit)).unwrap().path(),
            nominal.unit(key(unit)).unwrap().path()
        );
    }
    assert_eq!(coherence(&nominal, 31), rat(1, 1));
}

#[trace("TC-931", "FR-134-AC-7")]
#[test]
fn dimension_equivalence_derived_root_factor_applies_at_the_compound_pivot() {
    let graph = derived_graph()
        .with_dimension_equivalences(&base_rows())
        .unwrap();
    // s2 -> beat2 is 16; per-second -> per-beat is 1/4; s / foot -> beat /
    // length is 8/3; no step charges an edge.
    assert_eq!(
        pivot(&graph, 31, &[(11, 2)], 91),
        (rat(16, 1), rat(16, 1), 0)
    );
    assert_eq!(
        pivot(&graph, 41, &[(11, -1)], 101),
        (rat(1, 4), rat(1, 4), 0)
    );
    assert_eq!(
        pivot(&graph, 51, &[(11, 1), (71, -1)], 121),
        (rat(8, 3), rat(8, 3), 0)
    );
    // The way back divides by the same factor.
    assert_eq!(
        pivot(&graph, 91, &[(11, 2)], 31),
        (rat(1, 1), rat(1, 16), 0)
    );
    // A product of base-rooted operands reaches the derived root: 1 s * 1 s
    // is exactly 1 s2.
    let table = UnitTable::declared(&graph);
    let seconds = q(&table, 1, 1, 21);
    let (Outcome::Completed(product), unit) = evaluate_quantity_unit(
        QuantityOperation::Multiply(
            table.resolve(&seconds).unwrap(),
            table.resolve(&seconds).unwrap(),
        ),
        &mut meter(),
    )
    .unwrap() else {
        panic!("product completes")
    };
    assert_eq!(product.magnitude(), &rat(16, 1));
    let mut scope = UnitScope::new(&table);
    scope.form(unit);
    let Outcome::Completed(back) = convert_quantity(
        scope.resolve(&product).unwrap(),
        declared(&table, 31),
        &QuantityTarget::Exact,
        &mut meter(),
    )
    .unwrap() else {
        panic!("conversion completes")
    };
    assert_eq!(
        back.identified_quantity().unwrap().0.magnitude(),
        &rat(1, 1)
    );
    // A squared operand and a derived operand multiply by the factor too.
    let squared = Integer::from(2_i64);
    let (Outcome::Completed(power), _) = evaluate_quantity_unit(
        QuantityOperation::Power(table.resolve(&seconds).unwrap(), &squared),
        &mut meter(),
    )
    .unwrap() else {
        panic!("power completes")
    };
    assert_eq!(power.magnitude(), &rat(16, 1));
    let square = q(&table, 1, 1, 31);
    let (Outcome::Completed(times), _) = evaluate_quantity_unit(
        QuantityOperation::Multiply(
            table.resolve(&square).unwrap(),
            table.resolve(&q(&table, 1, 1, 91)).unwrap(),
        ),
        &mut meter(),
    )
    .unwrap() else {
        panic!("product completes")
    };
    assert_eq!(times.magnitude(), &rat(16, 1));
}

#[trace("TC-931", "FR-134-AC-3")]
#[test]
fn dimension_equivalence_derived_conversions_charge_no_coherence_edge() {
    // Two units of one unlisted derived dimension: path edges as before.
    let base = |c: u64| {
        UnitNode::checked(
            key(3),
            Some(key(31)),
            rat(c as i64, 1),
            rat(0, 1),
            NominalDeclaration::default(),
        )
        .unwrap()
    };
    let graph = UnitGraph::from_checked_nodes(
        &BTreeMap::from([
            (key(1), base_dimension()),
            (key(2), base_dimension()),
            (key(3), derived(vec![(2, 2)])),
        ]),
        &BTreeMap::from([
            (key(11), unit(1, None, 1, 1, 0)),
            (key(21), unit(2, None, 1, 1, 0)),
            (key(31), unit(3, None, 1, 1, 0)),
            (key(32), base(2)),
            (key(33), base(3)),
        ]),
    )
    .unwrap();
    let projected = graph
        .with_dimension_equivalences(&[(key(1), key(1), rat(1, 1)), (key(2), key(1), rat(4, 1))])
        .unwrap();
    assert_eq!(coherence(&projected, 31), rat(16, 1));
    let table = UnitTable::declared(&projected);
    let mut charged = meter();
    let converted = exact(convert(&table, &q(&table, 1, 1, 32), 33, &mut charged));
    assert_eq!(converted.canonical(), &rat(2, 1));
    assert_eq!(charged.consumed(LimitKind::UnitEdges), 2);
}

#[trace("TC-931", "FR-134-AC-7")]
#[test]
fn dimension_equivalence_listed_derived_root_must_agree_with_its_base_roots() {
    // Beat^2 (3) and Time^2 (4) listed as one class: no base member, so the
    // member's scale is its coherent scale over the representative's.
    let graph = graph_of(
        vec![
            (1, base_dimension()),
            (2, base_dimension()),
            (3, derived(vec![(1, 2)])),
            (4, derived(vec![(2, 2)])),
        ],
        vec![1, 2, 3, 4],
    );
    let rows = |scale: i64| {
        vec![
            (key(1), key(1), rat(1, 1)),
            (key(2), key(1), rat(4, 1)),
            (key(3), key(3), rat(1, 1)),
            (key(4), key(3), rat(scale, 1)),
        ]
    };
    let admitted = graph.with_dimension_equivalences(&rows(16)).unwrap();
    assert_eq!(canonical_scale(&admitted, 41), rat(16, 1));
    assert_eq!(coherence(&admitted, 41), rat(1, 1));
    let refusal = graph.with_dimension_equivalences(&rows(4)).unwrap_err();
    assert_eq!(refusal.dimension, key(4));
    assert_eq!(refusal.cause, Cause::IncompatibleClass);
    // A malformed base row is reported before any coherence comparison.
    let mut malformed = rows(4);
    malformed[1].2 = rat(0, 1);
    assert_eq!(
        graph
            .with_dimension_equivalences(&malformed)
            .unwrap_err()
            .cause,
        Cause::NonPositiveScale
    );
}

#[trace("TC-931", "FR-134-AC-7")]
#[test]
fn dimension_equivalence_derived_representative_must_agree_with_its_base_members() {
    // Dur = R::Time, F::Time and R::Time in one class, 1 R::s = 1000 F::ms,
    // in both key orders of the derived member against the base members.
    for (d, f, r) in [(1_u64, 2_u64, 3_u64), (2, 1, 3)] {
        let graph = graph_of(
            vec![
                (d, derived(vec![(r, 1)])),
                (f, base_dimension()),
                (r, base_dimension()),
            ],
            vec![d, f, r],
        );
        let rep = d.min(f);
        let rows = |dur: i64, ms: (i64, i64), sec: i64| {
            let mut rows = vec![(key(r), key(rep), rat(sec, 1))];
            rows.push((key(rep), key(rep), rat(1, 1)));
            for (dimension, scale) in [(d, rat(dur, 1)), (f, rat(ms.0, ms.1))] {
                if dimension != rep {
                    rows.push((key(dimension), key(rep), scale));
                }
            }
            rows
        };
        // Coherent: 1 dsec = 1 R::s = 1000 F::ms.
        let coherent = if rep == d {
            rows(1, (1, 1000), 1)
        } else {
            rows(1000, (1, 1), 1000)
        };
        let projected = graph.with_dimension_equivalences(&coherent).unwrap();
        let scale = |dimension: u64| canonical_scale(&projected, dimension * 10 + 1);
        assert_eq!(scale(d).div(&scale(r)).unwrap(), rat(1, 1));
        assert_eq!(scale(f).div(&scale(d)).unwrap(), rat(1, 1000));
        // Incoherent: 1 dsec = 1 F::ms while 1 R::s = 1000 F::ms.
        let incoherent = rows(1, (1, 1), 1000);
        let refusal = graph.with_dimension_equivalences(&incoherent).unwrap_err();
        assert_eq!(refusal.dimension, key(d), "order {d}{f}{r}");
        assert_eq!(refusal.cause, Cause::IncompatibleClass);
    }
}

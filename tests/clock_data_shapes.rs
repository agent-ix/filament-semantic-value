// SPDX-License-Identifier: AGPL-3.0-or-later
//! FR-133 data parts after checked-package lowering removes Clock members.
//! The caller retains clock places and maps source tuple ordinals to data indices.
//! Round trips here use value accessors and checked constructors; FSL owns wire conversion.

mod common;

use filament_exact::{
    FieldValue, Integer, Meter, NodeKey, Outcome, Presence, ScalarLimits, UnionMember, Value,
    ValueType,
};
use filament_semantic_value::declaration::{
    Component, CompositeDeclaration, CompositeShape, ConstructionCause, EqualityOperand,
    EqualityOperator, FieldDeclaration, TypeEnvironment, UnionMemberDeclaration,
};
use filament_semantic_value::enumeration::EnumMemberIndex;
use ix_trace_rs::trace;

fn integer(n: i64) -> Value {
    Value::Integer(Integer::from(n))
}

fn environment() -> TypeEnvironment {
    let phase = common::node(4);
    let member = |name| {
        UnionMemberDeclaration::from_verified(
            UnionMember::from_admitted(phase.clone(), name).unwrap(),
            vec![],
        )
    };
    TypeEnvironment::new(
        [
            CompositeDeclaration::new(
                common::node(1),
                "Slot",
                CompositeShape::Tuple(vec![ValueType::Integer]),
            ),
            CompositeDeclaration::new(
                common::node(2),
                "Trio",
                CompositeShape::Tuple(vec![ValueType::Integer, ValueType::Boolean]),
            ),
            CompositeDeclaration::new(common::node(3), "Ticks", CompositeShape::Tuple(vec![])),
            CompositeDeclaration::new(
                phase.clone(),
                "Phase",
                CompositeShape::Union(vec![member("Idle"), member("Armed")]),
            ),
            CompositeDeclaration::new(
                common::node(5),
                "Timer",
                CompositeShape::Record(vec![FieldDeclaration::new(
                    "n",
                    ValueType::Integer,
                    Presence::Required,
                )]),
            ),
        ],
        [],
    )
    .unwrap()
}

fn assert_equal(
    environment: &TypeEnvironment,
    key: NodeKey,
    left: &Value,
    right: &Value,
    expected: bool,
) {
    let operand = EqualityOperand::typed(ValueType::Composite(key));
    let equality = environment
        .check_equality(
            EqualityOperator::Equal,
            operand.clone(),
            operand,
            &EnumMemberIndex::default(),
        )
        .unwrap();
    let mut meter = Meter::new(ScalarLimits {
        integer_bits: 1_000,
        decimal_digits: 1_000,
        scale_expansion: 1_000,
        text_input_bytes: 1_000,
        text_scalars: 1_000,
        normalized_scalars: 1_000,
        unit_edges: 1_000,
        value_occurrences: 1_000,
        work_units: 1_000,
        result_units: 1_000,
    });
    assert_eq!(
        equality.evaluate(environment, left, right, &mut meter),
        Outcome::Completed(expected)
    );
}

/// The Clock-payload case stays a distinct, empty-payload union member.
/// Trace: FR-133-AC-1, FR-133-AC-2
#[trace("TC-930", "FR-133-AC-1", "FR-133-AC-2")]
#[test]
fn clock_union_data_shape_round_trips_without_a_payload_placeholder() {
    let environment = environment();
    let key = common::node(4);
    let mut values = Vec::new();
    for (expected_position, name) in ["Armed", "Idle"].into_iter().enumerate() {
        let (position, member) = environment.union_member_named(key.clone(), name).unwrap();
        assert_eq!(position, expected_position);
        assert!(member.positions().is_empty());
        let binding = member.member().unwrap();
        let value = environment
            .union(key.clone(), binding.variant().clone(), vec![])
            .unwrap();
        let Value::Union(carrier) = &value else {
            panic!("a Clock case retains the union carrier");
        };
        assert_eq!(carrier.declaration(), &key);
        assert_eq!(carrier.member(), binding);
        assert_eq!(carrier.member().identifier(), name);
        assert!(carrier.payload().is_empty());
        assert_eq!(value.occ(), Integer::one());
        let rebuilt = environment
            .union(
                carrier.declaration().clone(),
                carrier.variant().clone(),
                carrier.payload().to_vec(),
            )
            .unwrap();
        assert!(environment
            .admits(&ValueType::Composite(key.clone()), &rebuilt)
            .unwrap());
        assert_equal(&environment, key.clone(), &value, &rebuilt, true);
        values.push(value);
    }
    let [armed, idle] = values.as_slice() else {
        panic!("both union cases must be constructed");
    };
    assert_equal(&environment, key.clone(), armed, idle, false);
    let (Value::Union(armed), Value::Union(idle)) = (armed, idle) else {
        panic!("both cases retain union carriers");
    };
    assert_ne!(armed.member(), idle.member());
    let refusal = environment
        .union(key, armed.variant().clone(), vec![Value::Boolean(false)])
        .unwrap_err();
    assert_eq!(refusal.component, Component::Value);
    assert_eq!(
        refusal.cause,
        ConstructionCause::WrongArity {
            declared: 0,
            supplied: 1
        }
    );
}

/// Clock ordinals have no slot; data positions retain their relative order.
/// Trace: FR-133-AC-1, FR-133-AC-2
#[trace("TC-930", "FR-133-AC-1", "FR-133-AC-2")]
#[test]
fn clock_tuple_data_shape_round_trips_only_its_ordered_data_slots() {
    let environment = environment();
    // Source Slot=(Clock,Integer), Trio=(Integer,Clock,Boolean), Ticks=(Clock).
    // Lowering supplies only data positions, in source-ordinal order.
    for (n, positions) in [
        (1, vec![integer(7)]),
        (2, vec![integer(4), Value::Boolean(true)]),
        (3, vec![]),
    ] {
        let key = common::node(n);
        let value = environment.tuple(key.clone(), positions.clone()).unwrap();
        let Value::Composite(carrier) = &value else {
            panic!("a data tuple retains the composite carrier");
        };
        assert_eq!(carrier.declaration(), &key);
        assert_eq!(carrier.slots().len(), positions.len());
        for (slot, expected) in carrier.slots().iter().zip(&positions) {
            match (slot, expected) {
                (FieldValue::Present(Value::Integer(actual)), Value::Integer(expected)) => {
                    assert_eq!(actual, expected)
                }
                (FieldValue::Present(Value::Boolean(actual)), Value::Boolean(expected)) => {
                    assert_eq!(actual, expected)
                }
                _ => panic!("data positions retain their kinds and supplied values"),
            }
        }
        let extracted = carrier
            .slots()
            .iter()
            .map(|slot| match slot {
                FieldValue::Present(value) => value.clone(),
                _ => panic!("tuple data positions are present values"),
            })
            .collect();
        let rebuilt = environment
            .tuple(carrier.declaration().clone(), extracted)
            .unwrap();
        assert!(environment
            .admits(&ValueType::Composite(key.clone()), &rebuilt)
            .unwrap());
        assert_equal(&environment, key, &value, &rebuilt, true);
    }
    for (key, supplied, declared) in [
        (common::node(1), vec![Value::Boolean(false), integer(7)], 1),
        (
            common::node(2),
            vec![integer(4), Value::Boolean(false), Value::Boolean(true)],
            2,
        ),
    ] {
        let supplied_len = supplied.len();
        let refusal = environment.tuple(key, supplied).unwrap_err();
        assert_eq!(refusal.component, Component::Value);
        assert_eq!(
            refusal.cause,
            ConstructionCause::WrongArity {
                declared,
                supplied: supplied_len
            }
        );
    }
    let refusal = environment
        .tuple(common::node(2), vec![Value::Boolean(true), integer(4)])
        .unwrap_err();
    assert_eq!(refusal.component, Component::Position(0));
    assert_eq!(refusal.cause, ConstructionCause::TypeMismatch);
}

/// A clock field contributes neither a value nor an absent placeholder.
/// Trace: FR-133-AC-1, FR-133-AC-2
#[trace("TC-930", "FR-133-AC-1", "FR-133-AC-2")]
#[test]
fn clock_record_data_shape_round_trips_only_its_named_data_field() {
    let environment = environment();
    let key = common::node(5);
    let fields = || vec![("n", FieldValue::Present(integer(3)))];
    let value = environment.record(key.clone(), fields()).unwrap();
    let Value::Composite(carrier) = &value else {
        panic!("a data record retains the composite carrier");
    };
    assert_eq!(carrier.declaration(), &key);
    let [FieldValue::Present(Value::Integer(n))] = carrier.slots() else {
        panic!("Timer retains only its data field n");
    };
    assert_eq!(n, &Integer::from(3_i64));
    let CompositeShape::Record(declared) = environment.composite(key.clone()).unwrap().shape()
    else {
        panic!("Timer remains a record");
    };
    assert_eq!(declared[0].name(), "n");
    let rebuilt = environment
        .record(
            key.clone(),
            vec![(declared[0].name(), carrier.slots()[0].clone())],
        )
        .unwrap();
    assert!(environment
        .admits(&ValueType::Composite(key.clone()), &rebuilt)
        .unwrap());
    assert_equal(&environment, key.clone(), &value, &rebuilt, true);
    let refusal = environment
        .record(
            key,
            vec![
                ("x", FieldValue::Present(integer(0))),
                ("n", FieldValue::Present(integer(3))),
            ],
        )
        .unwrap_err();
    assert_eq!(refusal.component, Component::Field("x".into()));
    assert_eq!(refusal.cause, ConstructionCause::UndeclaredField);
}

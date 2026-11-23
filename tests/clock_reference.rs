// SPDX-License-Identifier: AGPL-3.0-or-later
//! `ValueType::ClockReference` in declaration admission, equality, supplied
//! membership and object closure, through the public API (TC-932). The kernel
//! owns the type; these tests show this crate adds no rule of its own.
#![cfg(feature = "json-object")]

mod common;

use filament_exact::{
    from_admitted, from_admitted_map, Cancel, ClockPlaceReference, CollectionKind, CollectionType,
    FieldValue, IllTypedCause, Integer, MapType, Meter, ObjectId, ObjectReference, OptionValue,
    Outcome, Presence, Refusal, ScalarLimits, UnionMember, UniverseId, Value, ValueType,
};
use filament_semantic_value::declaration::{
    CompositeDeclaration, CompositeShape, DeclarationCause, EnvironmentFailure, EqualityOperand,
    EqualityOperator, FieldDeclaration, ObjectTypeDeclaration, TypeEnvironment,
    UnionMemberDeclaration,
};
use filament_semantic_value::enumeration::EnumMemberIndex;
use filament_semantic_value::object_closure::ObjectClosure;
use filament_semantic_value::supplied_membership::{
    InvalidMembershipCause, MembershipLocus, MembershipPart, MembershipPathArena,
    SuppliedAdmissionError, SuppliedMembershipBudget,
};
use ix_trace_rs::trace;

const P: &str = r#"["source","a","i","ix://i/calls"]"#;
const S: &str = r#"["source","a","i","ix://i/C"]"#;
const T: &str = r#"[["field",["source","a","i","ix://i/C"],"ix://i/C/x",["clock"]]]"#;

fn hex(json: &str) -> String {
    json.bytes().map(|byte| format!("{byte:02x}")).collect()
}

/// A canonical `place/v2` text whose key component is the JSON string `k`.
fn place_text(k: &str) -> String {
    format!(
        "place/v2/{}/{}/{}/{}/{}",
        hex(P),
        hex(&format!("\"{k}\"")),
        hex(S),
        hex(T),
        hex("[]")
    )
}

fn clock(universe: UniverseId, k: &str) -> Value {
    Value::ClockReference(
        ClockPlaceReference::new(universe, &place_text(k)).expect("canonical place id"),
    )
}

fn clock_type() -> ValueType {
    ValueType::ClockReference
}

fn sequence_of_clock() -> ValueType {
    ValueType::collection(CollectionType::new(
        CollectionKind::Sequence,
        clock_type(),
        None,
    ))
}

fn set_of_clock() -> ValueType {
    ValueType::collection(CollectionType::new(CollectionKind::Set, clock_type(), None))
}

fn clock_keyed_map() -> ValueType {
    ValueType::map(
        MapType::new(clock_type(), ValueType::Integer, None)
            .expect("a clock reference is a leaf key"),
    )
}

fn map_of_clock() -> ValueType {
    ValueType::map(MapType::new(ValueType::String, clock_type(), None).expect("leaf key admits"))
}

fn object_reference(key: &str) -> ObjectReference {
    ObjectReference::new(
        common::universe(1),
        common::effective(1),
        ObjectId::new(key).expect("non-empty key"),
    )
}

fn meter() -> Meter {
    Meter::new(ScalarLimits {
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
    })
}

fn declaring_environment() -> Result<TypeEnvironment, EnvironmentFailure> {
    let union = common::node(3);
    let member = UnionMemberDeclaration::from_verified(
        UnionMember::from_admitted(union.clone(), "Armed").unwrap(),
        vec![clock_type()],
    );
    TypeEnvironment::new(
        [
            CompositeDeclaration::new(
                common::node(1),
                "Call",
                CompositeShape::Record(vec![
                    FieldDeclaration::new("r", clock_type(), Presence::Required),
                    FieldDeclaration::new("o", ValueType::option(clock_type()), Presence::Required),
                    FieldDeclaration::new("s", sequence_of_clock(), Presence::Required),
                    FieldDeclaration::new("m", map_of_clock(), Presence::Required),
                    FieldDeclaration::new("e", set_of_clock(), Presence::Required),
                    FieldDeclaration::new("k", clock_keyed_map(), Presence::Required),
                ]),
            ),
            CompositeDeclaration::new(
                common::node(2),
                "Pair",
                CompositeShape::Tuple(vec![clock_type()]),
            ),
            CompositeDeclaration::new(union, "Phase", CompositeShape::Union(vec![member])),
        ],
        [ObjectTypeDeclaration::new(
            common::effective(1),
            "Holder",
            vec![FieldDeclaration::new("r", clock_type(), Presence::Required)],
        )],
    )
}

/// Declaring a clock-place reference leaf in every member position admits it
/// and resolves it unchanged.
#[trace("TC-932", "FR-135-AC-1")]
#[test]
fn clock_reference_is_an_admitted_leaf_in_every_declaration_position() {
    let environment = declaring_environment().expect("clock-reference members admit");
    assert_eq!(environment.runtime_type(&clock_type()), Some(clock_type()));
    let optional = ValueType::option(clock_type());
    assert_eq!(environment.runtime_type(&optional), Some(optional.clone()));
    for ty in [
        clock_type(),
        optional,
        sequence_of_clock(),
        map_of_clock(),
        set_of_clock(),
        clock_keyed_map(),
    ] {
        assert!(!environment.contains_ieee(&ty));
    }
}

/// An object-typed `Reference` target that is unknown is refused; a clock
/// reference has no target, so the same position admits it.
#[trace("TC-932", "FR-135-AC-1")]
#[test]
fn clock_reference_names_no_object_type_to_resolve() {
    let refusal = TypeEnvironment::new(
        [],
        [ObjectTypeDeclaration::new(
            common::effective(1),
            "Holder",
            vec![FieldDeclaration::new(
                "r",
                ValueType::Reference(common::effective(2)),
                Presence::Required,
            )],
        )],
    )
    .expect_err("an unknown object type is refused");
    assert!(matches!(refusal, EnvironmentFailure::Refused(_)));
    let EnvironmentFailure::Refused(invalid) = refusal else {
        unreachable!()
    };
    assert!(matches!(invalid.cause, DeclarationCause::Type(_)));
    assert!(declaring_environment().is_ok());
}

type Admission = Result<(), SuppliedAdmissionError<InvalidMembershipCause>>;

fn admit(
    environment: &TypeEnvironment,
    expected: &ValueType,
    value: &Value,
) -> (Admission, MembershipLocus) {
    let mut arena = MembershipPathArena::new().unwrap();
    let root = arena.root(0);
    let at = MembershipLocus {
        path: root,
        part: MembershipPart::Value,
    };
    let mut budget = SuppliedMembershipBudget::new(u64::MAX);
    let result = environment.admit_supplied::<InvalidMembershipCause>(
        expected,
        value,
        &mut arena,
        at,
        &mut budget,
        &Cancel::new(),
    );
    (result, at)
}

/// Kernel admission and supplied membership accept a clock-place reference
/// only at `ClockReference`, and refuse every other pairing by kind.
#[trace("TC-932", "FR-135-AC-2")]
#[test]
fn membership_admits_clock_references_only_at_the_clock_reference_type() {
    let environment = declaring_environment().unwrap();
    let reference = clock(common::universe(1), "c/d");
    let object = Value::Reference(object_reference("a"));
    let object_type = ValueType::Reference(common::effective(1));

    assert!(clock_type().admits(&reference));
    assert!(!clock_type().admits(&object));
    assert!(!clock_type().admits(&Value::String(place_text("c/d").as_str().into())));
    assert!(!clock_type().admits(&Value::Integer(Integer::from(1_i64))));
    assert!(!object_type.admits(&reference));

    assert_eq!(admit(&environment, &clock_type(), &reference).0, Ok(()));
    for refused in [
        object,
        Value::String(place_text("c/d").as_str().into()),
        Value::Integer(Integer::from(1_i64)),
    ] {
        let (result, at) = admit(&environment, &clock_type(), &refused);
        assert_eq!(
            result,
            Err(SuppliedAdmissionError::Invalid {
                cause: InvalidMembershipCause::Kind,
                locus: at,
            })
        );
    }
    let (result, at) = admit(&environment, &object_type, &reference);
    assert_eq!(
        result,
        Err(SuppliedAdmissionError::Invalid {
            cause: InvalidMembershipCause::Kind,
            locus: at,
        })
    );

    let sequence = from_admitted(
        CollectionType::new(CollectionKind::Sequence, clock_type(), None),
        vec![reference.clone(), clock(common::universe(1), "c/e")],
    );
    assert_eq!(
        admit(&environment, &sequence_of_clock(), &sequence).0,
        Ok(())
    );
    let optional = OptionValue::from_admitted(clock_type(), Some(reference));
    assert_eq!(
        admit(&environment, &ValueType::option(clock_type()), &optional).0,
        Ok(())
    );
}

fn equality(
    environment: &TypeEnvironment,
    left: &ValueType,
    right: &ValueType,
) -> Result<filament_semantic_value::declaration::CheckedEquality, filament_exact::IllTypedCause> {
    environment
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(left.clone()),
            EqualityOperand::typed(right.clone()),
            &EnumMemberIndex::default(),
        )
        .map_err(|refusal| refusal.cause)
}

/// The semantic equality path leaves the decision to the kernel: unequal
/// texts are `false`, one place is `true`, one text in two universes is a
/// foreign reference, and an object reference is a different type.
#[trace("TC-932", "FR-135-AC-3")]
#[test]
fn equality_follows_the_kernel_decision_for_clock_references() {
    let environment = declaring_environment().unwrap();
    let checked = equality(&environment, &clock_type(), &clock_type()).expect("same type");
    let evaluate =
        |left: &Value, right: &Value| checked.evaluate(&environment, left, right, &mut meter());
    let one = common::universe(1);
    let two = common::universe(2);
    assert_eq!(
        evaluate(&clock(one.clone(), "c/d"), &clock(one.clone(), "c/e")),
        Outcome::Completed(false)
    );
    assert_eq!(
        evaluate(&clock(one.clone(), "c/d"), &clock(one.clone(), "c/d")),
        Outcome::Completed(true)
    );
    assert_eq!(
        evaluate(&clock(one.clone(), "c/d"), &clock(two.clone(), "c/e")),
        Outcome::Completed(false)
    );
    assert!(matches!(
        evaluate(&clock(one, "c/d"), &clock(two, "c/d")),
        Outcome::Refused(Refusal::ForeignReference { .. })
    ));
    let mismatch = equality(
        &environment,
        &clock_type(),
        &ValueType::Reference(common::effective(1)),
    );
    assert!(matches!(mismatch, Err(IllTypedCause::TypeMismatch)));
}

fn holder_types(attribute: ValueType) -> TypeEnvironment {
    TypeEnvironment::new(
        [],
        [ObjectTypeDeclaration::new(
            common::effective(1),
            "Holder",
            vec![FieldDeclaration::new("r", attribute, Presence::Required)],
        )],
    )
    .expect("one clock-reference-bearing object type admits")
}

fn holder_closure(types: &TypeEnvironment, value: Value) -> ObjectClosure {
    ObjectClosure::new(
        types,
        [(
            object_reference("holder"),
            vec![("r", FieldValue::Present(value))],
        )],
        &[],
    )
    .expect("a clock-place reference names no object, so nothing dangles")
}

/// A clock-place reference adds no closure edge: the closure of an object
/// holding one admits with no other object and holds exactly that object.
#[trace("TC-932", "FR-135-AC-4")]
#[test]
fn clock_references_add_no_object_closure_edge() {
    let one = common::universe(1);
    let reference = clock(one.clone(), "c/d");
    let other_place = clock(one.clone(), "c/e");

    let bare = holder_types(clock_type());
    let admitted = holder_closure(&bare, reference.clone());
    assert_eq!(admitted.iter().count(), 1);
    let held = admitted
        .object(&object_reference("holder"))
        .and_then(|attributes| attributes.get("r"));
    assert!(matches!(
        held,
        Some(FieldValue::Present(Value::ClockReference(actual)))
            if actual.text() == place_text("c/d") && actual.universe() == &common::universe(1)
    ));
    assert_eq!(admitted, holder_closure(&bare, clock(one.clone(), "c/d")));
    assert_ne!(admitted, holder_closure(&bare, other_place.clone()));
    // The same place id text in another universe is another reference.
    assert_ne!(
        admitted,
        holder_closure(&bare, clock(common::universe(2), "c/d"))
    );

    let optional = holder_types(ValueType::option(clock_type()));
    let present = |value: &Value| OptionValue::from_admitted(clock_type(), Some(value.clone()));
    let admitted = holder_closure(&optional, present(&reference));
    assert_eq!(admitted.iter().count(), 1);
    assert_eq!(admitted, holder_closure(&optional, present(&reference)));
    assert_ne!(admitted, holder_closure(&optional, present(&other_place)));

    let mapped = holder_types(map_of_clock());
    let map = |value: &Value| {
        from_admitted_map(
            MapType::new(ValueType::String, clock_type(), None).expect("leaf key admits"),
            vec![(Value::String("k".into()), value.clone())],
        )
        .expect("map entries admit")
    };
    let admitted = holder_closure(&mapped, map(&reference));
    assert_eq!(admitted.iter().count(), 1);
    assert_eq!(admitted, holder_closure(&mapped, map(&reference)));
    assert_ne!(admitted, holder_closure(&mapped, map(&other_place)));
}

// SPDX-License-Identifier: AGPL-3.0-or-later
//! External checking under real allocator denial in its own integration binary.
mod common;

use filament_exact::{
    CollectionKind, CollectionType, FloatType, IeeeWidth, IllTyped, IllTypedCause, MapType,
    Presence, ValueType,
};
use filament_semantic_value::declaration::{
    CompositeDeclaration, CompositeShape, FieldDeclaration, StorageRequest, StorageUnit,
    TypeCheckError, TypeEnvironment,
};
use ix_trace_rs::trace;

#[global_allocator]
static ALLOCATOR: fsv_failing_allocator::FailingAllocator = fsv_failing_allocator::FailingAllocator;

fn collection(kind: CollectionKind, element: ValueType) -> ValueType {
    ValueType::collection(CollectionType::new(kind, element, None))
}

fn refused(cause: IllTypedCause) -> Result<(), TypeCheckError> {
    Err(TypeCheckError::Refused(IllTyped { cause }))
}

#[trace("TC-929", "FR-132-AC-2", "FR-132-AC-4")]
#[test]
fn float_bearing_set_type_is_admitted_and_root_denials_retry() {
    let key = common::node(1);
    let environment = TypeEnvironment::new(
        [CompositeDeclaration::new(
            key.clone(),
            "Three",
            CompositeShape::Tuple(vec![
                ValueType::Boolean,
                ValueType::Integer,
                ValueType::Float(FloatType::exact(IeeeWidth::Binary64)),
            ]),
        )],
        [],
    )
    .unwrap();
    let original_environment = environment.clone();
    let ty = collection(CollectionKind::Set, ValueType::Composite(key));
    assert!(environment.contains_ieee(&ty));
    assert_eq!(environment.try_check_type(&ty), Ok(()));
    // Root type-walk allocation denial leaves the environment unchanged and
    // the same type succeeds when retried.
    let pointer = core::mem::size_of::<&ValueType>();
    // Success and unknown-key refusal both recover from the root denial; the
    // latter must not clone the unknown structural identity for its error.
    for (ty, oracle) in [
        (ValueType::Integer, Ok(())),
        (
            ValueType::Composite(common::node(99)),
            refused(IllTypedCause::TypeMismatch),
        ),
    ] {
        fsv_failing_allocator::arm(0);
        let result = environment.try_check_type(&ty);
        let denied = fsv_failing_allocator::disarm();
        assert_eq!(denied, Some(pointer));
        assert_eq!(
            result,
            Err(TypeCheckError::Allocation(StorageRequest::new(
                1,
                StorageUnit::AdditionalElements
            )))
        );
        assert_eq!(environment.try_check_type(&ty), oracle);
        assert_eq!(environment, original_environment);
    }
}

#[trace("TC-929")]
#[test]
fn map_link_inspection_propagates_type_walk_allocation_denial() {
    let float = ValueType::Float(FloatType::exact(IeeeWidth::Binary64));
    let map = ValueType::map(MapType::new(ValueType::String, float, None).unwrap());
    let environment = TypeEnvironment::new(
        [CompositeDeclaration::new(
            common::node(1),
            "MapRecord",
            CompositeShape::Record(vec![FieldDeclaration::new(
                "value",
                map,
                Presence::Required,
            )]),
        )],
        [],
    )
    .unwrap();
    let ty = collection(CollectionKind::Set, ValueType::Composite(common::node(1)));

    // The public checker reserves its root worklist first. Deny the next
    // reservation, which is the metered walk that searches this composite for
    // a Map link.
    fsv_failing_allocator::arm(1);
    let result = environment.try_check_type(&ty);
    let denied_bytes = fsv_failing_allocator::disarm();

    assert!(
        denied_bytes.is_some(),
        "the Map-link traversal allocation was denied"
    );
    assert_eq!(
        result,
        Err(TypeCheckError::Allocation(StorageRequest::new(
            1,
            StorageUnit::AdditionalElements
        )))
    );
    assert_eq!(
        environment.try_check_type(&ty),
        refused(IllTypedCause::OperatorIneligible),
        "a failed check leaves the environment and retry behavior intact"
    );
}

#[trace("TC-929", "FR-132-AC-1")]
#[test]
fn external_semantics_preserve_nested_traversal_and_refusal_priority() {
    let environment = TypeEnvironment::new([], []).unwrap();
    for ty in [
        ValueType::Boolean,
        ValueType::Integer,
        ValueType::String,
        ValueType::option(ValueType::option(ValueType::Integer)),
        collection(
            CollectionKind::Sequence,
            ValueType::Float(FloatType::exact(IeeeWidth::Binary64)),
        ),
    ] {
        assert_eq!(environment.try_check_type(&ty), Ok(()));
    }
    for ty in [
        ValueType::Composite(common::node(9)),
        ValueType::Reference(common::effective(9)),
    ] {
        assert_eq!(
            environment.try_check_type(&ty),
            refused(IllTypedCause::TypeMismatch)
        );
    }
    for ty in [
        ValueType::Population(None),
        ValueType::option(ValueType::Population(None)),
    ] {
        assert_eq!(
            environment.try_check_type(&ty),
            refused(IllTypedCause::OperatorIneligible)
        );
    }
    let unknown = ValueType::Composite(common::node(999));
    fsv_failing_allocator::arm(1);
    let unknown_result = environment.try_check_type(&unknown);
    let denied = fsv_failing_allocator::disarm();
    assert_eq!(
        denied, None,
        "unknown-key errors allocate no identity clone"
    );
    assert_eq!(unknown_result, refused(IllTypedCause::TypeMismatch));
    for kind in [
        CollectionKind::Set,
        CollectionKind::Bag,
        CollectionKind::OrderedSet,
    ] {
        let ieee = ValueType::option(ValueType::Float(FloatType::exact(IeeeWidth::Binary64)));
        assert_eq!(environment.try_check_type(&collection(kind, ieee)), Ok(()));
        assert_eq!(
            environment.try_check_type(&collection(kind, ValueType::Composite(common::node(9)))),
            refused(IllTypedCause::TypeMismatch)
        );
    }
}

#[trace("TC-929", "FR-132-AC-1")]
#[test]
fn ieee_discovery_preserves_record_tuple_union_and_cycle_behavior() {
    use filament_exact::{Identifier, MapType, Presence};
    use filament_semantic_value::declaration::{
        FieldDeclaration, ObjectTypeDeclaration, UnionMemberDeclaration,
    };
    let float = ValueType::Float(FloatType::exact(IeeeWidth::Binary64));
    let environment = TypeEnvironment::new(
        [
            CompositeDeclaration::new(
                common::node(1),
                "Record",
                CompositeShape::Record(vec![FieldDeclaration::new(
                    "value",
                    float.clone(),
                    Presence::Required,
                )]),
            ),
            CompositeDeclaration::new(
                common::node(2),
                "Tuple",
                CompositeShape::Tuple(vec![ValueType::Composite(common::node(1))]),
            ),
            CompositeDeclaration::new(
                common::node(3),
                "Union",
                CompositeShape::Union(vec![
                    UnionMemberDeclaration::resolved(
                        Identifier::new("Leaf").unwrap(),
                        vec![ValueType::Composite(common::node(2))],
                    ),
                    UnionMemberDeclaration::resolved(
                        Identifier::new("Next").unwrap(),
                        vec![ValueType::option(ValueType::Composite(common::node(3)))],
                    ),
                ]),
            ),
            CompositeDeclaration::new(
                common::node(4),
                "Cycle",
                CompositeShape::Union(vec![UnionMemberDeclaration::resolved(
                    Identifier::new("Next").unwrap(),
                    vec![ValueType::option(ValueType::Composite(common::node(4)))],
                )]),
            ),
        ],
        [ObjectTypeDeclaration::new(
            common::effective(1),
            "Object",
            vec![],
        )],
    )
    .unwrap();
    assert_eq!(
        environment.try_check_type(&ValueType::Reference(common::effective(1))),
        Ok(())
    );
    for number in 1..=4 {
        let composite = ValueType::Composite(common::node(number));
        assert_eq!(environment.try_check_type(&composite), Ok(()));
        let expected = if number == 4 {
            Ok(())
        } else {
            refused(IllTypedCause::OperatorIneligible)
        };
        for kind in [
            CollectionKind::Set,
            CollectionKind::Bag,
            CollectionKind::OrderedSet,
        ] {
            assert_eq!(
                environment.try_check_type(&collection(kind, composite.clone())),
                expected
            );
        }
    }
    let map =
        ValueType::map(MapType::new(ValueType::String, ValueType::option(float), None).unwrap());
    assert_eq!(environment.try_check_type(&map), Ok(()));
    assert_eq!(
        environment.try_check_type(&collection(CollectionKind::Set, map)),
        refused(IllTypedCause::OperatorIneligible)
    );
}

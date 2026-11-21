// SPDX-License-Identifier: AGPL-3.0-or-later
//! FR-134 admission of dimension-class projections; runtime values keep using
//! the quantity core's edge-by-edge meter.

use super::{Dimension, NominalUnitForm, UnitEdge, UnitGraph, UnmeteredAffine};
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use filament_exact::{Integer, NodeKey, Rational};

/// The first malformed membership row, without a projected graph.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("dimension equivalence for {dimension}: {cause}")]
pub struct DimensionEquivalenceRefusal {
    /// The member or referenced dimension that failed admission.
    pub dimension: NodeKey,
    /// The failed membership check.
    pub cause: DimensionEquivalenceCause,
}

/// The closed membership-refusal vocabulary, in row-check precedence order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum DimensionEquivalenceCause {
    /// A member or its representative is absent from the graph.
    #[error("unknown dimension")]
    UnknownDimension,
    /// A dimension has more than one membership row.
    #[error("duplicate dimension")]
    DuplicateDimension,
    /// A participating dimension has no canonical unit root.
    #[error("missing canonical root")]
    MissingRoot,
    /// The representative has no self-representative row.
    #[error("missing representative row")]
    MissingRepresentative,
    /// The representative is not the least member key.
    #[error("noncanonical representative")]
    NonCanonicalRepresentative,
    /// A root scale is zero or negative.
    #[error("nonpositive root scale")]
    NonPositiveScale,
    /// The representative's scale is not one.
    #[error("nonidentity representative")]
    NonIdentityRepresentative,
    /// A class containing a derived dimension has inconsistent normalized maps.
    #[error("incompatible class")]
    IncompatibleClass,
}

fn refusal(dimension: &NodeKey, cause: DimensionEquivalenceCause) -> DimensionEquivalenceRefusal {
    DimensionEquivalenceRefusal {
        dimension: dimension.clone(),
        cause,
    }
}

impl UnitGraph {
    /// Project admitted dimension classes onto this graph's nominal unit nodes.
    ///
    /// Rows contain `(dimension, representative, positive root scale)`. The
    /// caller admits declaration cycles and implied ratios; this method checks
    /// the row contract of FR-134 and preserves nominal identities. Reprojection
    /// starts from the nominal graph, so identification edges never accumulate.
    /// An empty row slice leaves the graph unchanged.
    ///
    /// # Errors
    /// Returns the first invalid row and its first applicable typed cause.
    pub fn with_dimension_equivalences(
        &self,
        members: &[(NodeKey, NodeKey, Rational)],
    ) -> Result<Self, DimensionEquivalenceRefusal> {
        if members.is_empty() {
            return Ok(self.clone());
        }
        let original = self.nominal_dimensions();
        let base_classes = self.base_classes(members);
        let dimensions: BTreeMap<_, _> = original
            .iter()
            .map(|(key, map)| (key.clone(), normalize(map, &base_classes)))
            .collect();
        self.check_equivalences(members, &dimensions)?;
        let mut projected = self.clone();
        projected.dimensions = dimensions;
        for unit in projected.units.values_mut() {
            let nominal_dimension = unit.dimension_node.clone();
            let (root, path, canonical) = self.nominal_path(&unit.key, &nominal_dimension)?;
            unit.root = root;
            unit.path = path;
            unit.canonical = canonical;
            unit.conversion_class = nominal_dimension.clone();
            if let Some(dimension) = projected.dimensions.get(&nominal_dimension) {
                unit.dimension = dimension.clone();
            }
            if let Some((_, representative, scale)) =
                members.iter().find(|row| row.0 == nominal_dimension)
            {
                unit.conversion_class = representative.clone();
                if representative != &nominal_dimension {
                    let edge = UnitEdge::from_admitted(
                        scale.clone(),
                        Rational::from_integer(Integer::zero()),
                    );
                    unit.canonical = unit
                        .canonical
                        .followed_by(&edge, &mut UnmeteredAffine)
                        .unwrap_or_else(|never| match never {});
                    unit.path.push(edge);
                    unit.root = self.nominal_root(representative).ok_or_else(|| {
                        refusal(representative, DimensionEquivalenceCause::MissingRoot)
                    })?;
                }
            }
        }
        Ok(projected)
    }

    fn nominal_dimensions(&self) -> BTreeMap<NodeKey, Dimension> {
        self.nominal
            .values()
            .filter_map(|node| match &node.form {
                NominalUnitForm::Dimension { base, terms } => Some((
                    node.key.clone(),
                    Dimension::from_admitted(if *base {
                        alloc::vec![(node.key.clone(), Integer::one())]
                    } else {
                        terms.clone()
                    }),
                )),
                NominalUnitForm::Unit { .. } => None,
            })
            .collect()
    }

    fn is_base(&self, dimension: &NodeKey) -> bool {
        matches!(
            self.nominal.get(dimension).map(|node| &node.form),
            Some(NominalUnitForm::Dimension { base: true, .. })
        )
    }

    fn base_classes(&self, members: &[(NodeKey, NodeKey, Rational)]) -> BTreeMap<NodeKey, NodeKey> {
        members
            .iter()
            .filter(|row| self.is_base(&row.0))
            .filter_map(|(dimension, representative, _)| {
                members
                    .iter()
                    .filter(|row| &row.1 == representative && self.is_base(&row.0))
                    .map(|row| &row.0)
                    .min()
                    .map(|base| (dimension.clone(), base.clone()))
            })
            .collect()
    }

    fn nominal_root(&self, dimension: &NodeKey) -> Option<NodeKey> {
        self.nominal.values().find_map(|node| match &node.form {
            NominalUnitForm::Unit {
                dimension: found,
                target: None,
                ..
            } if found == dimension => Some(node.key.clone()),
            _ => None,
        })
    }

    fn check_equivalences(
        &self,
        members: &[(NodeKey, NodeKey, Rational)],
        dimensions: &BTreeMap<NodeKey, Dimension>,
    ) -> Result<(), DimensionEquivalenceRefusal> {
        use DimensionEquivalenceCause as Cause;
        let mut seen = BTreeSet::new();
        let one = Rational::from_integer(Integer::one());
        let zero = Rational::from_integer(Integer::zero());
        for (dimension, representative, scale) in members {
            for key in [dimension, representative] {
                if !self.dimensions.contains_key(key) {
                    return Err(refusal(key, Cause::UnknownDimension));
                }
            }
            if !seen.insert(dimension) {
                return Err(refusal(dimension, Cause::DuplicateDimension));
            }
            for key in [dimension, representative] {
                if self.nominal_root(key).is_none() {
                    return Err(refusal(key, Cause::MissingRoot));
                }
            }
            if !members
                .iter()
                .any(|row| &row.0 == representative && &row.1 == representative)
            {
                return Err(refusal(representative, Cause::MissingRepresentative));
            }
            if members
                .iter()
                .any(|row| &row.1 == representative && &row.0 < representative)
            {
                return Err(refusal(dimension, Cause::NonCanonicalRepresentative));
            }
            if scale <= &zero {
                return Err(refusal(dimension, Cause::NonPositiveScale));
            }
            if dimension == representative && scale != &one {
                return Err(refusal(dimension, Cause::NonIdentityRepresentative));
            }
            let has_derived = members
                .iter()
                .any(|row| &row.1 == representative && !self.is_base(&row.0));
            if has_derived
                && members.iter().any(|row| {
                    &row.1 == representative && dimensions.get(&row.0) != dimensions.get(dimension)
                })
            {
                return Err(refusal(dimension, Cause::IncompatibleClass));
            }
        }
        Ok(())
    }

    fn nominal_path(
        &self,
        key: &NodeKey,
        dimension: &NodeKey,
    ) -> Result<(NodeKey, Vec<UnitEdge>, UnitEdge), DimensionEquivalenceRefusal> {
        let mut current = key.clone();
        let mut path = Vec::new();
        let mut canonical = UnitEdge::identity();
        loop {
            match self.nominal.get(&current).map(|node| &node.form) {
                Some(NominalUnitForm::Unit {
                    target: Some(target),
                    scale,
                    offset,
                    ..
                }) => {
                    let edge = UnitEdge::from_admitted(scale.clone(), offset.clone());
                    canonical = canonical
                        .followed_by(&edge, &mut UnmeteredAffine)
                        .unwrap_or_else(|never| match never {});
                    path.push(edge);
                    current = target.clone();
                }
                Some(NominalUnitForm::Unit { target: None, .. }) => {
                    return Ok((current, path, canonical))
                }
                _ => return Err(refusal(dimension, DimensionEquivalenceCause::MissingRoot)),
            }
        }
    }
}

fn normalize(dimension: &Dimension, base_classes: &BTreeMap<NodeKey, NodeKey>) -> Dimension {
    let mut normalized = BTreeMap::new();
    for (base, exponent) in &dimension.0 {
        let key = base_classes.get(base).unwrap_or(base).clone();
        let prior = normalized.entry(key).or_insert_with(Integer::zero);
        *prior = prior.add(exponent);
    }
    normalized.retain(|_, exponent| !exponent.is_zero());
    Dimension(normalized)
}

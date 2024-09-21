#![allow(unused)]
#![allow(unused_variables)]

type Result<T, E = String> = std::result::Result<T, E>;

use indexmap::IndexSet;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorSource {
    pub factor_source_id: FactorSourceIDFromHash,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorSourceIDFromHash;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NetworkID;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CAP26KeyKind;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CAP26EntityKind;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeySpace;

/// Lacks the index
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PartialDerivationRequest {
    factor_source_id: FactorSourceIDFromHash,
    network_id: NetworkID,
    key_kind: CAP26KeyKind,
    entity_kind: CAP26EntityKind,
    key_space: KeySpace,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct FactorSources(IndexSet<FactorSource>);
impl FactorSources {
    fn can_derive(&self, _partial_derivation_request: &PartialDerivationRequests) -> bool {
        true
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct PartialDerivationRequests(IndexSet<PartialDerivationRequest>);

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct KnownTakenFactorInstances;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProbablyFreeFactorInstances;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnChainAnalyzer;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProfileAnalyzer;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cache;

pub trait DerivationInteractors {
    fn call(&self);
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DerivationsAndAnalysis {
    pub known_taken: KnownTakenFactorInstances,
    pub probably_free: ProbablyFreeFactorInstances,
}

pub async fn _derive_many(
    factor_sources: FactorSources,
    partial_derivation_request: PartialDerivationRequests,
    maybe_cache: Option<Cache>,
    maybe_onchain_analyser: Option<OnChainAnalyzer>,
    maybe_profile_analyser: Option<ProfileAnalyzer>,
    derivation_interactors: Arc<dyn DerivationInteractors>,
) -> Result<DerivationsAndAnalysis> {
    assert!(factor_sources.can_derive(&partial_derivation_request));
    assert!(
        !(maybe_cache.is_none()
            && maybe_onchain_analyser.is_none()
            && maybe_profile_analyser.is_none())
    );

    Ok(DerivationsAndAnalysis::default())
}

#![allow(unused)]
#![allow(unused_variables)]

use indexmap::IndexSet;
use std::sync::Arc;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct FactorSource;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct FactorSourceIDFromHash;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct NetworkID;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct CAP26KeyKind;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct CAP26EntityKind;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct KeySpace;

/// Lacks the index
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct PartialDerivationRequest {
    factor_source_id: FactorSourceIDFromHash,
    network_id: NetworkID,
    key_kind: CAP26KeyKind,
    entity_kind: CAP26EntityKind,
    key_space: KeySpace,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct FactorSources(IndexSet<FactorSource>);
impl FactorSources {
    fn can_derive(&self, _partial_derivation_request: &PartialDerivationRequests) -> bool {
        true
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct PartialDerivationRequests(IndexSet<PartialDerivationRequest>);

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct KnownTakenFactorInstances;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct ProbablyFreeFactorInstances;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct OnChainAnalyzer;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct ProfileAnalyzer;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct Cache;

trait DerivationInteractors {
    fn call(&self);
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct DerivationsAndAnalysis {
    known_taken: KnownTakenFactorInstances,
    probably_free: ProbablyFreeFactorInstances,
}

// fn __derivation_indices()

type Result<T, E = String> = std::result::Result<T, E>;

async fn _derive_many(
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

fn main() {
    println!("Hello, world!");
}

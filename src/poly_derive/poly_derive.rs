#![allow(unused)]
#![allow(unused_variables)]

use crate::prelude::*;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct FactorSources(IndexSet<FactorSource>);
impl FactorSources {
    fn can_derive(&self, abstract_derivation_request: &AbstractDerivationRequests) -> bool {
        true
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct KnownTakenFactorInstances {
    pub recovered_unsecurified_accounts: IndexSet<UnsecurifiedAccount>,
    pub recovered_securified_accounts: IndexSet<SecurifiedAccount>,
}
impl KnownTakenFactorInstances {
    pub fn recovered_accounts(&self) -> IndexSet<Account> {
        let mut accounts = IndexSet::new();
        accounts.extend(
            self.recovered_unsecurified_accounts
                .iter()
                .map(|a| Account::Unsecurified(a.clone())),
        );
        accounts.extend(
            self.recovered_securified_accounts
                .iter()
                .map(|a| Account::Securified(a.clone())),
        );
        accounts
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct ProbablyFreeFactorInstances(pub IndexSet<FactorInstance>);

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct FactorInstances(pub IndexSet<FactorInstance>);

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnChainAnalyzer;
impl OnChainAnalyzer {
    pub fn new(gateway: Arc<dyn Gateway>) -> Self {
        Self
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProfileAnalyzer;

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cache;
impl Cache {
    pub fn new(probably_free_factor_instances: ProbablyFreeFactorInstances) -> Self {
        Self
    }
}

pub trait DerivationInteractors {
    fn call(&self);
}

pub trait Gateway {
    fn call(&self);
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct DerivationsAndAnalysis {
    pub known_taken: KnownTakenFactorInstances,
    pub probably_free: ProbablyFreeFactorInstances,
}

pub type HDPathValue = u32;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CAP26Index {
    Unsecurified(HDPathValue),
    Securified(HDPathValue),
}
impl CAP26Index {
    pub fn key_space(&self) -> KeySpace {
        match self {
            CAP26Index::Unsecurified(_) => KeySpace::Unsecurified,
            CAP26Index::Securified(_) => KeySpace::Securified,
        }
    }
}

pub struct KeysCollector;
impl KeysCollector {
    pub fn new(
        factor_sources: FactorSources,
        derivation_paths: IndexMap<FactorSourceIDFromHash, IndexSet<DerivationPath>>,
        interactors: Arc<dyn DerivationInteractors>,
    ) -> Result<Self> {
        Ok(Self)
    }

    pub async fn derive() -> Result<FactorInstances> {
        Ok(FactorInstances::default())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnsecurifiedAccount(pub FactorInstance);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SecurifiedAccount(pub Vec<FactorInstance>);

pub enum PolyDeriveRequestKind {
    OARS,
}

pub struct PolyDeriveInput {
    network_id: NetworkID,
    factor_sources: FactorSources,
    request_kind: PolyDeriveRequestKind,
    maybe_cache: Option<Cache>,
    maybe_onchain_analyser: Option<OnChainAnalyzer>,
    maybe_profile_analyser: Option<ProfileAnalyzer>,
    derivation_interactors: Arc<dyn DerivationInteractors>,
}

impl PolyDeriveInput {
    fn new(
        network_id: NetworkID,
        factor_sources: FactorSources,
        request_kind: PolyDeriveRequestKind,
        maybe_cache: impl Into<Option<Cache>>,
        maybe_onchain_analyser: impl Into<Option<OnChainAnalyzer>>,
        maybe_profile_analyser: impl Into<Option<ProfileAnalyzer>>,
        derivation_interactors: Arc<dyn DerivationInteractors>,
    ) -> Self {
        let maybe_cache = maybe_cache.into();
        let maybe_onchain_analyser = maybe_onchain_analyser.into();
        let maybe_profile_analyser = maybe_profile_analyser.into();

        assert!(
            !(maybe_cache.is_none()
                && maybe_onchain_analyser.is_none()
                && maybe_profile_analyser.is_none())
        );
        Self {
            network_id,
            factor_sources,
            request_kind,
            maybe_cache,
            maybe_onchain_analyser,
            maybe_profile_analyser,
            derivation_interactors,
        }
    }

    pub fn oars(
        factor_sources: FactorSources,
        gateway: Arc<dyn Gateway>,
        derivation_interactors: Arc<dyn DerivationInteractors>,
    ) -> Self {
        Self::new(
            NetworkID::Mainnet,
            factor_sources,
            PolyDeriveRequestKind::OARS,
            None,
            OnChainAnalyzer::new(gateway),
            None,
            derivation_interactors,
        )
    }
}

async fn _poly_derive(input: PolyDeriveInput) -> Result<DerivationsAndAnalysis> {
    // let mut instances = FactorInstances::default();
    // let indices = ...;
    // let requests = partial_derivation_request.materialize(indices);
    // if let Some(ref cache) = maybe_cache {
    //     let cached = cache.load(&requests).await?;
    // }

    Ok(DerivationsAndAnalysis::default())
}

/// onboarding account recover scan
pub async fn oars(
    factor_sources: FactorSources,
    derivation_interactors: Arc<dyn DerivationInteractors>,
    gateway: Arc<dyn Gateway>,
) -> Result<(Profile, Cache)> {
    let analysis = _poly_derive(PolyDeriveInput::oars(
        factor_sources.clone(),
        gateway,
        derivation_interactors,
    ))
    .await?;

    let cache = Cache::new(analysis.probably_free);
    let accounts = analysis.known_taken.recovered_accounts();
    let profile = Profile::new(factor_sources, accounts);

    Ok((profile, cache))
}

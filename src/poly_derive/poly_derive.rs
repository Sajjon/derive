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
impl FactorInstances {
    pub fn from(iter: impl IntoIterator<Item = FactorInstance>) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[derive(Default, Clone)]
pub struct OnChainAnalyzer {
    gateway: Option<Arc<dyn Gateway>>,
}
impl OnChainAnalyzer {
    fn new(gateway: impl Into<Option<Arc<dyn Gateway>>>) -> Self {
        Self {
            gateway: gateway.into(),
        }
    }

    pub fn with_gateway(gateway: Arc<dyn Gateway>) -> Self {
        Self::new(gateway)
    }

    pub fn dummy() -> Self {
        Self::new(None)
    }
}

#[derive(Default, Clone)]
pub struct ProfileAnalyzer {
    profile: Option<Arc<Profile>>,
}
impl ProfileAnalyzer {
    fn new(profile: impl Into<Option<Arc<Profile>>>) -> Self {
        Self {
            profile: profile.into(),
        }
    }

    pub fn with_profile(profile: Arc<Profile>) -> Self {
        Self::new(profile)
    }

    pub fn dummy() -> Self {
        Self::new(None)
    }
}

#[derive(Debug)]
pub struct Cache {
    factor_instances_for_requests: RwLock<IndexMap<DerivationRequestInKeySpace, FactorInstances>>,
}
impl Cache {
    fn with_map(map: IndexMap<DerivationRequestInKeySpace, FactorInstances>) -> Self {
        Self {
            factor_instances_for_requests: RwLock::new(map),
        }
    }
    pub fn new(probably_free_factor_instances: ProbablyFreeFactorInstances) -> Self {
        let map = probably_free_factor_instances
            .0
            .into_iter()
            .into_group_map_by(|x| x.derivation_in_key_space())
            .into_iter()
            .map(|(k, v)| (k, FactorInstances::from(v)))
            .collect::<IndexMap<DerivationRequestInKeySpace, FactorInstances>>();

        Self::with_map(map)
    }
    pub fn empty() -> Self {
        Self::with_map(IndexMap::default())
    }
}

pub trait DerivationInteractors {
    fn call(&self);
}

pub trait Gateway {
    fn call(&self);
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct IntermediaryDerivationsAndAnalysis {
    pub known_taken: KnownTakenFactorInstances,
    pub probably_free: ProbablyFreeFactorInstances,
}

#[derive(Clone, Debug)]
pub struct FinalDerivationsFinalAndAnalysis {
    pub recovered_accounts: IndexSet<Account>,
    pub cache: Arc<Cache>,
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

pub struct PolyDerivation {
    network_id: NetworkID,
    factor_sources: FactorSources,
    request_kind: PolyDeriveRequestKind,

    /// If no cache present, a new one is created and will be filled.
    cache: Arc<Cache>,

    /// If not present (no Gateway) or if offline, a "dummy" one is used
    /// which says everything is free.
    onchain_analyser: OnChainAnalyzer,

    /// If not present (no Profile) a dummy one is used which says everything is free.
    profile_analyser: ProfileAnalyzer,

    /// GUI hook
    derivation_interactors: Arc<dyn DerivationInteractors>,
}

impl PolyDerivation {
    fn new(
        network_id: NetworkID,
        factor_sources: FactorSources,
        request_kind: PolyDeriveRequestKind,
        maybe_cache: impl Into<Option<Arc<Cache>>>,
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
            cache: maybe_cache.unwrap_or_else(|| Arc::new(Cache::empty())),
            onchain_analyser: maybe_onchain_analyser.unwrap_or_else(OnChainAnalyzer::dummy),
            profile_analyser: maybe_profile_analyser.unwrap_or_else(ProfileAnalyzer::dummy),
            derivation_interactors,
        }
    }

    pub fn oars(
        factor_sources: &FactorSources,
        gateway: Arc<dyn Gateway>,
        derivation_interactors: Arc<dyn DerivationInteractors>,
    ) -> Self {
        Self::new(
            NetworkID::Mainnet,
            factor_sources.clone(),
            PolyDeriveRequestKind::OARS,
            None,
            OnChainAnalyzer::new(gateway),
            None,
            derivation_interactors,
        )
    }
}

impl PolyDerivation {
    async fn initial_analysis(&self) -> Result<IntermediaryDerivationsAndAnalysis> {
        Ok(IntermediaryDerivationsAndAnalysis::default())
    }
    async fn next_analysis(&self, analysis: &mut IntermediaryDerivationsAndAnalysis) -> Result<()> {
        Ok(())
    }

    async fn is_done(&self, analysis: &IntermediaryDerivationsAndAnalysis) -> Result<bool> {
        Ok(false)
    }

    pub async fn poly_derive(self) -> Result<FinalDerivationsFinalAndAnalysis> {
        let mut analysis = self.initial_analysis().await?;
        loop {
            let is_done = self.is_done(&analysis).await?;
            if is_done {
                break;
            }
            self.next_analysis(&mut analysis).await?;
        }
        Ok(FinalDerivationsFinalAndAnalysis {
            recovered_accounts: analysis.known_taken.recovered_accounts(),
            cache: self.cache,
        })
    }
}

/// onboarding account recover scan
pub async fn oars(
    factor_sources: FactorSources,
    interactors: Arc<dyn DerivationInteractors>,
    gateway: Arc<dyn Gateway>,
) -> Result<(Profile, Arc<Cache>)> {
    let derivation = PolyDerivation::oars(&factor_sources, gateway, interactors);

    let analysis = derivation.poly_derive().await?;
    let cache = analysis.cache;
    let accounts = analysis.recovered_accounts;
    let profile = Profile::new(factor_sources, accounts);

    Ok((profile, cache))
}

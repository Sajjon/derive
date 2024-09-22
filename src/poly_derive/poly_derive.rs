#![allow(unused)]
#![allow(unused_variables)]

use std::net;

use crate::prelude::*;

pub enum PolyDeriveRequestKind {
    /// Onboarding Account Recovery Scan
    /// Assumes `Mainnet`
    OARS { factor_sources: FactorSources },

    /// Manual Account Recovery Scan
    /// Done using a single FactorSource
    MARS {
        factor_source: FactorSource,
        network_id: NetworkID,
    },

    /// PreDerive FactorInstances for new FactorSource
    PreDeriveInstancesForNewFactorSource { factor_source: FactorSource },

    /// New Virtual Unsecurified Account
    NewVirtualUnsecurifiedAccount {
        network_id: NetworkID,
        factor_source: FactorSource,
    },

    /// Securify unsecurified Account
    SecurifyUnsecurifiedAccount {
        unsecurified_account: UnsecurifiedAccount,
        matrix_of_factor_sources: MatrixOfFactorSources,
    },

    /// Securify unsecurified Account
    UpdateSecurifiedAccount {
        securified_account: SecurifiedAccount,
        matrix_of_factor_sources: MatrixOfFactorSources,
    },
}

/// Offsets to next derivation entity index to use for a given
/// factor source selector.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct FactorInstancesCacheCursors {
    map: IndexMap<FactorSourceIDFromHash, HDPathValue>,
}

pub struct PolyDerivation {
    request_kind: PolyDeriveRequestKind,

    /// If no cache present, a new one is created and will be filled.
    cache: Arc<Cache>,

    /// If not present (no Gateway) or if offline, a "dummy" one is used
    /// which says everything is free.
    onchain_analyser: OnChainAnalyzer,

    /// If not present (no Profile) a dummy one is used which says everything is free.
    profile_analyser: ProfileAnalyzer,

    /// GUI hooks
    derivation_interactors: Arc<dyn DerivationInteractors>,
    is_derivation_done_query: Arc<dyn IsDerivationDoneQuery>,
}

impl PolyDerivation {
    fn new(
        request_kind: PolyDeriveRequestKind,
        maybe_cache: impl Into<Option<Arc<Cache>>>,
        maybe_onchain_analyser: impl Into<Option<OnChainAnalyzer>>,
        maybe_profile_analyser: impl Into<Option<ProfileAnalyzer>>,
        derivation_interactors: Arc<dyn DerivationInteractors>,
        is_derivation_done_query: Arc<dyn IsDerivationDoneQuery>,
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
            request_kind,
            cache: maybe_cache.unwrap_or_else(|| Arc::new(Cache::default())),
            onchain_analyser: maybe_onchain_analyser.unwrap_or_else(OnChainAnalyzer::dummy),
            profile_analyser: maybe_profile_analyser.unwrap_or_else(ProfileAnalyzer::dummy),
            derivation_interactors,
            is_derivation_done_query,
        }
    }

    pub fn oars(
        factor_sources: &FactorSources,
        gateway: Arc<dyn Gateway>,
        derivation_interactors: Arc<dyn DerivationInteractors>,
        is_derivation_done_query: Arc<dyn IsDerivationDoneQuery>,
    ) -> Self {
        Self::new(
            PolyDeriveRequestKind::OARS {
                factor_sources: factor_sources.clone(),
            },
            None,
            OnChainAnalyzer::with_gateway(gateway),
            None,
            derivation_interactors,
            is_derivation_done_query,
        )
    }

    pub fn mars(
        factor_source: &FactorSource,
        gateway: Arc<dyn Gateway>,
        cache: impl Into<Option<Arc<Cache>>>,
        profile: Arc<Profile>,
        derivation_interactors: Arc<dyn DerivationInteractors>,
        is_derivation_done_query: Arc<dyn IsDerivationDoneQuery>,
    ) -> Self {
        Self::new(
            PolyDeriveRequestKind::MARS {
                factor_source: factor_source.clone(),
                network_id: profile.current_network(),
            },
            cache,
            OnChainAnalyzer::with_gateway(gateway),
            ProfileAnalyzer::with_profile(profile),
            derivation_interactors,
            is_derivation_done_query,
        )
    }

    pub fn pre_derive_instance_for_new_factor_source(
        factor_source: &FactorSource,
        gateway: impl Into<Option<Arc<dyn Gateway>>>,
        cache: impl Into<Option<Arc<Cache>>>,
        profile: Arc<Profile>,
        derivation_interactors: Arc<dyn DerivationInteractors>,
    ) -> Self {
        Self::new(
            PolyDeriveRequestKind::PreDeriveInstancesForNewFactorSource {
                factor_source: factor_source.clone(),
            },
            cache,
            OnChainAnalyzer::new(gateway),
            ProfileAnalyzer::with_profile(profile),
            derivation_interactors,
            Arc::new(YesDone),
        )
    }

    pub fn new_virtual_unsecurified_account(
        network_id: NetworkID,
        factor_source: &FactorSource,
        gateway: impl Into<Option<Arc<dyn Gateway>>>,
        cache: impl Into<Option<Arc<Cache>>>,
        profile: Arc<Profile>,
        derivation_interactors: Arc<dyn DerivationInteractors>,
    ) -> Self {
        Self::new(
            PolyDeriveRequestKind::NewVirtualUnsecurifiedAccount {
                network_id,
                factor_source: factor_source.clone(),
            },
            cache,
            OnChainAnalyzer::new(gateway),
            ProfileAnalyzer::with_profile(profile),
            derivation_interactors,
            Arc::new(YesDone),
        )
    }

    /// Securify unsecurified Account
    pub fn securify_unsecurified_account(
        account_address: AccountAddress,
        matrix_of_factor_sources: MatrixOfFactorSources,
        gateway: impl Into<Option<Arc<dyn Gateway>>>,
        cache: impl Into<Option<Arc<Cache>>>,
        profile: Arc<Profile>,
        derivation_interactors: Arc<dyn DerivationInteractors>,
    ) -> Self {
        let unsecurified_account = profile
            .get_account(&account_address)
            .unwrap()
            .as_unsecurified()
            .unwrap()
            .clone();
        //     unsecurified_account: UnsecurifiedAccount,
        //     matrix_of_factor_sources: MatrixOfFactorSources,
        // },

        Self::new(
            PolyDeriveRequestKind::SecurifyUnsecurifiedAccount {
                unsecurified_account,
                matrix_of_factor_sources,
            },
            cache,
            OnChainAnalyzer::new(gateway),
            ProfileAnalyzer::with_profile(profile),
            derivation_interactors,
            Arc::new(YesDone),
        )
    }
}

#[async_trait]
pub trait IsDerivationDoneQuery {
    async fn is_done(&self, derived_accounts: &DerivedFactorInstances) -> Result<bool>;
}

pub struct YesDone;
#[async_trait]
impl IsDerivationDoneQuery for YesDone {
    async fn is_done(&self, derived_accounts: &DerivedFactorInstances) -> Result<bool> {
        Ok(true)
    }
}

impl PolyDerivation {
    async fn is_done(&self, derived_accounts: &DerivedFactorInstances) -> Result<bool> {
        self.is_derivation_done_query
            .is_done(derived_accounts)
            .await
    }

    async fn derive_more(&self) -> Result<()> {
        //    let index_offset = self.cache_with_cursor.try_read().unwrap().
        // let requests = ...;
        // if let Some(cached) = self.cache.load(&requests).await? {
        //     analysis.merge(cached);
        //     return Ok(());
        // }
        todo!()
    }

    fn derived_instances(&self) -> DerivedFactorInstances {
        // self.cache.recovered_accounts()
        todo!()
    }

    pub async fn poly_derive(self) -> Result<FinalDerivationsFinalAndAnalysis> {
        loop {
            let is_done = self.is_done(&&self.derived_instances()).await?;
            if is_done {
                break;
            }
            self.derive_more().await?;
        }

        let derived_instances = self.derived_instances();
        let cache = self.cache;

        let analysis = FinalDerivationsFinalAndAnalysis {
            derived_instances,
            cache,
        };

        Ok(analysis)
    }
}

/// onboarding account recover scan
pub async fn oars(
    factor_sources: FactorSources,
    interactors: Arc<dyn DerivationInteractors>,
    gateway: Arc<dyn Gateway>,
    is_derivation_done_query: Arc<dyn IsDerivationDoneQuery>,
) -> Result<(Profile, Arc<Cache>)> {
    let network_id = NetworkID::Mainnet;

    let derivation = PolyDerivation::oars(
        &factor_sources,
        gateway,
        interactors,
        is_derivation_done_query,
    );

    let analysis = derivation.poly_derive().await?;
    let cache = analysis.cache;

    let recovered_unsecurified_accounts =
        analysis.derived_instances.accounts_unsecurified(network_id);

    // TODO handle securified!
    let profile = Profile::new(factor_sources, recovered_unsecurified_accounts);

    Ok((profile, cache))
}

pub async fn mars(
    factor_source: FactorSource,
    interactors: Arc<dyn DerivationInteractors>,
    gateway: Arc<dyn Gateway>,
    profile: &mut Profile,
    cache: impl Into<Option<Arc<Cache>>>,
    is_derivation_done_query: Arc<dyn IsDerivationDoneQuery>,
) -> Result<Arc<Cache>> {
    let network_id = profile.current_network();
    let derivation = PolyDerivation::mars(
        &factor_source,
        gateway,
        cache,
        Arc::new(profile.clone()),
        interactors,
        is_derivation_done_query,
    );

    let analysis = derivation.poly_derive().await?;
    let cache = analysis.cache;
    let accounts = analysis.derived_instances.accounts_unsecurified(network_id);

    profile.insert_accounts(accounts)?;

    Ok(cache)
}

pub async fn pre_derive_instance_for_new_factor_source(
    factor_source: &FactorSource, // not yet added to Profile.
    gateway: impl Into<Option<Arc<dyn Gateway>>>,
    cache: impl Into<Option<Arc<Cache>>>,
    profile: &mut Profile,
    derivation_interactors: Arc<dyn DerivationInteractors>,
) -> Result<Arc<Cache>> {
    let network_id = profile.current_network();
    let derivation = PolyDerivation::pre_derive_instance_for_new_factor_source(
        factor_source,
        gateway,
        cache,
        Arc::new(profile.clone()),
        derivation_interactors,
    );

    let analysis = derivation.poly_derive().await?;
    let cache = analysis.cache;
    profile.add_factor_source(factor_source.clone())?;

    Ok(cache)
}

pub async fn new_virtual_unsecurified_account(
    name: impl AsRef<str>,
    network_id: NetworkID,
    factor_source: &FactorSource,
    gateway: impl Into<Option<Arc<dyn Gateway>>>,
    cache: impl Into<Option<Arc<Cache>>>,
    profile: &mut Profile,
    derivation_interactors: Arc<dyn DerivationInteractors>,
) -> Result<Account> {
    let network_id = profile.current_network();
    let derivation = PolyDerivation::new_virtual_unsecurified_account(
        network_id,
        factor_source,
        gateway,
        cache,
        Arc::new(profile.clone()),
        derivation_interactors,
    );

    let analysis = derivation.poly_derive().await?;

    let mut account = analysis
        .derived_instances
        .accounts_unsecurified(network_id)
        .first()
        .ok_or_else(|| "No account".to_owned())
        .cloned()?;

    account.set_name(name);

    profile.insert_accounts(IndexSet::from_iter([account.clone()]))?;

    Ok(account)
}

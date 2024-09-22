#![allow(unused)]
#![allow(unused_variables)]

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FactorSourceSelector {
    NewOfKind(FactorSourceKind),
    Specific(FactorSourceIDFromHash),
}

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

    /// Add new FactorSource
    PreDeriveInstancesForNewFactorSource {
        factor_source_kind: FactorSourceKind,
    },
}

/// Offsets to next derivation entity index to use for a given
/// factor source selector.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct FactorInstancesCacheCursors {
    map: IndexMap<FactorSourceSelector, HDPathValue>,
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
}

#[async_trait]
pub trait IsDerivationDoneQuery {
    async fn is_done(&self, derived_accounts: &DerivedAccounts) -> Result<bool>;
}

impl PolyDerivation {
    async fn is_done(&self, derived_accounts: &DerivedAccounts) -> Result<bool> {
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

    fn derived_accounts(&self) -> DerivedAccounts {
        // self.cache.recovered_accounts()
        todo!()
    }

    pub async fn poly_derive(self) -> Result<FinalDerivationsFinalAndAnalysis> {
        loop {
            let is_done = self.is_done(&self.derived_accounts()).await?;
            if is_done {
                break;
            }
            self.derive_more().await?;
        }

        let derived_accounts = self.derived_accounts();
        let cache = self.cache;

        let analysis = FinalDerivationsFinalAndAnalysis {
            derived_accounts,
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
    let derivation = PolyDerivation::oars(
        &factor_sources,
        gateway,
        interactors,
        is_derivation_done_query,
    );

    let analysis = derivation.poly_derive().await?;
    let cache = analysis.cache;
    let accounts = analysis.derived_accounts.all_accounts();
    let profile = Profile::new(factor_sources, accounts);

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
    let accounts = analysis.derived_accounts.all_accounts();

    profile.insert_accounts(accounts)?;

    Ok(cache)
}

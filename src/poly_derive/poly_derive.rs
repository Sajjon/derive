#![allow(unused)]
#![allow(unused_variables)]

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FactorSourceSelector {
    NewOfKind(FactorSourceKind),
    Specific(FactorSourceIDFromHash),
}
impl FactorSourceSelector {
    fn matches_derivation_path(&self, path: DerivationPath) -> bool {
        match self {
            Self::NewOfKind(kind) => path.factor_source_id().factor_source_kind == *kind,
            Self::Specific(id) => path.factor_source_id() == *id,
        }
    }
    fn factor_source_kind(&self) -> FactorSourceKind {
        match self {
            Self::NewOfKind(kind) => kind.clone(),
            Self::Specific(id) => id.factor_source_kind,
        }
    }
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

    /// Add new FactorSource
    PreDeriveInstancesForNewFactorSource {
        factor_source_kind: FactorSourceKind,
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

    pub fn pre_derive_instance_for_new_factor_source(
        factor_source_kind: &FactorSourceKind,
        gateway: impl Into<Option<Arc<dyn Gateway>>>,
        cache: impl Into<Option<Arc<Cache>>>,
        profile: Arc<Profile>,
        derivation_interactors: Arc<dyn DerivationInteractors>,
    ) -> Self {
        Self::new(
            PolyDeriveRequestKind::PreDeriveInstancesForNewFactorSource {
                factor_source_kind: factor_source_kind.clone(),
            },
            cache,
            OnChainAnalyzer::new(gateway),
            ProfileAnalyzer::with_profile(profile),
            derivation_interactors,
            Arc::new(WhenDerivedInstancesForFactorSource {
                factor_source_selector: FactorSourceSelector::NewOfKind(factor_source_kind.clone()),
            }),
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
            Arc::new(WhenMatchingSingle {
                request: DerivationRequestInKeySpace::new(
                    factor_source.factor_source_id.clone(),
                    network_id,
                    CAP26EntityKind::Account,
                    CAP26KeyKind::T9n,
                    KeySpace::Unsecurified,
                ),
            }),
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
            Arc::new(WhenMatchingSingle {
                request: DerivationRequestInKeySpace::new(
                    factor_source.factor_source_id.clone(),
                    network_id,
                    CAP26EntityKind::Account,
                    CAP26KeyKind::T9n,
                    KeySpace::Unsecurified,
                ),
            }),
        )
    }
}

#[async_trait]
pub trait IsDerivationDoneQuery {
    async fn is_done(&self, derived_accounts: &DerivedAccounts) -> Result<bool>;
}
pub struct WhenMatchingSingle {
    pub request: DerivationRequestInKeySpace,
}
#[async_trait]
impl IsDerivationDoneQuery for WhenMatchingSingle {
    async fn is_done(&self, derived_accounts: &DerivedAccounts) -> Result<bool> {
        if derived_accounts
            .unsecurified_accounts
            .iter()
            .any(|a| a.veci.derivation_in_key_space() == self.request)
        {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}

pub struct WhenDerivedInstancesForFactorSource {
    pub factor_source_selector: FactorSourceSelector,
}
#[async_trait]
impl IsDerivationDoneQuery for WhenDerivedInstancesForFactorSource {
    async fn is_done(&self, derived_accounts: &DerivedAccounts) -> Result<bool> {
        let expected_count = self
            .factor_source_selector
            .factor_source_kind()
            .derivation_batch_size();
        if derived_accounts
            .unsecurified_accounts
            .iter()
            .filter(|a| {
                self.factor_source_selector
                    .matches_derivation_path(a.0.derivation_path())
            })
            .collect_vec()
            .len()
            >= expected_count
        {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
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

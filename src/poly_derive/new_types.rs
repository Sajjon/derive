use std::{net, ops::Range};

use __std_iter::Step;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeySpace {
    Unsecurified,
    Securified,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DerivationRequestAbstractFactorAbstractIndex<T, U: KeySpaced> {
    abstract_factor: T,
    pub network_id: NetworkID,
    pub entity_kind: CAP26EntityKind,
    pub key_kind: CAP26KeyKind,
    pub key_space: KeySpace,
    abstract_last_component: U,
}
pub trait KeySpaced {
    fn is_in_key_space(&self, key_space: KeySpace) -> bool;
}
impl<T, U: KeySpaced> DerivationRequestAbstractFactorAbstractIndex<T, U> {
    /// # Panics
    /// Panics if `abstract_last_component` does not match the key space
    fn abstract_abstract_new(
        abstract_factor: T,
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        key_space: KeySpace,
        abstract_last_component: U,
    ) -> Self {
        assert!(abstract_last_component.is_in_key_space(key_space));
        Self {
            abstract_factor,
            network_id,
            entity_kind,
            key_kind,
            key_space,
            abstract_last_component,
        }
    }
}

pub type DerivationPathAbstractIndex<U: KeySpaced> =
    DerivationRequestAbstractFactorAbstractIndex<FactorSourceIDFromHash, U>;

impl<U: KeySpaced> DerivationPathAbstractIndex<U> {
    pub fn factor_source_id(&self) -> FactorSourceIDFromHash {
        self.abstract_factor.clone()
    }

    fn new_with_factor_source_id(
        factor_source_id: FactorSourceIDFromHash,
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        key_space: KeySpace,
        abstract_last_component: U,
    ) -> Self {
        Self::abstract_abstract_new(
            factor_source_id,
            network_id,
            entity_kind,
            key_kind,
            key_space,
            abstract_last_component,
        )
    }
}
pub(crate) const BIP32_HARDENED: u32 = 0x8000_0000;
pub(crate) const BIP32_SECURIFIED_HALF: u32 = 0x4000_0000;
impl KeySpaced for CAP26Index {
    fn is_in_key_space(&self, key_space: KeySpace) -> bool {
        match self {
            CAP26Index::Unsecurified(_) => key_space == KeySpace::Unsecurified,
            CAP26Index::Securified(_) => key_space == KeySpace::Securified,
        }
    }
}
impl KeySpaced for KeySpace {
    fn is_in_key_space(&self, key_space: KeySpace) -> bool {
        key_space == *self
    }
}
impl KeySpaced for Range<CAP26Index> {
    fn is_in_key_space(&self, key_space: KeySpace) -> bool {
        self.clone()
            .collect_vec()
            .into_iter()
            .all(|i| i.is_in_key_space(key_space))
    }
}
pub type DerivationRequestWithRange = DerivationPathAbstractIndex<Range<CAP26Index>>;
pub type DerivationRequestInKeySpace = DerivationPathAbstractIndex<KeySpace>;
impl DerivationRequestInKeySpace {
    pub fn new(
        factor_source_id: FactorSourceIDFromHash,
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        key_space: KeySpace,
    ) -> Self {
        Self::new_with_factor_source_id(
            factor_source_id,
            network_id,
            entity_kind,
            key_kind,
            key_space,
            key_space,
        )
    }
}
impl DerivationRequestWithRange {
    pub fn new(
        factor_source_id: FactorSourceIDFromHash,
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        key_space: KeySpace,
        range: Range<CAP26Index>,
    ) -> Self {
        Self::new_with_factor_source_id(
            factor_source_id,
            network_id,
            entity_kind,
            key_kind,
            key_space,
            range,
        )
    }
}

pub type DerivationPath = DerivationPathAbstractIndex<CAP26Index>;
impl DerivationPath {
    pub fn erase_to_in_key_space(&self) -> DerivationRequestInKeySpace {
        DerivationRequestInKeySpace::new(
            self.factor_source_id(),
            self.network_id,
            self.entity_kind,
            self.key_kind,
            self.key_space(),
        )
    }
}

pub type DerivationRequestWithoutFactorInKeySpace =
    DerivationRequestAbstractFactorAbstractIndex<(), KeySpace>;

impl DerivationRequestWithoutFactorInKeySpace {
    pub fn with_factor_source(self, factor_source: &FactorSource) -> DerivationRequestInKeySpace {
        DerivationRequestInKeySpace::new(
            factor_source.factor_source_id.clone(),
            self.network_id,
            self.entity_kind,
            self.key_kind,
            self.key_space,
        )
    }
}

impl DerivationPath {
    pub fn index(&self) -> CAP26Index {
        self.abstract_last_component.clone()
    }

    pub fn key_space(&self) -> KeySpace {
        assert_eq!(self.index().key_space(), self.key_space);
        self.key_space
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct AbstractDerivationRequests(IndexSet<DerivationRequestWithoutFactorInKeySpace>);
impl AbstractDerivationRequests {
    pub fn for_each_factor_sources(
        &self,
        factor_sources: FactorSources,
    ) -> IndexSet<DerivationRequestInKeySpace> {
        factor_sources
            .factor_sources()
            .iter()
            .flat_map(|f| self.0.clone().into_iter().map(|x| x.with_factor_source(f)))
            .collect()
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorSources(Vec<FactorSource>);
impl FactorSources {
    pub fn from_iter(iter: impl IntoIterator<Item = FactorSource>) -> Self {
        Self(iter.into_iter().collect())
    }
    pub fn factor_sources(&self) -> IndexSet<FactorSource> {
        self.0.clone().into_iter().collect()
    }
    pub fn just(factor_source: FactorSource) -> Self {
        Self(vec![factor_source])
    }
    pub fn insert(&mut self, factor_source: FactorSource) {
        assert!(!self.0.iter().any(|f| f == &factor_source));
        self.0.push(factor_source);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorInstanceInUnsecurifiedSpace {
    factor_instance: FactorInstance,
}
impl From<FactorInstanceInUnsecurifiedSpace> for FactorInstance {
    fn from(value: FactorInstanceInUnsecurifiedSpace) -> Self {
        value.instance()
    }
}
impl FactorInstanceInUnsecurifiedSpace {
    /// # Panics
    /// Panics if it is not in unsecurified space
    pub fn new(factor_instance: FactorInstance) -> Self {
        assert_eq!(factor_instance.key_space(), KeySpace::Unsecurified);
        Self { factor_instance }
    }
    pub fn instance(&self) -> FactorInstance {
        self.factor_instance.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorInstanceInSecurifiedSpace {
    factor_instance: FactorInstance,
}
impl From<FactorInstanceInSecurifiedSpace> for FactorInstance {
    fn from(value: FactorInstanceInSecurifiedSpace) -> Self {
        value.instance()
    }
}
impl FactorInstanceInSecurifiedSpace {
    /// # Panics
    /// Panics if it is not in securified space
    pub fn new(factor_instance: FactorInstance) -> Self {
        assert_eq!(factor_instance.key_space(), KeySpace::Securified);
        Self { factor_instance }
    }
    pub fn instance(&self) -> FactorInstance {
        self.factor_instance.clone()
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct DerivedFactorInstances {
    unsecurified_factor_instances: IndexSet<FactorInstanceInUnsecurifiedSpace>,
    securified_matrices_of_factor_instances: IndexSet<MatrixOfFactorInstances>,
}
impl DerivedFactorInstances {
    pub fn unsecurified_accounts(&self, network_id: NetworkID) -> IndexSet<UnsecurifiedAccount> {
        self.unsecurified_factor_instances()
            .into_iter()
            .map(|fi| UnsecurifiedAccount::new(fi, network_id))
            .collect()
    }
    pub fn accounts_unsecurified(&self, network_id: NetworkID) -> IndexSet<Account> {
        self.unsecurified_accounts(network_id)
            .into_iter()
            .map(Into::into)
            .collect()
    }

    pub fn unsecurified_factor_instances(&self) -> IndexSet<FactorInstanceInUnsecurifiedSpace> {
        self.unsecurified_factor_instances.clone()
    }

    // pub fn account_addresses_of_securified(&self) -> IndexSet<AccountAddress> {
    //     self.securified_factor_instances
    //         .iter()
    //         .map(|f| AccountAddress::new(f.clone(), self.network_id))
    //         .collect()
    // }
    // pub fn all_account_addresses(&self) -> IndexSet<AccountAddress> {
    //     let mut addresses = IndexSet::new();
    //     addresses.extend(self.account_addresses_of_unsecurified());
    //     addresses.extend(self.account_addresses_of_securified());
    //     addresses
    // }
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
impl FromIterator<FactorInstance> for FactorInstances {
    fn from_iter<T: IntoIterator<Item = FactorInstance>>(iter: T) -> Self {
        Self::from(iter)
    }
}

#[derive(Default, Clone)]
pub struct OnChainAnalyzer {
    gateway: Option<Arc<dyn Gateway>>,
}
impl OnChainAnalyzer {
    pub fn new(gateway: impl Into<Option<Arc<dyn Gateway>>>) -> Self {
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

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct CachedFactorInstances(pub IndexMap<DerivationRequestInKeySpace, FactorInstances>);

#[derive(Debug, Default)]
pub struct Cache {
    factor_instances_for_requests: RwLock<CachedFactorInstances>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheLoadOutcome {
    pub requests: IndexSet<DerivationRequestInKeySpace>,
    /// Response to `requests`
    pub factor_instances: CachedFactorInstances,
    /// If `factor_instances` response satisfies all `requests`
    pub is_satisfying_all_requests: bool,
    /// If we should derive more factor instances after this response, either
    /// because we consumed the last factor instance in the cache or because
    /// we did not fully satisfy all requests.
    pub should_derive_more: bool,
}
impl CacheLoadOutcome {
    pub fn is_empty(&self) -> bool {
        self.factor_instances.0.is_empty()
    }
}

impl Cache {
    fn get(&self, key: &DerivationRequestInKeySpace) -> Option<FactorInstances> {
        self.factor_instances_for_requests
            .try_read()
            .unwrap()
            .0
            .get(key)
            .cloned()
    }

    pub async fn load(
        &self,
        requests: IndexSet<DerivationRequestInKeySpace>,
    ) -> Result<CacheLoadOutcome> {
        let mut found = CachedFactorInstances::default();
        let mut failure = false;
        for key in requests.iter() {
            let Some(loaded) = self.get(key) else {
                failure = true;
                continue;
            };
            found.0.insert(key.clone(), loaded);
        }

        Ok(CacheLoadOutcome {
            requests,
            factor_instances: found,
            is_satisfying_all_requests: !failure,
            should_derive_more: failure,
        })
    }
}
impl Cache {
    fn with_map(map: IndexMap<DerivationRequestInKeySpace, FactorInstances>) -> Self {
        Self {
            factor_instances_for_requests: RwLock::new(CachedFactorInstances(map)),
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntermediaryDerivationsAndAnalysis {
    pub derived_instances: DerivedFactorInstances,
    pub probably_free: ProbablyFreeFactorInstances,
}

#[derive(Clone, Debug)]
pub struct FinalDerivationsFinalAndAnalysis {
    pub derived_instances: DerivedFactorInstances,
    pub cache: Arc<Cache>,
}

pub type HDPathValue = u32;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd)]
pub enum CAP26Index {
    Unsecurified(HDPathValue),
    Securified(HDPathValue),
}
impl Step for CAP26Index {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        todo!()
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        todo!()
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        todo!()
    }
}
impl CAP26Index {
    pub fn new(base_index: HDPathValue) -> Self {
        if base_index <= (BIP32_HARDENED + BIP32_SECURIFIED_HALF) {
            Self::Unsecurified(base_index)
        } else {
            Self::Securified(base_index)
        }
    }
    pub fn base_index(&self) -> HDPathValue {
        match self {
            CAP26Index::Unsecurified(v) => *v,
            CAP26Index::Securified(v) => *v,
        }
    }
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
pub struct UnsecurifiedAccount {
    pub address: AccountAddress,
    pub veci: FactorInstance,
}
impl From<UnsecurifiedAccount> for Account {
    fn from(value: UnsecurifiedAccount) -> Self {
        Account::Unsecurified(value)
    }
}
impl UnsecurifiedAccount {
    pub fn new(veci: FactorInstanceInUnsecurifiedSpace, network_id: NetworkID) -> Self {
        Self {
            address: AccountAddress::new(veci.instance(), network_id),
            veci: veci.instance(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SecurifiedAccount {
    pub address: AccountAddress,
    pub veci: Option<FactorInstance>,
    pub matrix: MatrixOfFactorInstances,
}

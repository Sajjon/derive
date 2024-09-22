use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeySpace {
    Unsecurified,
    Securified,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DerivationRequestAbstractFactorAbstractIndex<T, U> {
    abstract_factor: T,
    pub network_id: NetworkID,
    pub entity_kind: CAP26EntityKind,
    pub key_kind: CAP26KeyKind,
    abstract_index: U,
}
impl<T, U> DerivationRequestAbstractFactorAbstractIndex<T, U> {
    fn abstract_abstract_new(
        abstract_factor: T,
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        abstract_index: U,
    ) -> Self {
        Self {
            abstract_factor,
            network_id,
            entity_kind,
            key_kind,
            abstract_index,
        }
    }
}

pub type DerivationPathAbstractIndex<U> =
    DerivationRequestAbstractFactorAbstractIndex<FactorSourceIDFromHash, U>;
impl<T> DerivationPathAbstractIndex<T> {
    pub fn factor_source_id(&self) -> FactorSourceIDFromHash {
        self.abstract_factor.clone()
    }

    fn new_with_factor_source_id(
        factor_source_id: FactorSourceIDFromHash,
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        abstract_index: T,
    ) -> Self {
        Self::abstract_abstract_new(
            factor_source_id,
            network_id,
            entity_kind,
            key_kind,
            abstract_index,
        )
    }
}

pub type DerivationRequestInKeySpace = DerivationPathAbstractIndex<KeySpace>;
impl DerivationRequestInKeySpace {
    pub fn key_space(&self) -> KeySpace {
        self.abstract_index
    }
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
    pub fn key_space(&self) -> KeySpace {
        self.abstract_index.clone()
    }
}

impl DerivationPath {
    pub fn index(&self) -> CAP26Index {
        self.abstract_index.clone()
    }

    pub fn key_space(&self) -> KeySpace {
        self.index().key_space()
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct AbstractDerivationRequests(IndexSet<DerivationRequestWithoutFactorInKeySpace>);

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorSources(Vec<FactorSource>);
impl FactorSources {
    pub fn factor_sources(&self) -> IndexSet<FactorSource> {
        self.0.clone().into_iter().collect()
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DerivedFactorInstances {
    pub network_id: NetworkID,
    unsecurified_factor_instances: IndexSet<FactorInstanceInUnsecurifiedSpace>,
    securified_factor_instances: IndexSet<FactorInstanceInSecurifiedSpace>,
}
impl DerivedFactorInstances {
    pub fn unsecurified_factor_instances(&self) -> IndexSet<FactorInstanceInUnsecurifiedSpace> {
        self.unsecurified_factor_instances.clone()
    }

    pub fn account_addresses_of_unsecurified(&self) -> IndexSet<AccountAddress> {
        self.unsecurified_factor_instances
            .iter()
            .map(|f| AccountAddress::new(f.clone(), self.network_id))
            .collect()
    }
    pub fn account_addresses_of_securified(&self) -> IndexSet<AccountAddress> {
        self.securified_factor_instances
            .iter()
            .map(|f| AccountAddress::new(f.clone(), self.network_id))
            .collect()
    }
    pub fn all_account_addresses(&self) -> IndexSet<AccountAddress> {
        let mut addresses = IndexSet::new();
        addresses.extend(self.account_addresses_of_unsecurified());
        addresses.extend(self.account_addresses_of_securified());
        addresses
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

#[derive(Debug, Default)]
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
    pub derived_instances: DerivedFactorInstances,
    pub probably_free: ProbablyFreeFactorInstances,
}

#[derive(Clone, Debug)]
pub struct FinalDerivationsFinalAndAnalysis {
    pub derived_accounts: DerivedAccounts,
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
pub struct UnsecurifiedAccount {
    pub address: AccountAddress,
    pub veci: FactorInstance,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SecurifiedAccount {
    pub address: AccountAddress,
    pub veci: Option<FactorInstance>,
    pub matrix: MatrixOfFactorInstances,
}

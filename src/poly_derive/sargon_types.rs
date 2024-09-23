use enum_as_inner::EnumAsInner;

use crate::prelude::*;

pub type Result<T, E = String> = std::result::Result<T, E>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorSource {
    pub factor_source_id: FactorSourceIDFromHash,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FactorSourceKind {
    Device,
    Ledger,
}
impl FactorSourceKind {
    pub fn derivation_batch_size(&self) -> usize {
        match self {
            Self::Device => 20,
            Self::Ledger => 10,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PublicKey {
    pub bytes: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PublicKeyHash {
    bytes: [u8; 29],
}
impl PublicKeyHash {
    fn hashing(public_key: PublicKey) -> Self {
        use sha256::digest;
        let digest_hex = digest(&public_key.bytes);
        let digest = hex::decode(digest_hex).unwrap();
        Self {
            bytes: digest.as_slice().try_into().unwrap(),
        }
    }
    pub fn new(factor_instance: impl Into<FactorInstance>) -> Self {
        let factor_instance = factor_instance.into();
        Self::hashing(factor_instance.public_key())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccountAddress {
    pub network_id: NetworkID,
    pub public_key_hash: PublicKeyHash,
}
impl AccountAddress {
    pub fn new(factor_instance: impl Into<FactorInstance>, network_id: NetworkID) -> Self {
        let factor_instance = factor_instance.into();
        Self {
            network_id,
            public_key_hash: factor_instance.factor_source_id.public_key_hash,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorSourceIDFromHash {
    /// Hash at special node.
    pub public_key_hash: PublicKeyHash,
    pub factor_source_kind: FactorSourceKind,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorInstance {
    derivation_path: DerivationPath,
    public_key: PublicKey,
    factor_source_id: FactorSourceIDFromHash,
}
impl FactorInstance {
    pub fn public_key(&self) -> PublicKey {
        self.public_key.clone()
    }
    pub fn derivation_path(&self) -> DerivationPath {
        self.derivation_path.clone()
    }
    pub fn derivation_in_key_space(&self) -> DerivationRequestInKeySpace {
        self.derivation_path.erase_to_in_key_space()
    }
    pub fn key_space(&self) -> KeySpace {
        self.derivation_in_key_space().key_space
    }
    pub fn factor_source_id(&self) -> FactorSourceIDFromHash {
        self.factor_source_id.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MatrixOfAbstractFactor<T> {
    threshold_factors: Vec<T>,
    threshold: usize,
    override_factors: Vec<T>,
}
pub type MatrixOfFactorSources = MatrixOfAbstractFactor<FactorSource>;
impl MatrixOfFactorSources {
    pub fn all_factor_sources(&self) -> FactorSources {
        let mut set = IndexSet::new();
        set.extend(self.threshold_factors.iter().cloned());
        set.extend(self.override_factors.iter().cloned());
        FactorSources::from_iter(set)
    }
}
pub type MatrixOfFactorInstances = MatrixOfAbstractFactor<FactorInstanceInSecurifiedSpace>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NetworkID {
    Mainnet,
    Testnet,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CAP26KeyKind {
    T9n,
    Rola,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CAP26EntityKind {
    Account,
    Identity,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, EnumAsInner)]
pub enum Account {
    Unsecurified(UnsecurifiedAccount),
    Securified(SecurifiedAccount),
}
impl Account {
    pub fn new_unsecurified(
        instance: FactorInstanceInUnsecurifiedSpace,
        network_id: NetworkID,
    ) -> Self {
        Self::Unsecurified(UnsecurifiedAccount::new(instance, network_id))
    }
    pub fn set_name(&mut self, _name: impl AsRef<str>) {
        // noop
    }
    pub fn address(&self) -> AccountAddress {
        match self {
            Self::Unsecurified(a) => a.address.clone(),
            Self::Securified(a) => a.address.clone(),
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Profile {
    pub factor_sources: FactorSources,
    pub accounts: IndexSet<Account>,
}
impl Profile {
    pub fn new(factor_sources: FactorSources, accounts: IndexSet<Account>) -> Self {
        Self {
            factor_sources,
            accounts,
        }
    }

    pub fn current_network(&self) -> NetworkID {
        NetworkID::Mainnet
    }

    pub fn get_account(&self, address: &AccountAddress) -> Result<Account> {
        self.accounts
            .iter()
            .find(|a| a.address() == *address)
            .cloned()
            .ok_or("Account not found".to_owned())
    }

    pub fn insert_accounts(&mut self, accounts: IndexSet<Account>) -> Result<()> {
        let count = self.accounts.len();
        let expected_after_insertion = count + accounts.len();
        self.accounts.extend(accounts);
        assert_eq!(self.accounts.len(), expected_after_insertion);
        Ok(())
    }

    pub fn add_factor_source(&mut self, factor_source: FactorSource) -> Result<()> {
        self.factor_sources.insert(factor_source);
        Ok(())
    }
}

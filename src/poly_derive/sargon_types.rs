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

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorSourceIDFromHash(pub Uuid);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorInstance {
    derivation_path: DerivationPath,
    factor_source_id: FactorSourceIDFromHash,
}
impl FactorInstance {
    pub fn derivation_path(&self) -> DerivationPath {
        self.derivation_path.clone()
    }
    pub fn derivation_in_key_space(&self) -> DerivationRequestInKeySpace {
        self.derivation_path.erase_to_in_key_space()
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
pub type MatrixOfFactorInstances = MatrixOfAbstractFactor<FactorInstance>;

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Account {
    Unsecurified(UnsecurifiedAccount),
    Securified(SecurifiedAccount),
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
    pub fn insert_accounts(&mut self, accounts: IndexSet<Account>) -> Result<()> {
        let count = self.accounts.len();
        let expected_after_insertion = count + accounts.len();
        self.accounts.extend(accounts);
        assert_eq!(self.accounts.len(), expected_after_insertion);
        Ok(())
    }
}

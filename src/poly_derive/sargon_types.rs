use crate::prelude::*;

pub type Result<T, E = String> = std::result::Result<T, E>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorSource {
    pub factor_source_id: FactorSourceIDFromHash,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorSourceIDFromHash(pub uuid::Uuid);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorInstance {
    derivation_path: DerivationPath,
    factor_source_id: FactorSourceIDFromHash,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NetworkID {
    Mainnet,
    Testnet,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CAP26KeyKind {
    T9n,
    Rola,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
}

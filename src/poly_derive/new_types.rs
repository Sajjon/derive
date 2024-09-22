use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

pub type DerivationPathAbstractIndex<U> =
    DerivationRequestAbstractFactorAbstractIndex<FactorSourceIDFromHash, U>;

impl<T> DerivationPathAbstractIndex<T> {
    pub fn factor_source_id(&self) -> FactorSourceIDFromHash {
        self.abstract_factor.clone()
    }
}

pub type DerivationRequestInKeySpace = DerivationPathAbstractIndex<KeySpace>;
pub type DerivationPath = DerivationPathAbstractIndex<CAP26Index>;

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

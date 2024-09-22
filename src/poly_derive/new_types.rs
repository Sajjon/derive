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
    fn new(
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

mod poly_derive;

pub mod prelude {

    pub use crate::poly_derive::*;

    pub use indexmap::{IndexMap, IndexSet};
    pub use itertools::*;
    pub use std::collections::HashMap;
    pub use std::{
        ops::Index,
        sync::{Arc, RwLock},
    };
}

pub use prelude::*;

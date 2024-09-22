mod poly_derive;

pub mod prelude {

    pub use crate::poly_derive::*;

    pub use indexmap::{IndexMap, IndexSet};
    pub use std::{
        ops::Index,
        sync::{Arc, Once},
    };
}

pub use prelude::*;

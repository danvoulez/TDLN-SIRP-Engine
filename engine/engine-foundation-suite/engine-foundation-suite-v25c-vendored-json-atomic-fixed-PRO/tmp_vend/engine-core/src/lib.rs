pub mod model;
pub mod providers;
pub mod runtime;

/// Public alias to avoid product/domain jargon.
pub use crate::model::SemanticChip as AtomicUnit;

/// Domain-neutral alias
pub type AtomicUnit = crate::model::SemanticChip;

pub mod json_atomic;

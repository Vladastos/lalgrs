//! Matrix and Vector Operations library in rust

use thiserror::Error;

pub mod matrix;
pub mod vector;

pub mod prelude {
    pub use crate::matrix::Matrix;
    pub use crate::vector::Vector;
}

#[derive(Debug, Clone, Error)]
pub enum LalgrsError {
    #[error("Mismatched vector dimensions. Found {vector1} and {vector2}")]
    MismatchedVectorDimensions { vector1: usize, vector2: usize },
}

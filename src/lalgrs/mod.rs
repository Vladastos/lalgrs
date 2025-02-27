use thiserror::Error;
pub mod structs;

#[derive(Debug, Clone, Error)]
pub enum LalgrsError {
    #[error("Mismatched vector dimensions. Found {vector1} and {vector2}")]
    MismatchedVectorDimensions { vector1: usize, vector2: usize },

    #[error("Mismatched vector and matrix dimensions. Vector has {vector_size} elements and matrix has {matrix_columns} columns")]
    MismatchedVectorAndMatrixDimensions {
        vector_size: usize,
        matrix_columns: usize,
    },

    #[error("Mismatched matrix sizes. Left hand side matrix columns :{first_matrix_columns},  Right hand side matrix rows:{second_matrix_rows}")]
    MismatchedMatrixDimensions {
        first_matrix_columns: usize,
        second_matrix_rows: usize,
    },

    #[error("Could not create matrix. All columns must have the same length")]
    InvalidMatrixDimensions,
}

use std::ops;

use crate::lalgrs::LalgrsError;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Base struct definitions
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub struct LalgrsVector {
    pub values: Vec<f64>,
}

impl LalgrsVector {
    pub fn new(values: Vec<f64>) -> LalgrsVector {
        LalgrsVector { values }
    }
    pub fn size(&self) -> usize {
        self.values.len()
    }
}

#[derive(Debug, Clone)]
pub struct LalgrsMatrix {
    columns: Vec<Vec<f64>>,
}

impl LalgrsMatrix {
    pub fn new(columns: Vec<Vec<f64>>) -> Result<LalgrsMatrix, LalgrsError> {
        let are_all_vectors_the_same_size = columns.iter().all(|v| v.len() == columns[0].len());
        if !are_all_vectors_the_same_size {
            return Err(LalgrsError::InvalidMatrixDimensions);
        }

        Ok(LalgrsMatrix { columns })
    }
    pub fn rows(&self) -> usize {
        return self.columns.len();
    }
    pub fn columns(&self) -> usize {
        return self.columns[0].len();
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// # Operations
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// ## Addition between two vectors.
/// If one of the two vectors is empty, returns the other one.
/// If the two vectors have different lengths and none of them are empty, returns an error.
/// Otherwise returns a new vector which is the sum of the two operands.
impl ops::Add<LalgrsVector> for LalgrsVector {
    type Output = Result<LalgrsVector, LalgrsError>;
    fn add(self, rhs: LalgrsVector) -> Self::Output {
        if self.size() == 0 {
            return Ok(rhs);
        }

        if rhs.size() == 0 {
            return Ok(self);
        }

        if self.size() != rhs.size() {
            return Err(LalgrsError::MismatchedVectorDimensions {
                vector1: self.size(),
                vector2: rhs.size(),
            });
        };

        return Ok(LalgrsVector::new(
            self.values
                .iter()
                .zip(rhs.values)
                .map(|e| -> f64 { e.0 + e.1 })
                .collect(),
        ));
    }
}

/// ## Negation of a vector. Simply multiplies each element by -1
impl ops::Neg for LalgrsVector {
    type Output = LalgrsVector;
    fn neg(self) -> Self::Output {
        return LalgrsVector::new(self.values.iter().map(|f| -> f64 { -f }).collect());
    }
}

/// ## Subtraction between two vectors.
/// Performs addition with the negative of the second operand
impl ops::Sub<LalgrsVector> for LalgrsVector {
    type Output = Result<LalgrsVector, LalgrsError>;
    fn sub(self, rhs: LalgrsVector) -> Self::Output {
        return self + (-rhs);
    }
}

/// ## Multiplication between a vector and a scalar
/// Multiplies each element of the vector by the scalar
impl ops::Mul<f64> for LalgrsVector {
    type Output = LalgrsVector;
    fn mul(self, rhs: f64) -> Self::Output {
        LalgrsVector::new(self.values.iter().map(|v| -> f64 { v * rhs }).collect())
    }
}

/// ## Multiplication between a matrix and a vector
/// If the number of columns in the matrix does not match the number of elements in the vector, returns an error
/// Otherwise returns a new vector which is the result of the multiplication
impl ops::Mul<LalgrsVector> for LalgrsMatrix {
    type Output = Result<LalgrsVector, LalgrsError>;
    fn mul(self, rhs: LalgrsVector) -> Self::Output {
        if self.columns() != rhs.size() {
            return Err(LalgrsError::MismatchedVectorAndMatrixDimensions {
                vector_size: rhs.size(),
                matrix_columns: self.columns(),
            });
        }

        let result = self
            .columns
            .iter()
            // Map each column to a LalgrsVector
            .map(|col| -> LalgrsVector { return LalgrsVector::new(col.to_vec()) })
            // Zip each LalgrsVector with the corresponding vector element
            .zip(rhs.clone().values)
            // Multiply each LalgrsVector with the corresponding vector element
            .map(|f| -> LalgrsVector { f.0 * f.1 })
            // Sum all LalgrsVectors
            .reduce(|res, element| -> LalgrsVector { (res + element).unwrap() });

        return Ok(result.unwrap_or(LalgrsVector::new(vec![])));
    }
}

/// ## Multiplication between two matrices
/// If the number of columns in the first matrix does not match the number of rows in the second matrix, returns an error
/// Otherwise returns a new matrix which is the result of the multiplication
impl ops::Mul<LalgrsMatrix> for LalgrsMatrix {
    type Output = Result<LalgrsMatrix, LalgrsError>;
    fn mul(self, rhs: LalgrsMatrix) -> Self::Output {
        todo!();
        return Ok(self);
    }
}

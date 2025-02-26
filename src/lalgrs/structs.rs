use std::ops::{self, Add, Mul, Neg};

use crate::lalgrs::LalgrsError;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Base struct definitions
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub struct LalgrsVector<T: Add<T, Output = T>> {
    pub values: Vec<T>,
}

impl<T: Add<T, Output = T>> LalgrsVector<T> {
    pub fn new(values: Vec<T>) -> LalgrsVector<T> {
        LalgrsVector { values }
    }
    pub fn size(&self) -> usize {
        self.values.len()
    }
}

#[derive(Debug, Clone)]
pub struct LalgrsMatrix<T> {
    columns: Vec<Vec<T>>,
}

impl<T> LalgrsMatrix<T> {
    pub fn new(columns: Vec<Vec<T>>) -> Result<LalgrsMatrix<T>, LalgrsError> {
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
impl<T: Add<T, Output = T>> ops::Add<LalgrsVector<T>> for LalgrsVector<T>
where
    for<'a> &'a T: Add<T, Output = T>,
{
    type Output = Result<LalgrsVector<T>, LalgrsError>;
    fn add(self, rhs: LalgrsVector<T>) -> Self::Output {
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
                .map(|e| -> T { e.0 + e.1 })
                .collect(),
        ));
    }
}

/// ## Negation of a vector. Simply multiplies each element by -1
impl<T: Add<T, Output = T>> ops::Neg for LalgrsVector<T>
where
    for<'a> &'a T: Neg<Output = T>,
{
    type Output = LalgrsVector<T>;
    fn neg(self) -> Self::Output {
        return LalgrsVector::new(self.values.iter().map(|f| -> T { -f }).collect());
    }
}

/// ## Subtraction between two vectors.
/// Performs addition with the negative of the second operand
impl<T: Add<T, Output = T> + Neg> ops::Sub<LalgrsVector<T>> for LalgrsVector<T>
where
    for<'a> &'a T: Add<T, Output = T>,
    for<'a> &'a T: Neg<Output = T>,
{
    type Output = Result<LalgrsVector<T>, LalgrsError>;
    fn sub(self, rhs: LalgrsVector<T>) -> Self::Output {
        return self + (-rhs);
    }
}

/// ## Multiplication between a vector and a scalar
/// Multiplies each element of the vector by the scalar
impl<T: Add<T, Output = T> + Mul<T, Output = T> + Clone> ops::Mul<T> for LalgrsVector<T>
where
    for<'a> &'a T: Mul<T, Output = T>,
{
    type Output = LalgrsVector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        LalgrsVector::new(
            self.values
                .iter()
                .map(|v| -> T { v * rhs.clone() })
                .collect(),
        )
    }
}

/// ## Multiplication between a matrix and a vector
/// If the number of columns in the matrix does not match the number of elements in the vector, returns an error
/// Otherwise returns a new vector which is the result of the multiplication
impl<T: Add<T, Output = T> + Clone + Mul<T, Output = T>> ops::Mul<LalgrsVector<T>>
    for LalgrsMatrix<T>
where
    for<'a> &'a T: Add<T, Output = T>,
    for<'a> &'a T: Mul<T, Output = T>,
{
    type Output = Result<LalgrsVector<T>, LalgrsError>;
    fn mul(self, rhs: LalgrsVector<T>) -> Self::Output {
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
            .map(|col| -> LalgrsVector<T> { return LalgrsVector::new(col.to_vec()) })
            // Zip each LalgrsVector with the corresponding vector element
            .zip(rhs.clone().values)
            // Multiply each LalgrsVector with the corresponding vector element
            .map(|f| -> LalgrsVector<T> { f.0 * f.1 })
            // Sum all LalgrsVectors
            .reduce(|res, element| -> LalgrsVector<T> { (res + element).unwrap() });

        return Ok(result.unwrap_or(LalgrsVector::new(vec![])));
    }
}

/// ## Multiplication between two matrices
/// If the number of columns in the first matrix does not match the number of rows in the second matrix, returns an error
/// Otherwise returns a new matrix which is the result of the multiplication
impl<T> ops::Mul<LalgrsMatrix<T>> for LalgrsMatrix<T> {
    type Output = Result<LalgrsMatrix<T>, LalgrsError>;
    fn mul(self, rhs: LalgrsMatrix<T>) -> Self::Output {
        todo!();
        return Ok(self);
    }
}

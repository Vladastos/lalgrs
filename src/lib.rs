use std::ops::{self, Add, Mul, Neg};

use thiserror::Error;

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
impl<T: Add<T, Output = T>> Iterator for LalgrsVector<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.values.pop()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LalgrsMatrix<T: Add<T, Output = T>> {
    columns: Vec<LalgrsVector<T>>,
}

impl<T: Add<T, Output = T> + Clone> LalgrsMatrix<T> {
    pub fn new(columns: Vec<Vec<T>>) -> Result<LalgrsMatrix<T>, LalgrsError> {
        let are_all_vectors_the_same_size = columns.iter().all(|v| v.len() == columns[0].len());
        if !are_all_vectors_the_same_size {
            return Err(LalgrsError::InvalidMatrixDimensions);
        }

        Ok(LalgrsMatrix {
            columns: columns
                .iter()
                .map(|v| LalgrsVector::new(v.clone()))
                .collect(),
        })
    }
    pub fn rows(&self) -> usize {
        return self.columns.len();
    }
    pub fn columns(&self) -> usize {
        return self.columns[0].size();
    }
}

impl<T: Add<T, Output = T>> TryFrom<Vec<LalgrsVector<T>>> for LalgrsMatrix<T> {
    type Error = LalgrsError;
    fn try_from(value: Vec<LalgrsVector<T>>) -> Result<Self, Self::Error> {
        let are_all_vectors_the_same_size = value.iter().all(|v| v.size() == value[0].size());
        if !are_all_vectors_the_same_size {
            return Err(LalgrsError::InvalidMatrixDimensions);
        }
        Ok(LalgrsMatrix { columns: value })
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// # Operations
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// ## Addition between two vectors.
/// If one of the two vectors is empty, returns the other one.
/// If the two vectors have different lengths and none of them are empty, returns an error.
/// Otherwise returns a new vector which is the sum of the two operands.
impl<T: Add<T, Output = T> + Clone> ops::Add<LalgrsVector<T>> for LalgrsVector<T> {
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
                .map(|e| -> T { e.0.to_owned() + e.1.to_owned() })
                .collect(),
        ));
    }
}

/// ## Negation of a vector. Simply multiplies each element by -1
impl<T: Add<T, Output = T> + Neg<Output = T> + Clone> ops::Neg for LalgrsVector<T> {
    type Output = LalgrsVector<T>;
    fn neg(self) -> Self::Output {
        return LalgrsVector::new(
            self.values
                .iter()
                .map(|f| -> T {
                    return -f.to_owned();
                })
                .collect(),
        );
    }
}

/// ## Subtraction between two vectors.
/// Performs addition with the negative of the second operand
impl<T: Add<T, Output = T> + Neg<Output = T> + Clone> ops::Sub<LalgrsVector<T>>
    for LalgrsVector<T>
{
    type Output = Result<LalgrsVector<T>, LalgrsError>;
    fn sub(self, rhs: LalgrsVector<T>) -> Self::Output {
        return self + -rhs;
    }
}

/// ## Multiplication between a vector and a scalar
/// Multiplies each element of the vector by the scalar
impl<T: Add<T, Output = T> + Mul<T, Output = T> + Clone> ops::Mul<T> for LalgrsVector<T> {
    type Output = LalgrsVector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        LalgrsVector::new(
            self.values
                .iter()
                .map(|v| -> T { v.to_owned() * rhs.clone() })
                .collect(),
        )
    }
}

/// ## Multiplication between a matrix and a vector
/// If the number of columns in the matrix does not match the number of elements in the vector, returns an error
/// Otherwise returns a new vector which is the result of the multiplication
impl<T: Add<T, Output = T> + Clone + Mul<T, Output = T>> ops::Mul<LalgrsVector<T>>
    for LalgrsMatrix<T>
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
            // Zip each LalgrsVector with the corresponding vector element
            .zip(rhs.clone().values)
            // Multiply each LalgrsVector with the corresponding vector element
            .map(|f| -> LalgrsVector<T> { f.0.to_owned() * f.1.to_owned() })
            // Sum all LalgrsVectors
            .reduce(|res, element| -> LalgrsVector<T> { (res + element).unwrap() });

        return Ok(result.unwrap_or(LalgrsVector::new(vec![])));
    }
}

/// ## Addition between two matrices
/// If the number of columns in the first matrix does not match the number of columns in the second matrix, returns an error
/// Otherwise returns a new matrix which is the result of the addition
impl<T: Add<T, Output = T> + Clone> ops::Add<LalgrsMatrix<T>> for LalgrsMatrix<T> {
    type Output = Result<LalgrsMatrix<T>, LalgrsError>;
    fn add(self, rhs: LalgrsMatrix<T>) -> Self::Output {
        if self.columns() != rhs.columns() {
            return Err(LalgrsError::MismatchedMatrixDimensions {
                first_matrix_columns: self.columns(),
                second_matrix_rows: rhs.columns(),
            });
        }
        return Ok(LalgrsMatrix {
            columns: self
                .columns
                .iter()
                .zip(rhs.columns.iter())
                .map(|f| -> LalgrsVector<T> { (f.0.to_owned() + f.1.to_owned()).unwrap() })
                .collect(),
        });
    }
}

/// ## Negation of a matrix. Negates each column
impl<T: Add<T, Output = T> + Neg<Output = T> + Clone> ops::Neg for LalgrsMatrix<T> {
    type Output = LalgrsMatrix<T>;
    fn neg(self) -> Self::Output {
        let cols: Vec<LalgrsVector<T>> = self
            .columns
            .iter()
            .map(|f| -> LalgrsVector<T> { -f.to_owned() })
            .collect();
        return LalgrsMatrix::try_from(cols).unwrap();
    }
}

/// ## Subtraction between two matrices
/// Performs addition with the negative of the second operand
impl<T: Add<T, Output = T> + Neg<Output = T> + Clone> ops::Sub<LalgrsMatrix<T>>
    for LalgrsMatrix<T>
{
    type Output = Result<LalgrsMatrix<T>, LalgrsError>;
    fn sub(self, rhs: LalgrsMatrix<T>) -> Self::Output {
        return self + -rhs;
    }
}

/// ## Multiplication between a matrix and a scalar
/// Multiplies each element of the matrix by the scalar
impl<T: Add<T, Output = T> + Mul<T, Output = T> + Clone> ops::Mul<T> for LalgrsMatrix<T> {
    type Output = LalgrsMatrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        let cols: Vec<LalgrsVector<T>> = self
            .columns
            .iter()
            .map(|f| -> LalgrsVector<T> { f.to_owned() * rhs.clone() })
            .collect();
        LalgrsMatrix::try_from(cols).unwrap()
    }
}

/// ## Multiplication between two matrices
/// If the number of columns in the first matrix does not match the number of rows in the second matrix, returns an error
/// Otherwise returns a new matrix which is the result of the multiplication
impl<T: Add<T, Output = T> + Clone + Mul<T, Output = T>> ops::Mul<LalgrsMatrix<T>>
    for LalgrsMatrix<T>
{
    //! FIXME: This function clones too much
    type Output = Result<LalgrsMatrix<T>, LalgrsError>;
    fn mul(self, rhs: LalgrsMatrix<T>) -> Self::Output {
        if self.columns() != rhs.rows() {
            return Err(LalgrsError::MismatchedMatrixDimensions {
                first_matrix_columns: self.columns(),
                second_matrix_rows: rhs.rows(),
            });
        }

        let result_matrix: Vec<LalgrsVector<T>> = rhs
            .columns
            .iter()
            // For each column of the rhs operand
            .map(|col| -> LalgrsVector<T> {
                return col
                    .clone()
                    .into_iter()
                    // Iterate over the elements of the column
                    .zip(self.columns.clone())
                    // Zip the elements of the column with the columns of the lhs operand (each element of the column will be coupled with a column of the lhs operand)
                    .map(|tuple| -> LalgrsVector<T> {
                        // Multiply each element of the column with the corresponding column of the lhs operand
                        return tuple.1 * tuple.0;
                    })
                    .reduce(|acc, element| -> LalgrsVector<T> {
                        // Sum all LalgrsVectors. This produces a new LalgrsVector which is the result of multiplying a column of the rhs operand with a row of the lhs operand
                        (acc + element).unwrap()
                    })
                    .unwrap();
            })
            .collect();
        return Ok(LalgrsMatrix::try_from(result_matrix)?);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// # Errors
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Error, PartialEq)]
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

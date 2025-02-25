use std::ops;

use crate::LalgrsError;

#[derive(Debug, Clone)]
pub struct Vector {
    pub values: Vec<f64>,
}

impl Vector {
    pub fn new(values: Vec<f64>) -> Vector {
        Vector { values }
    }

    pub fn size(&self) -> usize {
        self.values.len()
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Result<Vector, LalgrsError>;

    /// Addition between two vectors.
    /// If one of the two vectors is empty, returns the other one.
    /// If the two vectors have different lengths and none of them are empty, returns an error.
    /// Otherwise returns a new vector which is the sum of the two operands.
    fn add(self, rhs: Vector) -> Self::Output {
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

        return Ok(Vector::new(
            self.values
                .iter()
                .zip(rhs.values)
                .map(|e| -> f64 { e.0 + e.1 })
                .collect(),
        ));
    }
}

pub mod lalgrs;
use lalgrs::{
    structs::{LalgrsMatrix, LalgrsVector},
    LalgrsError,
};
fn main() {
    run().unwrap_or_else(|res| println!("{}", res));
}

fn run() -> Result<(), LalgrsError> {
    let vec1 = LalgrsVector::new(vec![1, 2]);

    let matrix1 = LalgrsMatrix::new(vec![vec![0, 1], vec![2, 0]])?;

    let matrix2 = LalgrsMatrix::new(vec![vec![1, 1], vec![-2, 0]])?;

    let matrix_vector_mul_res = (matrix1.clone() * vec1)?;

    let matrix_mul_res = (matrix1 * matrix2)?;

    println!("Matrix vector multiplication result: {matrix_vector_mul_res:?}");

    println!("Matrix multiplication result: {matrix_mul_res:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::lalgrs::structs::{LalgrsMatrix, LalgrsVector};

    #[test]
    fn test_vector_operations() {
        let vec1 = LalgrsVector::new(vec![1.0, 2.0, 3.0]);
        let vec2 = LalgrsVector::new(vec![2.0, 3.0, 4.0]);

        let empty_vec = LalgrsVector::new(vec![]);
        assert_eq!(
            (vec1.clone() + vec2.clone()).unwrap(),
            LalgrsVector::new(vec![3.0, 5.0, 7.0])
        );
        assert_eq!((vec1.clone() + empty_vec.clone()).unwrap(), vec1.clone());
        assert_eq!((empty_vec.clone() + vec1.clone()).unwrap(), vec1.clone());
        assert_eq!(
            (vec1.clone() - vec2.clone()).unwrap(),
            LalgrsVector::new(vec![-1.0, -1.0, -1.0])
        );
        assert_eq!(-vec1.clone(), LalgrsVector::new(vec![-1.0, -2.0, -3.0]));
        assert_eq!(vec1.clone() * 2.0, LalgrsVector::new(vec![2.0, 4.0, 6.0]));
        assert_eq!(vec1.clone() * 0.0, LalgrsVector::new(vec![0.0, 0.0, 0.0]));
        assert_eq!((vec1.clone() - empty_vec).unwrap(), vec1.clone());
    }

    #[test]
    fn test_matrix_operations() {
        let matrix1 = LalgrsMatrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]).unwrap();
        let vec1 = LalgrsVector::new(vec![1.0, 2.0]);

        let matrix_vector_mul_res = (matrix1.clone() * vec1.clone()).unwrap();

        assert_eq!(matrix_vector_mul_res, LalgrsVector::new(vec![1.0, 2.0]));
    }
}

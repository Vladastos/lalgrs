pub mod lalgrs;

fn main() -> Result<(), lalgrs::LalgrsError> {
    let vec1 = lalgrs::structs::LalgrsVector::new(vec![1.0, 2.0]);

    let matrix1 = lalgrs::structs::LalgrsMatrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?;

    let multiplied = (matrix1 * vec1)?;
    println!("Multiplied: {multiplied:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lalgrs::structs::LalgrsVector;

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
}

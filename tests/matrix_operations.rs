use lalgrs::{LalgrsError, LalgrsMatrix, LalgrsVector};

fn init_vecs() -> (LalgrsVector<f64>, LalgrsVector<f64>) {
    let vec1 = LalgrsVector::new(vec![1.0, 2.0, 3.0]);
    let vec2 = LalgrsVector::new(vec![2.0, 3.0, 4.0]);
    return (vec1, vec2);
}
fn init_2x2_matrices() -> (LalgrsMatrix<f64>, LalgrsMatrix<f64>) {
    let matrix1 = LalgrsMatrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]).unwrap();
    let matrix2 = LalgrsMatrix::new(vec![vec![2.0, 0.0], vec![0.0, 2.0]]).unwrap();
    return (matrix1, matrix2);
}

fn init_3x3_matrices() -> (LalgrsMatrix<f64>, LalgrsMatrix<f64>) {
    let matrix1 = LalgrsMatrix::new(vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ])
    .unwrap();
    let matrix2 = LalgrsMatrix::new(vec![
        vec![2.0, 0.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![0.0, 0.0, 2.0],
    ])
    .unwrap();
    return (matrix1, matrix2);
}
#[test]
fn test_matrix_add() {
    let (matrix1, matrix2) = init_2x2_matrices();
    assert_eq!(
        (matrix1 + matrix2).unwrap(),
        LalgrsMatrix::new(vec![vec![3.0, 0.0], vec![0.0, 3.0]]).unwrap()
    );
}

#[test]
fn test_matrix_neg() {
    let matrix1 = LalgrsMatrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]).unwrap();
    assert_eq!(
        -matrix1,
        LalgrsMatrix::new(vec![vec![-1.0, 0.0], vec![0.0, -1.0]]).unwrap()
    );
}

#[test]
fn test_matrix_sub() {
    let (matrix1, matrix2) = init_2x2_matrices();
    assert_eq!(
        (matrix1 - matrix2).unwrap(),
        LalgrsMatrix::new(vec![vec![-1.0, 0.0], vec![0.0, -1.0]]).unwrap()
    );
}

#[test]
fn test_matrix_add_error() {
    let (matrix_3x3_1, matrix_3x3_2) = init_3x3_matrices();
    let (matrix_2x2_1, matrix_2x2_2) = init_2x2_matrices();
    assert_eq!(
        (matrix_3x3_1 + matrix_2x2_2).unwrap_err(),
        LalgrsError::MismatchedMatrixDimensions {
            first_matrix_columns: 3,
            second_matrix_rows: 2
        }
    );
    assert_eq!(
        (matrix_2x2_1 + matrix_3x3_2).unwrap_err(),
        LalgrsError::MismatchedMatrixDimensions {
            first_matrix_columns: 2,
            second_matrix_rows: 3
        }
    );
}

#[test]
fn test_matrix_sub_error() {
    let (matrix_3x3_1, matrix_3x3_2) = init_3x3_matrices();
    let (matrix_2x2_1, matrix_2x2_2) = init_2x2_matrices();
    assert_eq!(
        (matrix_3x3_1 - matrix_2x2_2).unwrap_err(),
        LalgrsError::MismatchedMatrixDimensions {
            first_matrix_columns: 3,
            second_matrix_rows: 2
        }
    );
    assert_eq!(
        (matrix_2x2_1 - matrix_3x3_2).unwrap_err(),
        LalgrsError::MismatchedMatrixDimensions {
            first_matrix_columns: 2,
            second_matrix_rows: 3
        }
    );
}

#[test]
fn test_matrix_add_empty() {
    let (matrix1, matrix2) = init_2x2_matrices();
    let empty_matrix = LalgrsMatrix::new(vec![]).unwrap();
    assert_eq!((matrix1.clone() + empty_matrix.clone()).unwrap(), matrix1);
    assert_eq!((empty_matrix.clone() + matrix1.clone()).unwrap(), matrix1);
    assert_eq!((matrix2.clone() + empty_matrix.clone()).unwrap(), matrix2);
    assert_eq!((empty_matrix.clone() + matrix2.clone()).unwrap(), matrix2);
    assert_eq!(
        (empty_matrix.clone() + empty_matrix.clone()).unwrap(),
        empty_matrix
    );
}

#[test]
fn test_matrix_sub_empty() {
    let (matrix1, matrix2) = init_2x2_matrices();
    let empty_matrix = LalgrsMatrix::new(vec![]).unwrap();
    assert_eq!((matrix1.clone() - empty_matrix.clone()).unwrap(), matrix1);
    assert_eq!((empty_matrix.clone() - matrix1.clone()).unwrap(), -matrix1);
    assert_eq!((matrix2.clone() - empty_matrix.clone()).unwrap(), matrix2);
    assert_eq!((empty_matrix.clone() - matrix2.clone()).unwrap(), -matrix2);
    assert_eq!(
        (empty_matrix.clone() - empty_matrix.clone()).unwrap(),
        empty_matrix
    );
}

#[test]
fn test_matrix_mul() {
    let matrix1 = LalgrsMatrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]).unwrap();
    let matrix2 = LalgrsMatrix::new(vec![vec![2.0, 0.0], vec![0.0, 2.0]]).unwrap();
    assert_eq!(
        (matrix1 * matrix2).unwrap(),
        LalgrsMatrix::new(vec![vec![2.0, 0.0], vec![0.0, 2.0]]).unwrap()
    );

    let matrix3 = LalgrsMatrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
    let matrix4 = LalgrsMatrix::new(vec![vec![5.0, 6.0], vec![7.0, 8.0]]).unwrap();
    assert_eq!(
        (matrix3 * matrix4).unwrap(),
        LalgrsMatrix::new(vec![vec![23.0, 34.0], vec![31.0, 46.0]]).unwrap()
    );

    let matrix5 = LalgrsMatrix::new(vec![vec![1, 0], vec![1, 1]]).unwrap();
    let matrix6 = LalgrsMatrix::new(vec![vec![0, 1], vec![-1, 0]]).unwrap();
    assert_eq!(
        (matrix5 * matrix6).unwrap(),
        LalgrsMatrix::new(vec![vec![1, 1], vec![-1, 0]]).unwrap()
    );
}

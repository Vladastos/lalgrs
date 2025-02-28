use lalgrs::{LalgrsError, LalgrsMatrix, LalgrsVector};

fn init_vecs() -> (LalgrsVector<f64>, LalgrsVector<f64>) {
    let vec1 = LalgrsVector::new(vec![1.0, 2.0, 3.0]);
    let vec2 = LalgrsVector::new(vec![2.0, 3.0, 4.0]);
    return (vec1, vec2);
}
fn init_2x2_matrixes() -> (LalgrsMatrix<f64>, LalgrsMatrix<f64>) {
    let matrix1 = LalgrsMatrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]).unwrap();
    let matrix2 = LalgrsMatrix::new(vec![vec![2.0, 0.0], vec![0.0, 2.0]]).unwrap();
    return (matrix1, matrix2);
}
fn init_3x3_matrixes() -> (LalgrsMatrix<f64>, LalgrsMatrix<f64>) {
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
    let (matrix1, matrix2) = init_2x2_matrixes();
    assert_eq!(
        (matrix1 + matrix2).unwrap(),
        LalgrsMatrix::new(vec![vec![3.0, 0.0], vec![0.0, 3.0]]).unwrap()
    );
}

#[test]
fn test_matrix_sub() {
    let (matrix1, matrix2) = init_2x2_matrixes();
    assert_eq!(
        (matrix1 - matrix2).unwrap(),
        LalgrsMatrix::new(vec![vec![-1.0, 0.0], vec![0.0, -1.0]]).unwrap()
    );
}

#[test]
fn test_matrix_add_error() {
    let (matrix_3x3_1, matrix_3x3_2) = init_3x3_matrixes();
    let (matrix_2x2_1, matrix_2x2_2) = init_2x2_matrixes();
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
    let (matrix_3x3_1, matrix_3x3_2) = init_3x3_matrixes();
    let (matrix_2x2_1, matrix_2x2_2) = init_2x2_matrixes();
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

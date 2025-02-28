use lalgrs::{LalgrsError, LalgrsVector};

fn init_vecs_size_2() -> (LalgrsVector<f64>, LalgrsVector<f64>) {
    let vec1 = LalgrsVector::new(vec![1.0, 2.0]);
    let vec2 = LalgrsVector::new(vec![2.0, 3.0]);
    return (vec1, vec2);
}
fn init_vecs_size_3() -> (LalgrsVector<f64>, LalgrsVector<f64>) {
    let vec1 = LalgrsVector::new(vec![1.0, 2.0, 3.0]);
    let vec2 = LalgrsVector::new(vec![2.0, 3.0, 4.0]);
    return (vec1, vec2);
}

#[test]
fn test_vector_add() {
    let (vec1, vec2) = init_vecs_size_3();
    assert_eq!(
        (vec1 + vec2).unwrap(),
        LalgrsVector::new(vec![3.0, 5.0, 7.0])
    );
}

#[test]
fn test_vector_add_error() {
    let (vec_size_3_1, vec_size_3_2) = init_vecs_size_3();
    let (vec_size_2_1, vec_size_2_2) = init_vecs_size_2();
    assert_eq!(
        (vec_size_3_1 + vec_size_2_2).unwrap_err(),
        LalgrsError::MismatchedVectorDimensions {
            vector1: 3,
            vector2: 2
        }
    );
    assert_eq!(
        (vec_size_2_1 + vec_size_3_2).unwrap_err(),
        LalgrsError::MismatchedVectorDimensions {
            vector1: 2,
            vector2: 3
        }
    );
}

#[test]
fn test_vector_add_empty() {
    let (vec1, vec2) = init_vecs_size_3();
    let empty_vec = LalgrsVector::new(vec![]);
    assert_eq!((vec1.clone() + empty_vec.clone()).unwrap(), vec1);
    assert_eq!((empty_vec.clone() + vec1.clone()).unwrap(), vec1);
    assert_eq!((vec2.clone() + empty_vec.clone()).unwrap(), vec2);
    assert_eq!((empty_vec.clone() + vec2.clone()).unwrap(), vec2);
    assert_eq!((empty_vec.clone() + empty_vec.clone()).unwrap(), empty_vec);
}

#[test]
fn test_vector_neg() {
    let vec1 = LalgrsVector::new(vec![1.0, 2.0, 3.0]);
    assert_eq!(-vec1, LalgrsVector::new(vec![-1.0, -2.0, -3.0]));
}

#[test]
fn test_vector_sub() {
    let (vec1, vec2) = init_vecs_size_3();
    assert_eq!(
        (vec1 - vec2).unwrap(),
        LalgrsVector::new(vec![-1.0, -1.0, -1.0])
    );
}
#[test]
fn test_vector_sub_empty() {
    let (vec1, vec2) = init_vecs_size_3();
    let empty_vec = LalgrsVector::new(vec![]);
    assert_eq!((vec1.clone() - empty_vec.clone()).unwrap(), vec1);
    assert_eq!((empty_vec.clone() - vec1.clone()).unwrap(), -vec1);
    assert_eq!((vec2.clone() - empty_vec.clone()).unwrap(), vec2);
    assert_eq!((empty_vec.clone() - vec2.clone()).unwrap(), -vec2);
    assert_eq!((empty_vec.clone() - empty_vec.clone()).unwrap(), empty_vec);
}

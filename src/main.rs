use lalgrs::{vector::Vector, LalgrsError};

fn main() -> Result<(), LalgrsError> {
    let vec1 = Vector::new(vec![1.0, 2.0, 3.0]);

    let vec2 = Vector::new(vec![2.0, 3.0, 4.0]);
    let vec3 = (vec1 + vec2)?;
    println!("{:?}", vec3);
    Ok(())
}

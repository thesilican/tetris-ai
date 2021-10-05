use nalgebra::DMatrix;
use std::time::Instant;

fn main() {
    let mat1 = DMatrix::<f32>::zeros(250, 1000);
    let mat2 = DMatrix::<f32>::zeros(1000, 1000);
    let start = Instant::now();
    let mat3 = mat1 * mat2;
    let end = start.elapsed();
    println!("{:?}", end);
}

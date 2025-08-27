use anyhow::Result;
use concurrency2::matrix::Matrix;

fn main() -> Result<()> {
    let matrix1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
    println!("{matrix1:?}");
    let matrix2 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
    println!("{matrix2:?}");
    Ok(())
}

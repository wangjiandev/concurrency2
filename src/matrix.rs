use anyhow::{Result, anyhow};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul},
};

#[derive(Debug)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: data.into(),
        }
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Mul<Output = T> + Add<Output = T> + Copy + AddAssign + Default,
{
    let cols = a.cols;
    let rows = b.rows;

    if cols != rows {
        return Err(anyhow!("Matrix dimensions do not match"));
    }

    let mut data: Vec<_> = vec![T::default(); a.rows * b.cols];

    for i in 0..a.rows {
        for j in 0..b.cols {
            for k in 0..a.cols {
                data[i * b.cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
        }
    }

    let result = Matrix::new(data, a.rows, b.cols);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_should_work_2_3() -> Result<()> {
        let matrix1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let matrix2 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        let result = multiply(&matrix1, &matrix2)?;
        assert_eq!(result.cols, 2);
        assert_eq!(result.rows, 2);
        assert_eq!(result.data, vec![22, 28, 49, 64]);
        Ok(())
    }

    #[test]
    fn test_multiply_should_work_2_2() -> Result<()> {
        let matrix1 = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let matrix2 = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let result = multiply(&matrix1, &matrix2)?;
        assert_eq!(result.cols, 2);
        assert_eq!(result.rows, 2);
        assert_eq!(result.data, vec![7, 10, 15, 22]);
        Ok(())
    }

    #[test]
    fn test_multiply_should_work() -> Result<()> {
        let matrix1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let matrix2 = Matrix::new(vec![10, 11, 20, 21, 30, 31], 3, 2);
        let result = multiply(&matrix1, &matrix2)?;
        assert_eq!(result.cols, 2);
        assert_eq!(result.rows, 2);
        assert_eq!(result.data, vec![140, 146, 320, 335]);
        Ok(())
    }
}

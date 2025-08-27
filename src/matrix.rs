use crate::vector::{Vector, dot_product};
use anyhow::{Result, anyhow};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

const NUM_WORKERS: usize = 4;

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

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy + AddAssign + Default + Send + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Failed to multiply matrices")
    }
}

pub struct MessageInput<T> {
    pub idx: usize,
    pub row: Vector<T>,
    pub col: Vector<T>,
}

impl<T> MessageInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

pub struct MessageOutput<T> {
    pub idx: usize,
    pub value: T,
}

impl<T> MessageOutput<T> {
    pub fn new(idx: usize, value: T) -> Self {
        Self { idx, value }
    }
}

pub struct Message<T> {
    pub input: MessageInput<T>,
    pub sender: oneshot::Sender<MessageOutput<T>>,
}

impl<T> Message<T> {
    pub fn new(input: MessageInput<T>, sender: oneshot::Sender<MessageOutput<T>>) -> Self {
        Self { input, sender }
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Mul<Output = T> + Add<Output = T> + Copy + AddAssign + Default + Send + 'static,
{
    let cols = a.cols;
    let rows = b.rows;

    if cols != rows {
        return Err(anyhow!("Matrix dimensions do not match"));
    }

    let senders = (0..NUM_WORKERS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Message<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MessageOutput::new(msg.input.idx, value)) {
                        eprintln!("Error sending message: {e:?}");
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let matrix_len = a.rows * b.cols;
    let mut data: Vec<_> = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);

    for i in 0..a.rows {
        for j in 0..b.cols {
            let row = Vector::new(&a.data[i * a.cols..(i + 1) * a.cols]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.cols)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            let index = i * b.cols + j;
            let input = MessageInput::new(index, row, col);

            let (tx, rx) = oneshot::channel::<MessageOutput<T>>();
            let message = Message::new(input, tx);
            if let Err(e) = senders[index % NUM_WORKERS].send(message) {
                eprintln!("Error sending message: {e:?}");
            }
            receivers.push(rx);
        }
    }

    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.value;
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
        let result = matrix1 * matrix2;
        assert_eq!(result.cols, 2);
        assert_eq!(result.rows, 2);
        assert_eq!(result.data, vec![22, 28, 49, 64]);
        Ok(())
    }

    #[test]
    fn test_multiply_should_work_2_2() -> Result<()> {
        let matrix1 = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let matrix2 = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let result = matrix1 * matrix2;
        assert_eq!(result.cols, 2);
        assert_eq!(result.rows, 2);
        assert_eq!(result.data, vec![7, 10, 15, 22]);
        Ok(())
    }

    #[test]
    fn test_multiply_should_work() -> Result<()> {
        let matrix1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let matrix2 = Matrix::new(vec![10, 11, 20, 21, 30, 31], 3, 2);
        let result = matrix1 * matrix2;
        assert_eq!(result.cols, 2);
        assert_eq!(result.rows, 2);
        assert_eq!(result.data, vec![140, 146, 320, 335]);
        Ok(())
    }
}

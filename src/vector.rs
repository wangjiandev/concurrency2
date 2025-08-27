use anyhow::{Result, anyhow};
use std::ops::{Add, AddAssign, Deref, Mul};

pub struct Vector<T> {
    pub data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy + AddAssign + Default,
{
    if a.len() != b.len() {
        return Err(anyhow!("Dot product dimensions do not match"));
    }

    let mut result = T::default();
    for i in 0..a.len() {
        result += a[i] * b[i];
    }

    Ok(result)
}

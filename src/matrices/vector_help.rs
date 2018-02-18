use num::Num;
use std::ops::Add;
use std::fmt::{Debug, Display};

pub fn reduce_row(vec: &Vec<f64>, k: usize, c: &Vec<f64>) -> (Vec<f64>, f64) {
    let n = &vec[k];
    if *n == 0.0 {
        return (vec.clone(), c[k]);
    }
    let mut red = Vec::new();

    for e in vec.iter() {
        red.push(e / n);
    }

    (red, c[k] / n)
}

pub fn get_scalar_product_of_vectors<T>(vec1: &Vec<T>, vec2: &Vec<T>) -> T
    where T: Num + Clone + Add<T> + Copy + Debug + Display
{
    (0..vec1.len()).fold(T::zero(), |sum, x| sum + vec1[x] * vec2[x])
}

pub fn add_gaussian(v1: &Vec<f64>, v2: &Vec<f64>, k: usize, a: f64, b: f64) -> (Vec<f64>, f64) {
    if v1.len() != v2.len() {
        panic!("Both vectors must have the same length");
    }

    let mut vec = Vec::new();
    let p = (-1.0) * v2[k];
    for i in 0..v1.len() {
        vec.push(p * v1[i] + v2[i]);
    }

    (vec, p * a + b)
}
extern crate num;
use self::num::{PrimInt, Integer, Float, Zero, One};
use std::iter::{Sum, Product};
use std::ops::{Sub, Mul, Div};
use std::iter::Iterator;
//use std::f64::consts::E;
use std::f64::consts::PI;
//use std::convert::Into;
use basic::pow;

pub fn factorial<T>(n: T) -> T
    where T: PrimInt + Integer + Product
{
    if n < T::zero() {
        panic!("Value for facultation must be a positive integer!");
    }

    num::range(T::one(), n + T::one()).product()
}

pub fn permutation<T>(n: T, karr: Vec<T>) -> T
    where T: PrimInt + Integer + Product + Mul + Sum
{
    let karr_2 = karr.clone();
    let sum: T = karr.into_iter().sum();
    if sum != n {
        panic!("Sum of parts is not equal to whole");
    }

    let divisor = karr_2.into_iter().fold(T::one(), |prod, x| prod * factorial(x));
    factorial(n) / divisor
}

pub fn combination<T>(n: T, k: T) -> T
    where T: PrimInt + Integer + Product
{
    if k > n {
        panic!("Number of selections outgrows the number of elements");
    }

    factorial(n) / factorial(n - k) / factorial(k)
}

pub fn combination_with_repetition<T>(n: T, k: T) -> T
    where T: PrimInt + Integer + Product
{
    let m = n + k - T::one();
    factorial(m) / factorial(m - k) / factorial(k)
}

pub fn variation<T>(n: T, k: T) -> T
    where T: PrimInt + Integer + Product
{
    if k > n {
        panic!("Number of selections outgrows the number of elements");
    }

    factorial(n) / factorial(n - k)
}

//pub fn variation_with_repetition<T>(n: T, k: T) -> T
//    where T: PrimInt + Integer + Unsigned
//{
//    pow(n, k)
//}

pub fn binomial_distribution<T, U>(n: T, p: U) -> Vec<f64>
    where T: Zero + PrimInt + Integer + Product + ConvertTof64,
          U: One + Sub + Float + ConvertTof64
{
    let q = U::one() - p;

    let binomial = num::range(T::zero(), n + T::one()).fold(Vec::new(), |mut vec, x| {
        let a = factorial(n) / (factorial(x) * factorial(n - x));
        let b = pow(p, x);
        let c = pow(q, n - x);
        let result = a.to_f64() * b.to_f64() * c.to_f64();
        vec.push(result);
        vec
    });

    binomial
}

pub fn hypergeometric_distribution<T>(N: T, M: T, n: T) -> Vec<f64>
    where T: PrimInt + Integer + Product + Sub + ConvertTof64
{
    let hypergeometric = num::range(T::zero(), n + T::one()).fold(Vec::new(), |mut vec, x| {
        let a = combination(M, x);
        let b = combination(N - M, n - x);
        let c = combination(N, n);
        let result = a.to_f64() * b.to_f64() / c.to_f64();
        vec.push(result);
        vec
    });

    hypergeometric
}

pub fn poisson_distribution<T>(my: T, x: T) -> f64
    where T: PrimInt + Integer + Product + ConvertTof64
{
    let a = pow(my, x).to_f64();
    let b = factorial(x).to_f64();
    let c = (my.to_f64() * (-1.0)).exp();

   a / b * c
}

pub fn gaussian_distribution<T>(my: T, sigma: T, x: T) -> f64
    where T: ConvertTof64 + Copy
{
    let a = (2.0 * PI).sqrt() * sigma.to_f64();
    let b = (x.to_f64() - my.to_f64()) / sigma.to_f64();
    let c = -0.5 * pow(b, 2);
    let d = c.exp();

    (1.0 / a) * d
}

pub fn standard_distribution<T>(x: T) -> f64
    where T: ConvertTof64
{
    let a = (2.0 * PI).sqrt();
    let b = -0.5 * pow(x.to_f64(), 2);
    let c = b.exp();

    (1.0 / a) * c
}

pub fn exponential_distribution<T>(lambda: T, x: T) -> f64
    where T: Zero + ConvertTof64 + PartialOrd + Copy
{
    if x < T::zero() {
        0.0
    } else {
        let a = lambda.to_f64() * (-1.0) * x.to_f64();
        lambda.to_f64() * a.exp()
    }
}

pub trait ConvertTof64 {
    fn to_f64(self) -> f64;
}

impl ConvertTof64 for i8 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ConvertTof64 for i16 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ConvertTof64 for i32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ConvertTof64 for i64 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ConvertTof64 for f32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ConvertTof64 for f64 {
    fn to_f64(self) -> f64 {
        self
    }
}
extern crate num;
use self::num::{PrimInt, Integer, Float, Zero, One};
use std::iter::{Sum, Product};
use std::ops::{Sub, Mul};
use std::iter::Iterator;
//use std::f64::consts::E;
use std::f64::consts::PI;
//use std::convert::Into;
use basic::{pow, Convert};

/// Calculates the factorial of a given number n.
/// Returns the calculated factorial.
/// Panics if n is smaller than 0.
pub fn factorial<T>(n: T) -> T
    where T: PrimInt + Integer + Product
{
    if n < T::zero() {
        panic!("Value for facultation must be a positive integer!");
    }

    num::range(T::one(), n + T::one()).product()
}

/// Calculates the number of possibilities the elements of a given multiset (a set containing smaller sets) can be arranged in a specific order.
/// n is the total number of elements in the multiset.
/// karr is a vector containing the number of elements in the specific subsets of the multisets.
/// Returns the number of possible permutations.
/// Panics if the combined number of elements in the subsets does not equal the number of elements in the multiset
// TODO: panic if vec contains negative or zero values
pub fn permutation<T>(n: T, karr: Vec<T>) -> T
    where T: PrimInt + Integer + Product + Mul + Sum
{
    if n < T::one() {
        panic!("Parameter n must be a positive integer!");
    }
    if karr.is_empty() {
        panic!("Parameter karr is an empty vector!")
    }

    let karr_2 = karr.clone();
    let sum: T = karr.into_iter().sum();
    if sum != n {
        panic!("Sum of parts is not equal to whole");
    }

    let divisor = karr_2.into_iter().fold(T::one(), |prod, x| prod * factorial(x));
    factorial(n) / divisor
}

/// Calculates how many times the elements of a given set can be arranged in no particular order without repetition.
/// n is the total number of elements in the set.
/// k is the number of elements that are picked from the set.
/// Panics if k is bigger than n.
pub fn combination<T>(n: T, k: T) -> T
    where T: PrimInt + Integer + Product
{
    if n < T::zero() {
        panic!("Parameter n must be a positive integer!");
    }
    if k < T::zero() {
        panic!("Parameter k must be a positive integer!");
    }
    if k > n {
        panic!("Number of selections outgrows the number of elements");
    }

    factorial(n) / factorial(n - k) / factorial(k)
}

/// Calculates how many times the elements of a given set can be arranged in no particular order with repetition.
/// n is the total number of elements in the set.
/// k is the number of elements that are picked from the set.
pub fn combination_with_repetition<T>(n: T, k: T) -> T
    where T: PrimInt + Integer + Product
{
    if n < T::zero() {
        panic!("Parameter n must be a positive integer!");
    }
    if k < T::zero() {
        panic!("Parameter k must be a positive integer!");
    }

    let m = n + k - T::one();
    factorial(m) / factorial(m - k) / factorial(k)
}

/// Calculates how many times the elements of a given set can be arranged in a particular order without repetition.
/// All elements in the set are different from each other.
/// n is the total number of elements in the set.
/// k is the number of elements that are picked from the set.
/// Panics if k is bigger than n.
pub fn variation<T>(n: T, k: T) -> T
    where T: PrimInt + Integer + Product
{
    if n < T::zero() {
        panic!("Parameter n must be a positive integer!");
    }
    if k < T::zero() {
        panic!("Parameter k must be a positive integer!");
    }

    if k > n {
        panic!("Number of selections outgrows the number of elements");
    }

    factorial(n) / factorial(n - k)
}

/// Calculates how many times the elements of a given set can be arranged in a particular order with repetition.
/// All elements in the set are different from each other.
/// n is the total number of elements in the set.
/// k is the number of elements that are picked from the set.
// TODO: panic if n and k are both zero
pub fn variation_with_repetition<T>(n: T, k: T) -> T
    where T: PrimInt + Integer
{
    if n < T::zero() {
        panic!("Parameter n must be a positive integer!");
    }
    if k < T::zero() {
        panic!("Parameter k must be a positive integer!");
    }

    pow(n, k)
}

/// Calculates the binomial distribution in dependency of n and p.
/// n is the number of Bernoulli experiments
/// p is the probability for an event.
/// Panics if p is not between 0 and 1.
pub fn binomial_distribution<T, U>(n: T, p: U) -> Vec<f64>
    where T: Zero + PrimInt + Integer + Product + Convert,
          U: One + Sub + Float + Convert
{
    if n < T::one() {
        panic!("Parameter n must be a positive integer!");
    }
    if p < U::zero() || p > U::one() {
        panic!("p must be in a range between 0 and 1!");
    }

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

/// Calculates the binomial distribution in dependency of N, M and n.
/// In a set with two types of elements,
/// N is the total number of elements in the set.
/// M is the number of elements in one of the two subsets.
/// n is the number of elements that are picked from the set
/// Panics if M or n are bigger than N.
#[allow(non_snake_case)]
pub fn hypergeometric_distribution<T>(N: T, M: T, n: T) -> Vec<f64>
    where T: PrimInt + Integer + Product + Sub + Convert
{
    if N < T::one() {
        panic!("Parameter N must be a positive integer!");
    }
    if M < T::one() {
        panic!("Parameter M must be a positive integer!");
    }
    if n < T::one() {
        panic!("Parameter n must be a positive integer!");
    }
    if M > N {
        panic!("Parameter M must be smaller than N!");
    }
    if n > N {
        panic!("Parameter n must be smaller than N!")
    }

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
    where T: PrimInt + Integer + Product + Convert
{
    if my <= T::zero() {
        panic!("Parameter µ must be positive!");
    }
    if x < T::zero() {
        panic!("Parameter x must be a positive integer!");
    }

    let a = pow(my, x).to_f64();
    let b = factorial(x).to_f64();
    let c = (my.to_f64() * (-1.0)).exp();

   a / b * c
}

pub fn gaussian_distribution<T>(my: T, sigma: T, x: T) -> f64
    where T: Convert + Copy + Zero + PartialOrd
{
    if sigma <= T::zero() {
        panic!("Parameter \u{03c3} must be bigger than 0!");
    }

    let a = (2.0 * PI).sqrt() * sigma.to_f64();
    let b = (x.to_f64() - my.to_f64()) / sigma.to_f64();
    let c = -0.5 * pow(b, 2);
    let d = c.exp();

    (1.0 / a) * d
}

pub fn standard_distribution<T>(x: T) -> f64
    where T: Convert
{
    let a = (2.0 * PI).sqrt();
    let b = -0.5 * pow(x.to_f64(), 2);
    let c = b.exp();

    (1.0 / a) * c
}

pub fn exponential_distribution<T>(lambda: T, x: T) -> f64
    where T: Zero + Convert + PartialOrd + Copy
{
    if lambda <= T::zero() {
        panic!("Parameter \u{03bb} must be bigger than 0!");
    }

    if x < T::zero() {
        0.0
    } else {
        let a = lambda.to_f64() * (-1.0) * x.to_f64();
        lambda.to_f64() * a.exp()
    }
}
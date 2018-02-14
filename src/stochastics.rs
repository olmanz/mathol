use num::{PrimInt, Integer, Float, Zero, One, range};
use std::iter::{Sum, Product};
use std::ops::{Sub, Mul};
use std::iter::Iterator;
//use std::f64::consts::E;
use std::f64::consts::PI;
//use std::convert::Into;
use basic::{pow, Convert};

/// Calculates the factorial of a given number n.
/// # Remarks
/// Returns the calculated factorial or an error message if n is smaller than 0.
/// # Examples
/// ```
/// assert_eq!(1, factorial(0).unwrap());
/// assert_eq!(2432902008176640000, factorial(20_u64).unwrap());
/// assert_eq!(479001600, factorial(12_i32).unwrap());
/// ```
pub fn factorial<T>(n: T) -> Result<T, &'static str>
    where T: PrimInt + Integer + Product
{
    if n < T::zero() {
        return Err("Value for facultation must be a positive integer!");
    }

    Ok(range(T::one(), n + T::one()).product())
}

/// Calculates the number of possibilities the elements of a given multiset (a set containing smaller sets) can be arranged in a specific order.
/// # Remarks
/// n is the total number of elements in the multiset.
///
/// karr is a vector containing the number of elements in the specific subsets of the multisets.
///
/// Returns the number of possible permutations or an error message if the combined number of
/// elements in the subsets does not equal the number of elements in the multiset
/// # Examples
/// ```
/// assert_eq!(10, permutation(5, vec![3, 2]).unwrap());
/// ```
pub fn permutation<T>(n: T, karr: Vec<T>) -> Result<T, &'static str>
    where T: PrimInt + Integer + Product + Mul + Sum
{
    if n < T::one() {
        return Err("Parameter n must be a positive integer!");
    }
    if karr.is_empty() {
        return Err("Parameter karr is an empty vector!");
    }

    let karr_2 = karr.clone();
    let sum: T = karr.into_iter().sum();
    if sum != n {
        return Err("Sum of parts is not equal to whole");
    }

    let divisor = karr_2.into_iter().fold(T::one(), |prod, x| prod * factorial(x).unwrap());
    Ok(factorial(n).unwrap() / divisor)
}

/// Calculates how many times the elements of a given set can be arranged in no particular order without repetition.
/// # Remarks
/// n is the total number of elements in the set.
///
/// k is the number of elements that are picked from the set.
///
/// Returns the number of possible combinations or an error message if k is bigger than n.
/// # Examples
/// ```
/// assert_eq!(792, combination(12, 7).unwrap());
/// ```
pub fn combination<T>(n: T, k: T) -> Result<T, &'static str>
    where T: PrimInt + Integer + Product
{
    if n < T::zero() {
        return Err("Parameter n must be a positive integer!");
    }
    if k < T::zero() {
        return Err("Parameter k must be a positive integer!");
    }
    if k > n {
        return Err("Number of selections outgrows the number of elements");
    }

    Ok(factorial(n).unwrap() / factorial(n - k).unwrap() / factorial(k).unwrap())
}

/// Calculates how many times the elements of a given set can be arranged in no particular order with repetition.
/// # Remarks
/// n is the total number of elements in the set.
///
/// k is the number of elements that are picked from the set.
///
/// Returns the number of possible combinations or an error message if either n or k are smaller than zero.
/// # Examples
/// ```
/// assert_eq!(220, combination_with_repetition(10, 3).unwrap());
/// ```
pub fn combination_with_repetition<T>(n: T, k: T) -> Result<T, &'static str>
    where T: PrimInt + Integer + Product
{
    if n < T::zero() {
        return Err("Parameter n must be a positive integer!");
    }
    if k < T::zero() {
        return Err("Parameter k must be a positive integer!");
    }

    let m = n + k - T::one();
    Ok(factorial(m).unwrap() / factorial(m - k).unwrap() / factorial(k).unwrap())
}

/// Calculates how many times the elements of a given set can be arranged in a particular order without repetition.
/// # Remarks
/// All elements in the set are different from each other.
///
/// n is the total number of elements in the set.
///
/// k is the number of elements that are picked from the set.
///
/// Returns the number of possible variations or an error message if k is bigger than n.
/// # Examples
/// ```
/// assert_eq!(336, variation(8, 3).unwrap());
/// ```
pub fn variation<T>(n: T, k: T) -> Result<T, &'static str>
    where T: PrimInt + Integer + Product
{
    if n < T::zero() {
        return Err("Parameter n must be a positive integer!");
    }
    if k < T::zero() {
        return Err("Parameter k must be a positive integer!");
    }

    if k > n {
        return Err("Number of selections outgrows the number of elements");
    }

    Ok(factorial(n).unwrap() / factorial(n - k).unwrap())
}

/// Calculates how many times the elements of a given set can be arranged in a particular order with repetition.
/// # Remarks
/// All elements in the set are different from each other.
///
/// n is the total number of elements in the set.
///
/// k is the number of elements that are picked from the set.
///
/// Returns the number of possible variations or an error message if k is bigger than n.
/// # Examples
/// ```
/// assert_eq!(125, variation_with_repetition(5, 3).unwrap());
/// ```
pub fn variation_with_repetition<T>(n: T, k: T) -> Result<T, &'static str>
    where T: PrimInt + Integer
{
    if n < T::zero() {
        return Err("Parameter n must be a positive integer!");
    }
    if k < T::zero() {
        return Err("Parameter k must be a positive integer!");
    }

    Ok(pow(n, k))
}

/// Calculates the binomial distribution in dependency of n and p.
/// # Remarks
/// n is the number of Bernoulli experiments
///
/// p is the probability for an event.
///
/// Returns the binomial distribution as a vector or an error message if p is not between 0 and 1.
/// # Examples
/// ```
/// let mut vec = vec![];
/// vec.push(0.5314410000000002);
/// vec.push(0.35429400000000016);
/// vec.push(0.09841500000000003);
/// vec.push(0.014580000000000004);
/// vec.push(0.0012150000000000004);
/// vec.push(0.00005400000000000002);
/// vec.push(0.0000010000000000000004);
/// assert_eq!(vec, binomial_distribution(6_i32, 0.1).unwrap());
/// ```
pub fn binomial_distribution<T, U>(n: T, p: U) -> Result<Vec<f64>, &'static str>
    where T: Zero + PrimInt + Integer + Product + Convert,
          U: One + Sub + Float + Convert
{
    if n < T::one() {
        return Err("Parameter n must be a positive integer!");
    }
    if p < U::zero() || p > U::one() {
        return Err("p must be in a range between 0 and 1!");
    }

    let q = U::one() - p;

    let binomial = range(T::zero(), n + T::one()).fold(Vec::new(), |mut vec, x| {
        let a = factorial(n).unwrap() / (factorial(x).unwrap() * factorial(n - x).unwrap());
        let b = pow(p, x);
        let c = pow(q, n - x);
        let result = a.to_f64() * b.to_f64() * c.to_f64();
        vec.push(result);
        vec
    });

    Ok(binomial)
}

/// Calculates the hypergeometric distribution in dependency of N, M and n.
/// # Remarks
/// In a set with two types of elements,
///
/// N is the total number of elements in the set.
///
/// M is the number of elements in one of the two subsets.
///
/// n is the number of elements that are picked from the set
///
/// Returns the hypergeometric distribution as a vector or an error message if M or n are bigger than N.
/// # Examples
/// ```
/// let mut vec = vec![];
/// vec.push(0.010101010101010102);
/// vec.push(0.1414141414141414);
/// vec.push(0.42424242424242425);
/// vec.push(0.35353535353535354);
/// vec.push(0.0707070707070707);
/// assert_eq!(vec, hypergeometric_distribution(12, 7, 4).unwrap());
/// ```
#[allow(non_snake_case)]
pub fn hypergeometric_distribution<T>(N: T, M: T, n: T) -> Result<Vec<f64>, &'static str>
    where T: PrimInt + Integer + Product + Sub + Convert
{
    if N < T::one() {
        return Err("Parameter N must be a positive integer!");
    }
    if M < T::one() {
        return Err("Parameter M must be a positive integer!");
    }
    if n < T::one() {
        return Err("Parameter n must be a positive integer!");
    }
    if M > N {
        return Err("Parameter M must be smaller than N!");
    }
    if n > N {
        return Err("Parameter n must be smaller than N!")
    }

    let hypergeometric = range(T::zero(), n + T::one()).fold(Vec::new(), |mut vec, x| {
        let a = combination(M, x).unwrap();
        let b = combination(N - M, n - x).unwrap();
        let c = combination(N, n).unwrap();
        let result = a.to_f64() * b.to_f64() / c.to_f64();
        vec.push(result);
        vec
    });

    Ok(hypergeometric)
}

/// Calculates the poisson distribution in dependency of my and x
/// # Remarks
/// Returns the poisson distribution for x or an error message if either my or x are zero.
/// # Examples
/// ```
/// assert_eq!(0.015328310048810096, poisson_distribution(1, 4).unwrap());
/// ```
pub fn poisson_distribution<T>(my: T, x: T) -> Result<f64, &'static str>
    where T: PrimInt + Integer + Product + Convert
{
    if my <= T::zero() {
        return Err("Parameter µ must be positive!");
    }
    if x < T::zero() {
        return Err("Parameter x must be a positive integer!");
    }

    let a = pow(my, x).to_f64();
    let b = factorial(x).unwrap().to_f64();
    let c = (my.to_f64() * (-1.0)).exp();

   Ok(a / b * c)
}

/// Calculates the gaussian distribution in dependency of my, sigma and x
/// # Remarks
/// Returns the gaussian distribution for x or an error message if sigma is smaller than 1.
/// # Examples
/// ```
/// assert_eq!(0.10648266850745075, gaussian_distribution(2, 3, 4).unwrap());
/// ```
pub fn gaussian_distribution<T>(my: T, sigma: T, x: T) -> Result<f64, &'static str>
    where T: Convert + Copy + Zero + PartialOrd
{
    if sigma <= T::zero() {
        return Err("Parameter \u{03c3} must be bigger than 0!");
    }

    let a = (2.0 * PI).sqrt() * sigma.to_f64();
    let b = (x.to_f64() - my.to_f64()) / sigma.to_f64();
    let c = -0.5 * pow(b, 2);
    let d = c.exp();

    Ok((1.0 / a) * d)
}

/// Calculates the standard distribution in dependency of x
/// # Remarks
/// Returns the standard distribution for x
/// # Examples
/// ```
/// assert_eq!(0.05399096651318806, standard_distribution(2.0));
/// ```
pub fn standard_distribution<T>(x: T) -> f64
    where T: Convert
{
    let a = (2.0 * PI).sqrt();
    let b = -0.5 * pow(x.to_f64(), 2);
    let c = b.exp();

    (1.0 / a) * c
}

/// Calculates the exponential distribution in dependency of lambda x
/// # Remarks
/// Returns the exponential distribution for x or an error message if lambda is smaller than 1.
/// # Examples
/// ```
/// assert_eq!(0.03663127777746836, exponential_distribution(2.0, 2.0).unwrap());
/// ```
pub fn exponential_distribution<T>(lambda: T, x: T) -> Result<f64, &'static str>
    where T: Zero + Convert + PartialOrd + Copy
{
    if lambda <= T::zero() {
        return Err("Parameter \u{03bb} must be bigger than 0!");
    }

    if x < T::zero() {
        Ok(0.0)
    } else {
        let a = lambda.to_f64() * (-1.0) * x.to_f64();
        Ok(lambda.to_f64() * a.exp())
    }
}
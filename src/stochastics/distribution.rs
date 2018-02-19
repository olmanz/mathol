use num::{PrimInt, Integer, Float, Zero, One, range};
use std::iter::{Product};
use std::ops::{Sub};
use std::iter::Iterator;
use std::f64::consts::PI;
use basic::{pow, Convert};
use stochastics::probability::{factorial, combination};
use error::DistributionError;


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
pub fn binomial_distribution<T, U>(n: T, p: U) -> Result<Vec<f64>, DistributionError>
    where T: Zero + PrimInt + Integer + Product + Convert,
          U: One + Sub + Float + Convert
{
    if n < T::one() {
        return Err(DistributionError {
            message: "Parameter n must be a positive integer!".to_string(),
        });
    }
    if p < U::zero() || p > U::one() {
        return Err(DistributionError {
            message: "p must be in a range between 0 and 1!".to_string(),
        });
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
pub fn hypergeometric_distribution<T>(N: T, M: T, n: T) -> Result<Vec<f64>, DistributionError>
    where T: PrimInt + Integer + Product + Sub + Convert
{
    if N < T::one() {
        return Err(DistributionError {
            message: "Parameter N must be a positive integer!".to_string(),
        });
    }
    if M < T::one() {
        return Err(DistributionError {
            message: "Parameter M must be a positive integer!".to_string(),
        });
    }
    if n < T::one() {
        return Err(DistributionError {
            message: "Parameter n must be a positive integer!".to_string(),
        });
    }
    if M > N {
        return Err(DistributionError {
            message: "Parameter M must be smaller than N!".to_string(),
        });
    }
    if n > N {
        return Err(DistributionError {
            message: "Parameter n must be smaller than N!".to_string(),
        })
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
pub fn poisson_distribution<T>(my: T, x: T) -> Result<f64, DistributionError>
    where T: PrimInt + Integer + Product + Convert
{
    if my <= T::zero() {
        return Err(DistributionError {
            message: "Parameter µ must be positive!".to_string(),
        });
    }
    if x < T::zero() {
        return Err(DistributionError {
            message: "Parameter x must be a positive Integer!".to_string(),
        });
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
pub fn gaussian_distribution<T>(my: T, sigma: T, x: T) -> Result<f64, DistributionError>
    where T: Convert + Copy + Zero + PartialOrd
{
    if sigma <= T::zero() {
        return Err(DistributionError {
            message: "Parameter \u{03c3} must be bigger than 0!".to_string(),
        });
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
pub fn exponential_distribution<T>(lambda: T, x: T) -> Result<f64, DistributionError>
    where T: Zero + Convert + PartialOrd + Copy
{
    if lambda <= T::zero() {
        return Err(DistributionError {
            message: "Parameter \u{03bb} must be bigger than 0!".to_string(),
        });
    }

    if x < T::zero() {
        Ok(0.0)
    } else {
        let a = lambda.to_f64() * (-1.0) * x.to_f64();
        Ok(lambda.to_f64() * a.exp())
    }
}
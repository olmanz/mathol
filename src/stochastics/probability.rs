use num::{PrimInt, Integer, range};
use std::iter::{Sum, Product};
use std::ops::{Mul};
use std::iter::Iterator;
use basics::pow::pow;
use error::*;
use std::fmt::{Debug, Display};

/// Calculates the factorial of a given number n.
/// # Remarks
/// Returns the calculated factorial or an error message if n is smaller than 0.
/// # Examples
/// ```
/// assert_eq!(1, factorial(0).unwrap());
/// assert_eq!(2432902008176640000, factorial(20_u64).unwrap());
/// assert_eq!(479001600, factorial(12_i32).unwrap());
/// ```
pub fn factorial<T>(n: T) -> Result<T, MatholError>
    where T: PrimInt + Integer + Product
{
    if n < T::zero() {
        return Err(MatholError::NegativeValueCause(NegativeValueError {
            message: "Value for factorial must be a positive integer".to_string(),
        }));
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
pub fn permutation<T>(n: T, karr: Vec<T>) -> Result<T, MatholError>
    where T: PrimInt + Integer + Product + Mul + Sum + Debug + Display
{
    if n < T::one() {
        return Err(MatholError::NegativeValueCause(NegativeValueError {
            message: "Parameter n must be a positive integer!".to_string(),
        }));
    }
    if karr.is_empty() {
        return Err(MatholError::EmptyVecCause(EmptyVectorError {
            message: "Parameter karr is an empty vector!".to_string(),
        }));
    }

    let karr_2 = karr.clone();
    let sum: T = karr.into_iter().sum();
    if sum != n {
        return Err(MatholError::SummationCause(SummationError {
            message: format!("Number of total elements is {:?}, but the summed number of elements from the subset is {:?}", n, sum),
        }));
    }

    let divisor = karr_2.into_iter().fold(T::one(), |prod, x| prod * factorial(x).unwrap());
    Ok(factorial(n)? / divisor)
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
pub fn combination<T>(n: T, k: T) -> Result<T, MatholError>
    where T: PrimInt + Integer + Product
{
    if n < T::zero() {
        return Err(MatholError::NegativeValueCause(NegativeValueError {
            message: "Parameter n must be a positive integer!".to_string(),
        }));
    }
    if k < T::zero() {
        return Err(MatholError::NegativeValueCause(NegativeValueError {
            message: "Parameter k must be a positive integer!".to_string(),
        }));
    }
    if k > n {
        return Err(MatholError::OutgrowCause(OutgrowError {
            message: format!("k is bigger than n"),
        }));
    }

    Ok(factorial(n)? / factorial(n - k)? / factorial(k)?)
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
pub fn combination_with_repetition<T>(n: T, k: T) -> Result<T, MatholError>
    where T: PrimInt + Integer + Product
{
    if n < T::zero() {
        return Err(MatholError::NegativeValueCause(NegativeValueError {
            message: "Parameter n must be a positive integer!".to_string(),
        }));
    }
    if k < T::zero() {
        return Err(MatholError::NegativeValueCause(NegativeValueError {
            message: "Parameter k must be a positive integer!".to_string(),
        }));
    }

    let m = n + k - T::one();
    Ok(factorial(m)? / factorial(m - k)? / factorial(k)?)
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
pub fn variation<T>(n: T, k: T) -> Result<T, MatholError>
    where T: PrimInt + Integer + Product
{
    if n < T::zero() {
        return Err(MatholError::NegativeValueCause(NegativeValueError {
            message: "Parameter n must be a positive integer!".to_string(),
        }));
    }
    if k < T::zero() {
        return Err(MatholError::NegativeValueCause(NegativeValueError {
            message: "Parameter k must be a positive integer!".to_string(),
        }));
    }
    if k > n {
        return Err(MatholError::OutgrowCause(OutgrowError {
            message: format!("k is bigger than n"),
        }));
    }

    Ok(factorial(n)? / factorial(n - k)?)
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
pub fn variation_with_repetition<T>(n: T, k: T) -> Result<T, MatholError>
    where T: PrimInt + Integer
{
    if n < T::zero() {
        return Err(MatholError::NegativeValueCause(NegativeValueError {
            message: "Parameter n must be a positive integer!".to_string(),
        }));
    }
    if k < T::zero() {
        return Err(MatholError::NegativeValueCause(NegativeValueError {
            message: "Parameter k must be a positive integer!".to_string(),
        }));
    }

    Ok(pow(n, k))
}
use num::{Num, FromPrimitive};
use basic::{Convert, pow};
use std::f64;

/// Calculates the arithmetic mean of a slice of Numbers
/// # Remarks
/// Returns a f64 value if the slice contains numbers or an error message if the slice is empty
/// # Examples
/// ```
/// use mathol::statistics::get_arithmetic_mean;
///
/// let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
/// assert_eq!(10.1, get_arithmetic_mean(&vec).unwrap());
///
/// let arr = [8, 6, 5, 11, 6, 6];
/// assert_eq!(7.0, get_arithmetic_mean(&arr).unwrap());
/// ```
pub fn get_arithmetic_mean<T>(data: &[T]) -> Result<f64, &str>
    where T: Num + Convert + Copy
{
    let n = data.iter().len();

    match n {
        0 => Err("Vector or Array is empty"),
        _ => {
            let sum = data.iter().fold(T::zero(), |sum, x| sum + *x);
            let arithmetic_mean = sum.to_f64() / n.to_f64();
            Ok(arithmetic_mean)
        },
    }
}

/// Calculates the harmonic mean of a slice of Numbers
/// # Remarks
/// Returns a f64 value if the slice contains numbers or an error message if the slice is empty or if it contains a zero
/// # Examples
/// ```
/// use mathol::statistics::get_harmonic_mean;
///
/// let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
/// assert_eq!(10.097006905739999, get_harmonic_mean(&vec).unwrap());
///
/// let vec = vec![8, 6, 5, 11, 6, 6];
/// assert_eq!(6.550868486352359, get_harmonic_mean(&vec).unwrap());
/// ```
pub fn get_harmonic_mean<T>(data: &[T]) -> Result<f64, &str>
    where T: Num + Convert + Copy + PartialEq
{
    let n = data.iter().len();

    match n {
        0 => Err("Vector or Array is empty"),
        _ => {
            if data.contains(&T::zero()) {
                return Err("Vector or Array contains zero");
            }

            let sum = data.iter().fold(0.0, |sum, x| sum + 1.0 / x.to_f64());
            let harmonic_mean = 1.0 / (sum / n.to_f64());
            Ok(harmonic_mean)
        }
    }
}

/// Calculates the quadratic mean of a slice of Numbers
/// # Remarks
/// Returns a f64 value if the slice contains numbers or an error message if the slice is empty
/// # Examples
/// ```
/// use mathol::statistics::get_quadratic_mean;
///
/// let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
/// assert_eq!(10.101485039339513, get_quadratic_mean(&vec).unwrap());
///
/// let vec = vec![8, 6, 5, 11, 6, 6];
/// assert_eq!(7.280109889280518, get_quadratic_mean(&vec).unwrap());
/// ```
pub fn get_quadratic_mean<T>(data: &[T]) -> Result<f64, &str>
    where T: Num + Convert + Copy + PartialEq
{
    let n = data.iter().len();

    match n {
        0 => Err("Vector or Array is empty"),
        _ => {
            let sum = data.iter().fold(T::zero(), |sum, x| sum + *x * *x);
            let quadratic_mean = (sum.to_f64() / n.to_f64()).sqrt();
            Ok(quadratic_mean )
        }
    }
}

/// Calculates the variance of a slice of Numbers
/// # Remarks
/// Returns a f64 value if the slice contains numbers or an error message if the slice is empty
/// # Examples
/// ```
/// use mathol::statistics::get_variance;
///
/// let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
/// assert_eq!(0.034285714285714246, get_variance(&vec).unwrap());
///
/// let vec = vec![8, 6, 5, 11, 6, 6];
/// assert_eq!(4.8, get_variance(&vec).unwrap());
/// ```
pub fn get_variance<T>(data: &[T]) -> Result<f64, &str>
    where T: Num + Convert + Copy + FromPrimitive
{
    let n = data.iter().len();

    match n {
        0 => Err("Vector or Array is empty"),
        _ => {
            let mean = FromPrimitive::from_f64(get_arithmetic_mean(data).unwrap());
            let a = data.iter().fold(T::zero(), |sum, x| sum + pow((*x - mean.unwrap()), 2));
            Ok(a.to_f64() / (n - 1).to_f64())
        }
    }
}

/// Calculates the standard deviation of a slice of Numbers
/// # Remarks
/// Returns a f64 value if the slice contains numbers or an error message if the slice is empty
/// # Examples
/// ```
/// use mathol::statistics::get_standard_deviation;
///
/// let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
/// assert_eq!(0.18516401995451018, get_standard_deviation(&vec).unwrap());
///
/// let vec = vec![8, 6, 5, 11, 6, 6];
/// assert_eq!(2.1908902300206643, get_standard_deviation(&vec).unwrap());
/// ```
pub fn get_standard_deviation<T>(data: &[T]) -> Result<f64, &str>
    where T: Num + Convert + Copy + FromPrimitive
{
    let n = data.iter().len();

    match n {
        0 => Err("Vector or Array is empty"),
        _ => Ok(get_variance(data).unwrap().sqrt()),
    }
}

/// # Remarks
/// Returns the minimum value of a slice or an error message it the slice is empty
/// # Examples
/// ```
/// use mathol::statistics::get_min;
///
/// let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
/// assert_eq!(9.8, get_min(&vec).unwrap());
///
/// let vec = vec![8, 6, 5, 11, 6, 6];
/// assert_eq!(5, get_min(&vec).unwrap());
/// ```
pub fn get_min<T>(data: &[T]) -> Result<T, &str>
    where T: Num + Convert + Copy + FromPrimitive + PartialOrd
{
    match data.iter().len() {
        0 => Err("Vector or Array is empty"),
        _ => {
            let min = data.iter().fold(data[0], |mut min, x| {
                if *x < min {
                    min = *x;
                }

                min
            });

            Ok(min)
        }
    }
}

/// # Remarks
/// Returns the maximum value of a slice or an error message it the slice is empty
/// # Examples
/// ```
/// use mathol::statistics::get_max;
///
/// let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
/// assert_eq!(10.3, get_max(&vec).unwrap());
///
/// let vec = vec![8, 6, 5, 11, 6, 6];
/// assert_eq!(11, get_max(&vec).unwrap());
/// ```
pub fn get_max<T>(data: &[T]) -> Result<T, &str>
    where T: Num + Convert + Copy + FromPrimitive + PartialOrd
{
    match data.iter().len() {
        0 => Err("Vector or Array is empty"),
        _ => {
            let max = data.iter().fold(data[0], |mut max, x| {
                if *x > max {
                    max = *x;
                }

                max
            });

            Ok(max)
        }
    }
}

/// # Remarks
/// Returns the difference between the minimum and maximum of a slice or an error message it the slice is empty
/// # Examples
/// ```
/// use mathol::statistics::get_span;
///
/// let vec = vec![9.8, 10.1, 10.3, 10.2, 10.2, 10.0, 9.9, 10.3];
/// assert_eq!(0.5, get_span(&vec).unwrap());
///
/// let vec = vec![8, 6, 5, 11, 6, 6];
/// assert_eq!(6, get_span(&vec).unwrap());
/// ```
pub fn get_span<T>(data: &[T]) -> Result<T, &str>
    where T: Num + Convert + Copy + FromPrimitive + PartialOrd
{
    match data.iter().len() {
        0 => Err("Vector or Array is empty"),
        _ => Ok(get_max(data).unwrap() - get_min(data).unwrap()),
    }
}
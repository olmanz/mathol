pub mod convert_trait;
pub mod amount_trait;
pub mod cotangent;

use num::Num;
use basics::convert_trait::Convert;


/// Custom function for calculating an exponentiation.
/// # Remarks
/// Returns the result of the exponentiation
/// # Example
/// ```
/// use mathol::basics::pow;
///
/// let result = pow(3, 2);
/// assert_eq!(9, result);
///
/// let result = pow(2.5, 2);
/// assert_eq!(6.25, result);
/// ```
pub fn pow<T, U>(base: T, exponent: U) -> T
    where T: Num + Copy, U: Num
{
    if exponent == U::zero() {
        T::one()
    } else {
        base * pow(base, exponent - U::one())
    }
}

/// Pythagorean theorem in 2D euclidean space functionality
/// # Remarks
/// Takes values a and b and returns c according to c = sqrt(a² + b²)
/// # Examples
/// ```
/// use mathol::basics::pythagoras2d;
///
/// let c = pythagoras2d(3, 4);
/// assert_eq!(5.0, c);
/// ```
pub fn pythagoras2d<T>(a: T, b: T) -> f64
    where T: Num + Convert + Copy
{
    ((a * a) + (b * b)).to_f64().sqrt()
}

/// Pythagorean theorem in 3D euclidean space functionality
/// # Remarks
/// Takes values a, b and c and returns d according to d = sqrt(a² + b² + c²)
/// # Examples
/// ```
/// use mathol::basics::pythagoras3d;
///
/// let c = pythagoras2d(3, 4, 5);
/// assert_eq!(7.071067812, c);
/// ```
pub fn pythagoras3d<T>(a: T, b: T, c: T) -> f64
    where T: Num + Convert + Copy
{
    ((a * a) + (b * b) + (c * c)).to_f64().sqrt()
}
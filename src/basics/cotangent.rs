/// Trait for calculating the cotangent
/// # Remarks
/// cot(x) is defined as cos(x) / sin(x).
/// Returns the calculated cotangent as a radian value.
/// # Examples
/// ```
/// use basics::cotangent::Cotangent;
///
/// let a = 2.7;
/// assert_eq!(-2.115383021, a.cot());
/// ```
pub trait Cotangent {
    fn cot(self) -> f64;
}

impl Cotangent for f64 {
    fn cot(self) -> f64 {
        println!("self: {}", self);
        self.cos() / self.sin()
    }
}
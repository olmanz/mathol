/// Trait for getting the amount of a numeric value.
/// # Remarks
/// Amount is the unsigned value of any number. For example, 3 is the amount of 3 and 5.2 is the amount of -5.2.
/// Gets implemented for every signed numeric type: All integer types and all float types.
/// # Examples
/// ```
/// use mathol::basics::amount_trait::Amount;
///
/// let a = 3;
/// let b = -5.2;
/// assert_eq!(3, a.get_amount());
/// assert_eq!(5.2, b.get_amount());
/// ```
pub trait Amount<T> {
    fn get_amount(self) -> T;
}

impl Amount<i8> for i8 {
    fn get_amount(self) -> i8 {
        if self.is_negative() {
            self * (-1)
        } else {
            self
        }
    }
}

impl Amount<i16> for i16 {
    fn get_amount(self) -> i16 {
        if self.is_negative() {
            self * (-1)
        } else {
            self
        }
    }
}

impl Amount<i32> for i32 {
    fn get_amount(self) -> i32 {
        if self.is_negative() {
            self * (-1)
        } else {
            self
        }
    }
}

impl Amount<i64> for i64 {
    fn get_amount(self) -> i64 {
        if self.is_negative() {
            self * (-1)
        } else {
            self
        }
    }
}

impl Amount<isize> for isize {
    fn get_amount(self) -> isize {
        if self.is_negative() {
            self * (-1)
        } else {
            self
        }
    }
}

impl Amount<f32> for f32 {
    fn get_amount(self) -> f32 {
        if self.is_sign_negative() {
            self * (-1.0)
        } else {
            self
        }
    }
}

impl Amount<f64> for f64 {
    fn get_amount(self) -> f64 {
        if self.is_sign_negative() {
            self * (-1.0)
        } else {
            self
        }
    }
}
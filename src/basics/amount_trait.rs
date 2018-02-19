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
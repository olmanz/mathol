extern crate num;
use self::num::{Num};

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn get_distance(p: &Point, q: &Point) -> f64 {
    ((p.x - q.x) * (p.x - q.x) + (p.y - q.y) * (p.y - q.y)).sqrt()
}

pub fn pow<T, U>(base: T, exponent: U) -> T
    where T: Num + Copy, U: Num
{
    if exponent == U::zero() {
        T::one()
    } else {
        base * pow(base, exponent - U::one())
    }
}

pub fn upow(base: u64, exponent: u64) -> u64 {
    if exponent == 0 {
        1
    } else {
        base * upow(base, exponent - 1)
    }
}

pub fn pythagoras2d(a: f64, b: f64) -> f64 {
    ((a * a) + (b * b)).sqrt()
}

pub fn pythagoras3d(a: f64, b: f64, c: f64) -> f64 {
    ((a * a) + (b * b) + (c * c)).sqrt()
}

pub trait Convert {
    fn to_usize(self) -> usize;
    fn to_f64(self) -> f64;
}

impl Convert for i8 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Convert for i16 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Convert for i32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Convert for i64 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Convert for isize {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Convert for u8 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Convert for u16 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Convert for u32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Convert for u64 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Convert for usize {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn to_usize(self) -> usize {
        self
    }
}

impl Convert for f32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Convert for f64 {
    fn to_f64(self) -> f64 {
        self
    }
    fn to_usize(self) -> usize {
        self as usize
    }
}

pub trait Amount<T> {
    fn get_amount(self) ->T;
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
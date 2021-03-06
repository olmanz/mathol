/// Trait for Converting between numeric values.
/// # Remarks
/// As of now, for this crate it is only relevant to convert to f64 or usize.
/// If necessary, more conversions can be added.
/// # Examples
/// ```
/// use mathol::basics::convert_trait::Convert;
///
/// let a: i32 = 9;
/// assert_eq!(9.0, a.to_f64());
/// ```
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
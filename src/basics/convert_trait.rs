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
pub mod basic {
    pub fn pow(base: i64, mut exponent: i64) -> i64 {
        let mut result: i64 = 1;

        while exponent > 0 {
            result = result * base;
            exponent -= 1;
        }

        result
    }
}
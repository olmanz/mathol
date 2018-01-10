pub fn pow(base: i64, mut exponent: i64) -> i64 {
    if exponent == 0 {
        1
    } else {
        base * pow(base, exponent - 1)
    }
}
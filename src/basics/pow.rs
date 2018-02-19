use num::Num;

pub fn pow<T, U>(base: T, exponent: U) -> T
    where T: Num + Copy, U: Num
{
    if exponent == U::zero() {
        T::one()
    } else {
        base * pow(base, exponent - U::one())
    }
}
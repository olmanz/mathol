use num::Num;
use basics::convert_trait::Convert;

pub fn pythagoras2d<T>(a: T, b: T) -> f64
    where T: Num + Convert + Copy
{
    ((a * a) + (b * b)).to_f64().sqrt()
}

pub fn pythagoras3d<T>(a: T, b: T, c: T) -> f64
    where T: Num + Convert + Copy
{
    ((a * a) + (b * b) + (c * c)).to_f64().sqrt()
}
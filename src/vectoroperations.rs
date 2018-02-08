extern crate num;
use self::num::Num;
use basic::{Convert, pow};

pub trait Vectoroperations {
    fn get_length(&self) -> Result<f64, &str>;
}

impl<T> Vectoroperations for Vec<T>
    where T: Num + Copy + Convert
{
    fn get_length(&self) -> Result<f64, &str> {
        match self.len() {
            0 => Err("Vector is empty"),
            _ => Ok(self.into_iter().fold(T::zero(), |sum, x| sum + pow(*x, 2)).to_f64().sqrt()),
        }
    }
}
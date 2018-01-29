extern crate num;
use self::num::{PrimInt, Integer};
use std::iter::Sum;
use basic::ConvertTof64;

pub struct Sample<T>
    where T: PrimInt + Integer + ConvertTof64
{
    pub number_of_elements: T,
    pub absolute_frequency: Vec<T>,
    pub relative_frequency: Vec<f64>,
    pub distribution_function: Vec<f64>,
}

impl<T> Sample<T>
    where T: PrimInt + Integer + ConvertTof64 + Sum
{
    pub fn build(number_of_elements: T, absolute_frequency: &Vec<T>) -> Sample<T> {
        let sum: T = absolute_frequency.to_vec().into_iter().sum();
        if sum != number_of_elements {
            panic!("Sum of sample units must equal number_of_elements");
        }

        let (relative_frequency, distribution_function) = get_relative_frequency(&number_of_elements, &absolute_frequency);

        Sample {
            relative_frequency,
            distribution_function,
            number_of_elements,
            absolute_frequency: absolute_frequency.to_vec(),
        }
    }
}

fn get_relative_frequency<T>(number_of_elements: &T, absolute_frequency: &Vec<T>) -> (Vec<f64>, Vec<f64>)
    where T: PrimInt + Integer + ConvertTof64
{
    absolute_frequency.iter().fold((Vec::new(), Vec::new()), |(mut vec1, mut vec2), x| {
        let result = x.to_f64().unwrap() / number_of_elements.to_f64().unwrap();
        vec1.push(result);
        vec2.push(vec1.iter().sum());
        (vec1, vec2)
    })
}
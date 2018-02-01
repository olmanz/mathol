extern crate num;
use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
//use std::ops::Add;
use self::num::{Num, FromPrimitive, Float};
//use std::ops::Div;
use basic::{pow, Convert};

pub struct Sample {
    pub absolute_frequency: usize,
    pub relative_frequency: f64,
}

pub fn get_sample<T>(data: &Vec<T>) -> HashMap<T, Sample>
    where T: Num + Convert + Copy + Eq + Hash
{
    let n = data.iter().len();
    let mut map: HashMap<T, Sample> = HashMap::new();
    for x in data {
        let absolute_frequency = data.iter().filter(|s| *s == x).count();
        let relative_frequency = absolute_frequency.to_f64() / n.to_f64();
        let sample = Sample {absolute_frequency, relative_frequency};
        map.insert(*x, sample);
    }

    map
}

pub fn get_mean<T>(data: &Vec<T>) -> f64
    where T: Num + Convert + Copy + FromPrimitive
{
    let n = data.iter().len();
    let sum = data.iter().fold(T::zero(), |sum, x| sum + *x);

    sum.to_f64() / n.to_f64()
}

pub fn get_variance<T>(data: &Vec<T>) -> f64
    where T: Num + Convert + Copy + FromPrimitive
{
    let n = data.iter().len();
    let mean = FromPrimitive::from_f64(get_mean(data)).unwrap();
    let a = data.iter().fold(T::zero(), |sum, x| sum + pow((*x - mean), 2));
    a.to_f64() / (n - 1).to_f64()
}

pub fn get_standard_deviation<T>(data: &Vec<T>) -> f64
    where T: Num + Convert + Copy + FromPrimitive
{
    get_variance(data).sqrt()
}
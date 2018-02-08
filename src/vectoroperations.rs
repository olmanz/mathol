extern crate num;
use self::num::Num;
use basic::{Convert, pow};

pub struct Vector<T>
    where T: Num + Copy + Convert
{
    pub x: T,
    pub y: T,
    pub z: T,
}

pub trait Vectoroperations<T>
    where T: Num + Copy + Convert
{
    fn get_length(&self) -> f64;
    fn get_direction_angle(&self) -> (f64, f64, f64);
    fn multiply_with_scalar(&self, lambda: T) -> Vector<T>;
    fn get_scalar_product(&self, vec: &Vector<T>) -> T;
    fn get_cut_angle(&self, vec: &Vector<T>) -> f64;
    fn get_vector_product(&self, vec: &Vector<T>) -> Vector<T>;
    fn get_triple_product(&self, vec_1: &Vector<T>, vec_2: &Vector<T>) -> T;
}

impl<T> Vectoroperations<T> for Vector<T>
    where T: Num + Copy + Convert
{
    fn get_length(&self) -> f64{
        (pow(self.x, 2) + pow(self.y, 2) + pow(self.z, 2)).to_f64().sqrt()
    }

    fn get_direction_angle(&self) -> (f64, f64, f64) {
        let n = self.get_length().to_f64();

        let alpha = (self.x.to_f64() / n).acos();
        let beta = (self.y.to_f64() / n).acos();
        let gamma = (self.z.to_f64() / n).acos();

        (alpha, beta, gamma)
    }

    fn multiply_with_scalar(&self, lambda: T) -> Vector<T> {
        Vector {x: lambda * self.x, y: lambda * self.y, z: lambda * self.z}
    }

    fn get_scalar_product(&self, vec: &Vector<T>) -> T {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    fn get_cut_angle(&self, vec: &Vector<T>) -> f64 {
        self.get_scalar_product(vec).to_f64() / (self.get_length().to_f64() * vec.get_length().to_f64())
    }

    fn get_vector_product(&self, vec: &Vector<T>) -> Vector<T> {
        Vector {
            x: self.y * vec.z - self.z * vec.y,
            y: self.z * vec.x - self.x * vec.z,
            z: self.x * vec.y - self.y * vec.x,
        }
    }

    fn get_triple_product(&self, vec_1: &Vector<T>, vec_2: &Vector<T>) -> T {
        self.get_scalar_product(&vec_1.get_vector_product(vec_2))
    }
}
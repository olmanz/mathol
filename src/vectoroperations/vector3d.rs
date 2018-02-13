use num::Num;
use basic::{Convert, Amount, pow};
use std::fmt::Debug;
use vectoroperations::vectortraits::Vectoroperations;

#[derive(Debug)]
pub struct Vector3D<T>
    where T: Num + Copy + Convert + Amount<T>
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3D<T>
    where T: Num + Copy + Convert + Amount<T> + Debug
{
    pub fn build_vector(x: T, y: T, z: T) -> Vector3D<T> {
        Vector3D {x, y, z}
    }
}

impl<T> Vectoroperations<T> for Vector3D<T>
    where T: Num + Copy + Convert + Amount<T> + Debug
{
    fn add_vector(&self, vec: &Vector3D<T>) -> Vector3D<T> {
        Vector3D::build_vector(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }

    fn sub_vector(&self, vec: &Vector3D<T>) -> Vector3D<T> {
        Vector3D::build_vector(self.x - vec.x, self.y - vec.y, self.z - vec.z)
    }

    fn get_length(&self) -> f64 {
        (pow(self.x, 2) + pow(self.y, 2) + pow(self.z, 2)).to_f64().sqrt()
    }

    fn get_direction_angle(&self) -> (f64, f64, f64) {
        let n = self.get_length().to_f64();

        let alpha = (self.x.to_f64() / n).acos();
        let beta = (self.y.to_f64() / n).acos();
        let gamma = (self.z.to_f64() / n).acos();

        (alpha, beta, gamma)
    }

    fn multiply_with_scalar(&self, lambda: T) -> Vector3D<T> {
        Vector3D::build_vector(lambda * self.x, lambda * self.y, lambda * self.z)
    }

    fn get_scalar_product(&self, vec: &Vector3D<T>) -> T {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    fn get_cut_angle(&self, vec: &Vector3D<T>) -> f64 {
        self.get_scalar_product(vec).to_f64() / (self.get_length().to_f64() * vec.get_length().to_f64())
    }

    fn get_vector_product(&self, vec: &Vector3D<T>) -> Vector3D<T> {
        Vector3D::build_vector(self.y * vec.z - self.z * vec.y, self.z * vec.x - self.x * vec.z, self.x * vec.y - self.y * vec.x)
    }

    fn get_triple_product(&self, vec_1: &Vector3D<T>, vec_2: &Vector3D<T>) -> T {
        self.x * (vec_1.y * vec_2.z - vec_1.z * vec_2.y) + self.y * (vec_1.z * vec_2.x - vec_1.x * vec_2.z) + self.z * (vec_1.x * vec_2.y - vec_1.y * vec_2.x)
    }
}
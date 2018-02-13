use num::Num;
use basic::{Convert, Amount};
use std::fmt::Debug;
use vectoroperations::vector3d::Vector3D;

pub trait Vectoroperations<T>
    where T: Num + Copy + Convert + Amount<T> + Debug
{
    fn add_vector(&self, vec: &Vector3D<T>) -> Vector3D<T>;
    fn sub_vector(&self, vec: &Vector3D<T>) -> Vector3D<T>;
    fn get_length(&self) -> f64;
    fn get_direction_angle(&self) -> (f64, f64, f64);
    fn multiply_with_scalar(&self, lambda: T) -> Vector3D<T>;
    fn get_scalar_product(&self, vec: &Vector3D<T>) -> T;
    fn get_cut_angle(&self, vec: &Vector3D<T>) -> f64;
    fn get_vector_product(&self, vec: &Vector3D<T>) -> Vector3D<T>;
    fn get_triple_product(&self, vec_1: &Vector3D<T>, vec_2: &Vector3D<T>) -> T;
}
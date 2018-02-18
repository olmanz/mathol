use num::Num;
use basic::{Convert, Amount, pow};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Vector2D<T>
    where T: Num + Copy + Convert + Amount<T>
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T>
    where T: Num + Copy + Convert + Amount<T> + Debug
{
    pub fn build_vector(x: T, y: T) -> Vector2D<T> {
        Vector2D {x, y}
    }

    pub fn get_distance(&self, other: &Vector2D<T>) -> f64 {
        (pow(self.x - other.x, 2) + pow(self.y - other.y, 2)).to_f64().sqrt()
    }

    pub fn add_vector(&self, vec: &Vector2D<T>) -> Vector2D<T> {
        Vector2D::build_vector(self.x + vec.x, self.y + vec.y)
    }

    pub fn sub_vector(&self, vec: &Vector2D<T>) -> Vector2D<T> {
        Vector2D::build_vector(self.x - vec.x, self.y - vec.y)
    }

    pub fn get_length(&self) -> f64 {
        (pow(self.x, 2) + pow(self.y, 2)).to_f64().sqrt()
    }

//    pub fn get_direction_angle(&self) -> (f64, f64, f64) {
//        unimplemented!()
//    }

    pub fn multiply_with_scalar(&self, lambda: T) -> Vector2D<T> {
        Vector2D::build_vector(lambda * self.x, lambda * self.y)
    }

    pub fn get_scalar_product(&self, vec: &Vector2D<T>) -> T {
        self.x * vec.x + self.y * vec.y
    }

    pub fn get_cut_angle(&self, vec: &Vector2D<T>) -> f64 {
        (self.get_scalar_product(&vec).to_f64() / (self.get_length() * vec.get_length())).acos()
    }

    pub fn get_vector_product(&self, vec: &Vector2D<T>) -> T {
        self.x * vec.y - self.y * vec.x
    }
}
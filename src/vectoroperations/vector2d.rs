use num::{Num, FromPrimitive};
use std::fmt::Debug;
use basics::{pow, pythagoras2d};
use basics::convert_trait::Convert;
use basics::amount_trait::Amount;

#[derive(Debug, Copy, Clone)]
pub struct Vector2D<T>
    where T: Num + Copy + Convert + Amount<T> + FromPrimitive
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T>
    where T: Num + Copy + Convert + Amount<T> + Debug + FromPrimitive
{
    pub fn build_vector(x: T, y: T) -> Vector2D<T> {
        Vector2D {x, y}
    }

    /// Transforms cartesic coordinates to polar coordinates.
    /// Returns an instance of the struct Polar
    pub fn transform_to_polar(self) -> Polar<f64> {
        Polar {
            r: pythagoras2d(self.x, self.y),
            phi: self.y.to_f64().atan2(self.x.to_f64()).to_degrees(),
        }
    }

    pub fn get_distance(self, other: Vector2D<T>) -> f64 {
        (pow(self.x - other.x, 2) + pow(self.y - other.y, 2)).to_f64().sqrt()
    }

    pub fn add_vector(self, vec: Vector2D<T>) -> Vector2D<T> {
        Vector2D::build_vector(self.x + vec.x, self.y + vec.y)
    }

    pub fn sub_vector(self, vec: Vector2D<T>) -> Vector2D<T> {
        Vector2D::build_vector(self.x - vec.x, self.y - vec.y)
    }

    pub fn get_length(self) -> f64 {
        (pow(self.x, 2) + pow(self.y, 2)).to_f64().sqrt()
    }

    pub fn multiply_with_scalar(self, lambda: T) -> Vector2D<T> {
        Vector2D::build_vector(lambda * self.x, lambda * self.y)
    }

    pub fn get_scalar_product(self, vec: Vector2D<T>) -> T {
        self.x * vec.x + self.y * vec.y
    }

    pub fn get_cut_angle(self, vec: Vector2D<T>) -> f64 {
        (self.get_scalar_product(vec).to_f64() / (self.get_length() * vec.get_length())).acos()
    }

    pub fn get_vector_product(self, vec: Vector2D<T>) -> T {
        self.x * vec.y - self.y * vec.x
    }
}


/// Rust struct for points in the two-dimensional polar coordinate system.
#[derive(Debug, Copy, Clone)]
pub struct Polar<T>
    where T: Num + Copy + Convert + Amount<T> + FromPrimitive
{
    /// radius (distance of the point from the pole)
    pub r: T,
    /// azimuth (angle to x-axis)
    pub phi: T,
}

impl<T> Polar<T>
    where T: Num + Copy + Convert + Amount<T> + FromPrimitive
{
    pub fn build_polar(r: T, phi: T) -> Polar<T> {
        Polar {
            r,
            phi,
        }
    }

    /// Transforms polar coordinates to cartesic coordinates.
    /// Returns an instance of the struct Vector2D
    pub fn transform_to_vector2d(self) -> Vector2D<f64> {
        Vector2D {
            x: self.r.to_f64() * self.phi.to_f64().to_radians().cos(),
            y: self.r.to_f64()* self.phi.to_f64().to_radians().sin(),
        }
    }
}
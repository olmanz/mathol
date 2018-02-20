use num::{Num, FromPrimitive};
use std::fmt::Debug;
use basics::{pow, pythagoras2d};
use basics::convert_trait::Convert;
use basics::amount_trait::Amount;


/// Struct for two-dimensional vectors
/// # Usage
/// ```
/// use mathol::vectoroperations::vector3d::Vector3D;
/// ```
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
    /// Takes x and y values and returns a Vector2D instance
    /// # Examples
    /// ```
    /// let vector = Vector3D::build_vector(2, 2);
    /// ```
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

    /// Calculates the distance between two points
    /// # Parameters
    /// other: The second point
    /// # Return values
    /// Returns the distance as f64 value
    /// # Examples
    /// ```
    /// let p = Vector2D::build_vector(1, 2);
    /// let q = Vector2D::build_vector(4, 6);
    /// assert_eq!(5.0, p.get_distance(q);
    /// ```
    pub fn get_distance(self, other: Vector2D<T>) -> f64 {
        (pow(self.x - other.x, 2) + pow(self.y - other.y, 2)).to_f64().sqrt()
    }

    /// Adds a vector to another vector
    /// # Remarks
    /// Returns the result of the addition as a new vector
    /// # Examples
    /// ```
    /// let vector_1 = Vector2D::build_vector(2, 1);
    /// let vector_2 = Vector2D::build_vector(3, 5);
    /// let vector_3 = vector_1.add_vector(vector_2);
    /// assert_eq!(5, vector_3.x);
    /// assert_eq!(6, vector_3.y);
    /// ```
    pub fn add_vector(self, vec: Vector2D<T>) -> Vector2D<T> {
        Vector2D::build_vector(self.x + vec.x, self.y + vec.y)
    }

    /// Subtracts a vector from another vector
    /// # Remarks
    /// Returns the result of the subtraction as a new vector
    /// # Examples
    /// ```
    /// let vector_1 = Vector2D::build_vector(2, 1);
    /// let vector_2 = Vector2D::build_vector(3, 5);
    /// let vector_3 = vector_1.add_vector(vector_2);
    /// assert_eq!(-1, vector_3.x);
    /// assert_eq!(-4, vector_3.y);
    /// ```
    pub fn sub_vector(self, vec: Vector2D<T>) -> Vector2D<T> {
        Vector2D::build_vector(self.x - vec.x, self.y - vec.y)
    }

    /// Calculates the length of a vector
    /// # Remarks
    /// Returns the length as an f64 value
    /// # Examples
    /// ```
    /// let vector = Vector2D::build_vector(3, 4);
    /// assert_eq!(5.0, vector.get_length());
    /// ```
    pub fn get_length(self) -> f64 {
        (pow(self.x, 2) + pow(self.y, 2)).to_f64().sqrt()
    }

    /// Multiplies a vector with a scalar value
    /// # Remarks
    /// Returns the result of the multiplication as a new vector
    /// # Examples
    /// ```
    /// let vector_1 = Vector2D::build_vector(2.7, 3.6);
    /// let vector_2 = vector_1.multiply_with_scalar(2.0)
    /// assert_eq!(5.4, vector_2.x);
    /// assert_eq!(7.2, vector_2.y);
    /// ```
    pub fn multiply_with_scalar(self, lambda: T) -> Vector2D<T> {
        Vector2D::build_vector(lambda * self.x, lambda * self.y)
    }

    /// Calculates the scalar product of two vectors
    /// # Remarks
    /// Returns the result as numeric value
    /// # Examples
    /// ```
    /// let vector_1 = Vector3D::build_vector(2, 3, 4);
    /// let vector_2 = Vector3D::build_vector(5, 6, 7);
    /// assert_eq!(56, vector_1.get_scalar_product(vector_2));
    /// ```
    pub fn get_scalar_product(self, vec: Vector2D<T>) -> T {
        self.x * vec.x + self.y * vec.y
    }

    /// Calculates the angle between two crossing vectors
    /// # Remarks
    /// Returns the cut angle as radian value
    /// # Examples
    /// ```
    /// let vec_1 = Vector2D::build_vector(4, 3);
    /// let vec_2 = Vector2D::build_vector(-3, 2);
    /// assert_eq!(1.9100889412489412, vec_1.get_cut_angle(vec_2));
    /// ```
    pub fn get_cut_angle(self, vec: Vector2D<T>) -> f64 {
        (self.get_scalar_product(vec).to_f64() / (self.get_length() * vec.get_length())).acos()
    }

    /// Calculates the vector product of two vectors
    /// # Remarks
    /// Returns the result as a new vector
    /// # Examples
    /// ```
    /// let vector_1 = Vector2D::build_vector(1, 4);
    /// let vector_2 = Vector2D::build_vector(-2, 5);
    /// assert_eq!(13, vector_1.get_vector_product(vector_2));
    /// ```
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
use num::Num;
use basic::{Convert, Amount, pow};
use std::fmt::Debug;

/// Struct for three-dimensional vectors
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
    /// Takes x, y and z values and returns a Vector3D instance
    /// # Examples
    /// ```
    /// let vector = Vector3D::build_vector(2, 2, 1);
    /// ```
    pub fn build_vector(x: T, y: T, z: T) -> Vector3D<T> {
        Vector3D {x, y, z}
    }

    /// Adds a vector to another vector
    /// # Remarks
    /// Returns the result of the addition as a new vector
    /// # Examples
    /// ```
    /// let vector_1 = Vector3D::build_vector(2, 2, 1);
    /// let vector_2 = Vector3D::build_vector(2, 3, 5);
    /// let vector_3 = vector_1.add_vector(&vector_2);
    /// assert_eq!(4, vector_3.x);
    /// assert_eq!(5, vector_3.y);
    /// assert_eq!(6, vector_3.z);
    /// ```
    pub fn add_vector(&self, vec: &Vector3D<T>) -> Vector3D<T> {
        Vector3D::build_vector(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }

    /// Subtracts a vector from another vector
    /// # Remarks
    /// Returns the result of the subtraction as a new vector
    /// # Examples
    /// ```
    /// let vector_1 = Vector3D::build_vector(2, 2, 1);
    /// let vector_2 = Vector3D::build_vector(2, 3, 5);
    /// let vector_3 = vector_1.add_vector(&vector_2);
    /// assert_eq!(0, vector_3.x);
    /// assert_eq!(-1, vector_3.y);
    /// assert_eq!(-4, vector_3.z);
    /// ```
    pub fn sub_vector(&self, vec: &Vector3D<T>) -> Vector3D<T> {
        Vector3D::build_vector(self.x - vec.x, self.y - vec.y, self.z - vec.z)
    }

    /// Calculates the length of a vector
    /// # Remarks
    /// Returns the length as an f64 value
    /// # Examples
    /// ```
    /// let vector_1 = Vector3D::build_vector(2, 3, 4);
    /// assert_eq!(5.385164807134504, vector_1.get_length());
    ///
    /// let vector_2 = Vector3D::build_vector(2.7, 3.6, 4.5);
    /// assert_eq!(6.363961030678928, vector_2.get_length());
    /// ```
    pub fn get_length(&self) -> f64 {
        (pow(self.x, 2) + pow(self.y, 2) + pow(self.z, 2)).to_f64().sqrt()
    }

    /// Calculates the three direction angles alpha, beta and gamma of a three-dimensional vector
    /// # Remarks
    /// Returns a tuple with the three angles as radians
    ///
    /// alpha is the angle of the vector to the x-axis
    ///
    /// beta is the angle of the vector to the y-axis
    ///
    /// gamma is the angle of the vector to the z-axis
    /// # Examples
    /// ```
    /// let vector = Vector3D::build_vector(4, -2, 5);
    /// (alpha, beta, gamma) = vector.get_direction_angle();
    /// assert_eq!(0.9319311825594854, alpha);
    /// assert_eq!(1.873542278417901, beta);
    /// assert_eq!(0.7297276562269663, gamma);
    /// ```
    pub fn get_direction_angle(&self) -> (f64, f64, f64) {
        let n = self.get_length().to_f64();

        let alpha = (self.x.to_f64() / n).acos();
        let beta = (self.y.to_f64() / n).acos();
        let gamma = (self.z.to_f64() / n).acos();

        (alpha, beta, gamma)
    }

    /// Multiplies a vector with a scalar value
    /// # Remarks
    /// Returns the result of the multiplication as a new vector
    /// # Examples
    /// ```
    /// let vector_1 = Vector3D::build_vector(2.7, 3.6, 4.5);
    /// let vector_2 = vector_1.multiply_with_scalar(2.0)
    /// assert_eq!(5.4, vector_2.x);
    /// assert_eq!(7.2, vector_2.y);
    /// assert_eq!(9.0, vector_2.z);
    /// ```
    pub fn multiply_with_scalar(&self, lambda: T) -> Vector3D<T> {
        Vector3D::build_vector(lambda * self.x, lambda * self.y, lambda * self.z)
    }

    /// Calculates the scalar product of two vectors
    /// # Remarks
    /// Returns the result as numeric value
    /// # Examples
    /// ```
    /// let vector_1 = Vector3D::build_vector(2, 3, 4);
    /// let vector_2 = Vector3D::build_vector(5, 6, 7);
    /// assert_eq!(56, vector_1.get_scalar_product(&vector_2));
    /// ```
    pub fn get_scalar_product(&self, vec: &Vector3D<T>) -> T {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    /// Calculates the angle between two crossing vectors
    /// # Remarks
    /// Returns the cut angle as radian value
    /// # Examples
    /// ```
    /// let vector_1 = Vector3D::build_vector(1, 2, -3);
    /// let vector_2 = Vector3D::build_vector(5, -1, -5);
    /// assert_eq!(0.6736330697086078, vector_1.get_cut_angle(&vector_2));
    /// ```
    pub fn get_cut_angle(&self, vec: &Vector3D<T>) -> f64 {
        self.get_scalar_product(vec).to_f64() / (self.get_length().to_f64() * vec.get_length().to_f64())
    }

    /// Calculates the vector product of two vectors
    /// # Remarks
    /// Returns the result as a new vector
    /// # Examples
    /// ```
    /// let vector_1 = Vector3D::build_vector(1, 4, 0);
    /// let vector_2 = Vector3D::build_vector(-2, 5, 3);
    /// let vec_product = vector_1.get_vector_product(&vector_2);
    /// assert_eq!(12, vec_product.x);
    /// assert_eq!(-3, vec_product.y);
    /// assert_eq!(13, vec_product.z);
    /// ```
    pub fn get_vector_product(&self, vec: &Vector3D<T>) -> Vector3D<T> {
        Vector3D::build_vector(self.y * vec.z - self.z * vec.y, self.z * vec.x - self.x * vec.z, self.x * vec.y - self.y * vec.x)
    }

    /// Calculates the triple product of three vectors
    /// # Remarks
    /// Returns the result as numeric value
    /// # Examples
    /// ```
    /// let vector_1 = Vector3D::build_vector(1, -2, 4);
    /// let vector_2 = Vector3D::build_vector(4, 1, 2);
    /// let vector_3 = Vector3D::build_vector(-2, -5, -6);
    /// assert_eq!(0, vector_1.get_triple_product(&vector_2, &vector_3));
    /// ```
    pub fn get_triple_product(&self, vec_1: &Vector3D<T>, vec_2: &Vector3D<T>) -> T {
        self.x * (vec_1.y * vec_2.z - vec_1.z * vec_2.y) + self.y * (vec_1.z * vec_2.x - vec_1.x * vec_2.z) + self.z * (vec_1.x * vec_2.y - vec_1.y * vec_2.x)
    }
}
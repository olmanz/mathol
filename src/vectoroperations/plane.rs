use num::Num;
use std::fmt::Debug;
use basics::convert_trait::Convert;
use basics::amount_trait::Amount;
use vectoroperations::vector3d::Vector3D;
use vectoroperations::line3d::Line3D;
use error::*;

/// A struct for a parametric representation of a plane in three-dimensional space
#[derive(Debug, Copy, Clone)]
pub struct Plane<T>
    where T: Num + Copy + Convert + Amount<T> + PartialOrd + PartialEq
{
    /// The support vector (a point on the plane)
    pub r: Vector3D<T>,
    /// Vector that stands perpendicular to the plane
    pub n: Vector3D<T>
}

impl<T> Plane<T>
    where T: Num + Copy + Convert + Amount<T> + Debug + PartialOrd + PartialEq
{
    /// Builds a line that goes through three given points p, q and r
    /// # Remarks
    /// Return an instance of the struct plane
    /// # Examples
    /// Point p has the coordinates (1 | 1 | 2), Point q has the coordinates (0 | 4 | -5), and Point
    /// r has the coordinates (-3 | 4 | 9). These three points all lie on the same plane.
    ///
    /// The resulting plane has the support vector (1 | 1 | 2). The first direction vector is (-1 | 3 | -7)
    /// and the second direction vector is (-4 | 3 | 7).
    ///
    /// ```
    /// let p = Vector3D::build_vector(1, 1, 2);
    /// let q = Vector3D::build_vector(0, 4, -5);
    /// let r = Vector3D::build_vector(-3, 4, 9);
    /// let vec = Plane::build_plane_from_three_points(p, q, r);
    /// assert_eq!(1, vec.r.x);
    /// assert_eq!(1, vec.r.y);
    /// assert_eq!(2, vec.r.z);
    /// assert_eq!(-1, vec.a.x);
    /// assert_eq!(3, vec.a.y);
    /// assert_eq!(-7, vec.a.z);
    /// assert_eq!(-4, vec.b.x);
    /// assert_eq!(3, vec.b.y);
    /// assert_eq!(7, vec.b.z);
    /// ```
    pub fn build_plane_from_three_points(p: Vector3D<T>, q: Vector3D<T>, r: Vector3D<T>) -> Plane<T> {
        Plane {
            r: Vector3D::build_vector(p.x, p.y, p.z),
            n: q.sub_vector(p).get_vector_product(r.sub_vector(p)),
        }
    }

    pub fn build_plane_with_vectors(r: Vector3D<T>, n: Vector3D<T>) -> Plane<T> {
        Plane {
            r: Vector3D::build_vector(r.x, r.y, r.z),
            n: Vector3D::build_vector(n.x, n.y, n.z),
        }
    }

    /// Calculates the distance between a point and a plane in three-dimensional space
    /// # Remarks
    /// Returns the distance as f64 value
    /// # Examples
    /// ```
    /// let r = Vector3D::build_vector(3.0, 1.0, 8.0);
    /// let a = Vector3D::build_vector(-2.0, 2.0, 1.0);
    /// let b = Vector3D::build_vector(4.5, 3.0, 1.0);
    /// let q = Vector3D::build_vector(1.0, 2.0, 0.0);
    /// let plane = Plane {r, a, b};
    /// assert_eq!(7.845728264713728, plane.get_distance_from_point(q));
    /// ```
    pub fn get_distance_from_point(self, p: Vector3D<T>) -> f64 {
        let r = p.sub_vector(self.r);
        let d = self.n.get_scalar_product(r).to_f64().get_amount() / self.n.get_length().get_amount();
        d
    }

    /// Checks if a line and a plane are parallel to each other
    /// # Remarks
    /// Returns a boolean value
    /// # Examples
    /// ```
    /// let l = Line3D {r: Vector3D::build_vector(1, 2, 3), a: Vector3D::build_vector(4, 2, 2)};
    /// let p = Plane {r: Vector3D::build_vector(2, 3, 5), a: Vector3D::build_vector(2, 1, 1), b: Vector3D::build_vector(1, 3, 4)};
    /// assert_eq!(true, p.is_parallel_to_line(l));
    /// ```
    pub fn is_parallel_to_line(self, l: Line3D<T>) -> bool {
        if l.a.get_scalar_product(self.n).to_f64() == 0.0 {
            true
        } else {
            false
        }
    }

    /// Calculates the distance between a line and a plane if they do not cross
    /// # Remarks
    /// Returns a result value
    ///
    /// If the line and the plane are parallel, the distance between them is returned as an Ok(f64) value
    ///
    /// If the line and the plane do cross, an error message is returned
    /// # Examples
    /// Line and plane are parallel:
    ///
    /// ```
    /// let l = Line3D {r: Vector3D::build_vector(1, 2, 3), a: Vector3D::build_vector(4, 2, 2)};
    /// let p = Plane {r: Vector3D::build_vector(2, 3, 5), a: Vector3D::build_vector(2, 1, 1), b: Vector3D::build_vector(1, 3, 4)};
    /// assert_eq!(Ok(0.46188021535170054), p.get_distance_from_line(l));
    /// ```
    ///
    /// Line and plane are not parallel:
    ///
    /// ```
    /// let l = Line3D {r: Vector3D::build_vector(1, 2, 3), a: Vector3D::build_vector(4, 2, 3)};
    /// let p = Plane {r: Vector3D::build_vector(2, 3, 5), a: Vector3D::build_vector(2, 1, 1), b: Vector3D::build_vector(1, 3, 4)};
    /// assert_eq!(Err("Line is not parallel to plane"), p.get_distance_from_line(l));
    /// ```
    pub fn get_distance_from_line(self, l: Line3D<T>) -> Result<f64, MatholError> {
        if !self.is_parallel_to_line(l) {
            return Err(MatholError::VectorCause(VectorError {
                message: "Line is not parallel to plane".to_string(),
            }));
        }

        let r = l.r.sub_vector(self.r);
        let d = self.n.get_scalar_product(r).to_f64().get_amount() / self.n.get_length().get_amount();
        Ok(d)
    }

    /// Checks if a plane and another plane are parallel to each other
    /// # Remarks
    /// Returns a boolean value
    /// # Examples
    /// ```
    /// let p = Plane {r: Vector3D::build_vector(2, 3, 5), a: Vector3D::build_vector(2, 1, 1), b: Vector3D::build_vector(1, 3, 4)};
    /// let q = Plane {r: Vector3D::build_vector(4, 3, 7), a: Vector3D::build_vector(4, 2, 2), b: Vector3D::build_vector(2, 6, 8)};
    /// assert_eq!(true, p.is_parallel_to_plane(q));
    /// ```
    pub fn is_parallel_to_plane(self, p: Plane<T>) -> bool {
        if self.n.get_vector_product(p.n).get_length().to_f64() == 0.0 {
            true
        } else {
            false
        }
    }

    /// Calculates the distance between a plane and another plane if they do not cross
    /// # Remarks
    /// Returns a result value
    ///
    /// If the planes are parallel, the distance between them is returned as an Ok(f64) value
    ///
    /// If the planes do cross, an error message is returned
    /// # Examples
    /// Planes are parallel:
    ///
    /// ```
    /// let p = Plane {r: Vector3D::build_vector(2, 3, 5), a: Vector3D::build_vector(2, 1, 1), b: Vector3D::build_vector(1, 3, 4)};
    /// let q = Plane {r: Vector3D::build_vector(4, 3, 7), a: Vector3D::build_vector(4, 2, 2), b: Vector3D::build_vector(2, 6, 8)};
    /// assert_eq!(Ok(1.3856406460551016), p.get_distance_from_plane(q));
    /// ```
    ///
    /// Planes are not parallel:
    ///
    /// ```
    /// let p = Plane {r: Vector3D::build_vector(2, 3, 5), a: Vector3D::build_vector(2, 1, 1), b: Vector3D::build_vector(1, 3, 4)};
    /// let q = Plane {r: Vector3D::build_vector(4, 3, 7), a: Vector3D::build_vector(4, 2, 3), b: Vector3D::build_vector(2, 6, 8)};
    /// assert_eq!(Err("The planes are not parallel"), p.get_distance_from_plane(q));
    /// ```
    pub fn get_distance_from_plane(self, p: Plane<T>) -> Result<f64, MatholError> {
        if !self.is_parallel_to_plane(p) {
            return Err(MatholError::VectorCause(VectorError {
                message: "The planes are not parallel".to_string(),
            }));
        }

        let r = self.r.sub_vector(p.r);
        let d = self.n.get_scalar_product(r).to_f64().get_amount() / self.n.get_length().get_amount();
        Ok(d)
    }

    /// Calculates the point where a line cuts through a plane
    /// # Parameters
    /// self: The plane
    ///
    /// l: The line
    /// # Return value
    /// Returns the point as an instance of Vector3D in case of success, otherwise an error message
    /// # Examples
    /// ```
    /// let l = Line3D::build_line_from_two_points(Vector3D::build_vector(2, 0, 5), Vector3D::build_vector(5, -4, 4));
    /// let p = Plane::build_plane_with_vectors(Vector3D::build_vector(1, 1, 2), Vector3D::build_vector(2, 1, 1));
    /// let s = p.get_cutting_point_with_line(l).expect("error");
    /// assert_eq!(-10, s.x);
    /// assert_eq!(16, s.y);
    /// assert_eq!(9, s.z);
    /// ```
    pub fn get_cutting_point_with_line(self, l: Line3D<T>) -> Result<Vector3D<T>, MatholError> {
        if self.is_parallel_to_line(l) {
            return Err(MatholError::VectorCause(VectorError {
                message: "The line is parallel to the plane".to_string(),
            }));
        }

        let a = self.n.get_scalar_product(self.r.sub_vector(l.r));
        let b = self.n.get_scalar_product(l.a);

        Ok(l.r.add_vector(l.a.multiply_with_scalar(a / b)))
    }
}
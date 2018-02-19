use num::Num;
use basics::convert_trait::Convert;
use basics::amount_trait::Amount;
use std::fmt::Debug;
use vectoroperations::vector3d::Vector3D;
use error::*;

/// A struct for a parametric representation of a line in three-dimensional space
#[derive(Debug, Copy, Clone)]
pub struct Line3D<T>
    where T: Num + Copy + Convert + Amount<T> + PartialOrd + PartialEq
{
    /// The support vector (a point on the line)
    pub r: Vector3D<T>,
    /// The direction vector of the line
    pub a: Vector3D<T>,
}

impl<T> Line3D<T>
    where T: Num + Copy + Convert + Amount<T> + Debug + PartialOrd + PartialEq
{
    /// Builds a line that goes through two given points p and q
    /// # Remarks
    /// Return an instance of the struct line
    /// # Examples
    /// Point p has the coordinates (-1 | 5 | 0) and Point q has the coordinates (1 | -3 | 2).
    ///
    /// The resulting line has the support vector (-1 | 5 | 0) and the direction vector (2 | -8 | 2).
    ///
    /// ```
    /// let p = Vector3D::build_vector(-1, 5, 0);
    /// let q = Vector3D::build_vector(1, -3, 2);
    /// let line = Line3D::build_line_from_two_points(&p, &q);
    /// assert_eq!(-1, line.r.x);
    /// assert_eq!(5, line.r.y);
    /// assert_eq!(0, line.r.z);
    /// assert_eq!(2, line.a.x);
    /// assert_eq!(-8, line.a.y);
    /// assert_eq!(2, line.a.z);
    /// ```
    pub fn build_line_from_two_points(p: Vector3D<T>, q: Vector3D<T>) -> Line3D<T> {
        Line3D {
            r: Vector3D::build_vector(p.x, p.y, p.z),
            a: q.sub_vector(p),
        }
    }

    /// Calculates the distance between a point and a line in three-dimensional space
    /// # Remarks
    /// Returns the distance as f64 value
    /// # Examples
    /// ```
    /// let p = Vector3D::build_vector(1, 5, 3);
    /// let l = Line3D {r: Vector3D::build_vector(1, 1, 4), a: Vector3D::build_vector(2, -3, 5)};
    /// assert_eq!(3.0650834967591445, l.distance_from_point(&p));
    /// ```
    pub fn distance_from_point(self, p: Vector3D<T>) -> f64 {
        let r = p.sub_vector(self.r);
        self.a.get_vector_product(r).get_length() / self.a.get_length()
    }

    /// Checks if two lines are parallel to each other
    /// # Remarks
    /// Returns a boolean value
    /// # Examples
    /// ```
    /// let l1 = Line3D {r: Vector3D::build_vector(1, 0, 5), a: Vector3D::build_vector(2, 1, 1)};
    /// let l2 = Line3D {r: Vector3D {x: 0, y: 2, z: 1}, a: Vector3D {x: 2, y: 1, z: 1}};
    /// assert_eq!(true, l1.are_parallel(&l2));
    /// ```
    pub fn are_parallel(self, l: Line3D<T>) -> bool {
        if self.a.get_vector_product(l.a).get_length() == 0.0 {
            true
        } else {
            false
        }
    }

    /// Checks if two lines do cross
    /// # Remarks
    /// Returns a boolean value
    /// # Examples
    /// ```
    /// let l1 = Line3D {r: Vector3D::build_vector(1, 1, 0), a: Vector3D::build_vector(2, 1, 1)};
    /// let l2 = Line3D {r: Vector3D::build_vector(2, 0, 2), a: Vector3D::build_vector(1, -1, 2)};
    /// assert_eq!(true, l1.do_cross(&l2));
    /// ```
    pub fn do_cross(self, l: Line3D<T>) -> bool {
        if !self.are_parallel(l) && self.a.get_triple_product(l.a, l.r.sub_vector(self.r)).to_f64() == 0.0 {
            true
        } else {
            false
        }
    }

    /// Checks if two lines are skew
    /// # Remarks
    /// Returns a boolean value
    /// # Examples
    /// ```
    /// let l1 = Line3D {r: Vector3D::build_vector(5, 2, 1), a: Vector3D::build_vector(1, 1, 3)};
    /// let l2 = Line3D {r: Vector3D::build_vector(2, -1, 0), a: Vector3D::build_vector(3, 2, 1)};
    /// assert_eq!(true, l1.are_skew(&l2));
    /// ```
    pub fn are_skew(self, l: Line3D<T>) -> bool {
        if !self.are_parallel(l) && self.a.get_triple_product(l.a, l.r.sub_vector(self.r)).to_f64() != 0.0 {
            true
        } else {
            false
        }
    }

    /// Returns the distance between two lines that do not cross
    /// # Remarks
    /// Returns a result value
    ///
    /// If the lines are parallel or skew, the distance between them is returned as an Ok(f64) value
    ///
    /// If the lines cross, an error message is returned
    /// # Examples
    /// Lines are parallel:
    ///
    /// ```
    /// let l1 = Line3D {r: Vector3D::build_vector(1, 0, 5), a: Vector3D::build_vector(2, 1, 1)};
    /// let l2 = Line3D {r: Vector3D::build_vector(0, 2, 1), a: Vector3D::build_vector(2, 1, 1)};
    /// assert_eq!(Ok(4.281744192888377), l1.distance_from_line(&l2));
    /// ```
    ///
    /// Lines are skew:
    ///
    /// ```
    /// let l1 = Line3D {r: Vector3D::build_vector(5, 2, 1), a: Vector3D::build_vector(1, 1, 3)};
    /// let l2 = Line3D {r: Vector3D::build_vector(2, -1, 0), a: Vector3D::build_vector(3, 2, 1)};
    /// assert_eq!(Ok(0.8432740427115678), l1.distance_from_line(&l2));
    /// ```
    ///
    /// Lines do cross:
    ///
    /// ```
    /// let l1 = Line3D {r: Vector3D::build_vector(1, 1, 0), a: Vector3D {x: 2, y: 1, z: 1}};
    /// let l2 = Line3D {r: Vector3D {x: 2, y: 0, z: 2}, a: Vector3D {x: 1, y: -1, z: 2}};
    /// assert_eq!(Err("Lines do cross"), l1.distance_from_line(&l2));
    /// ```
    pub fn distance_from_line(self, line: Line3D<T>) -> Result<f64, MatholError> {
        let r = line.r.sub_vector(self.r);
        if self.are_parallel(line) {
            Ok(self.a.get_vector_product(r).get_length() / self.a.get_length())
        } else if self.are_skew(line) {
            Ok(self.a.get_triple_product(line.a, r).to_f64().get_amount() / self.a.get_vector_product(line.a).get_length())
        } else {
            return Err(MatholError::VectorCause(VectorError {
                message: "Lines do cross".to_string(),
            }));
        }
    }
}
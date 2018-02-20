use std::f64::consts::PI;
use basics::{pow, pythagoras2d};
use basics::convert_trait::Convert;
use basics::amount_trait::Amount;
use basics::cotangent::Cotangent;
use std::ops::Add;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use vectoroperations::vector2d::Vector2D;
use num::{Num, FromPrimitive};
use geometrics::traits::{Area, Perimeter, Height, Diagonal};
use error::*;


/// Struct representing a triangle in euclidean space
/// # Usage
/// ```
/// pub use mathol::geometrics::planimetry::Triangle;
/// ```
#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    /// Edge a
    pub a: f64,
    /// Edge b
    pub b: f64,
    /// Edge c
    pub c: f64,
}

impl Triangle
{
    /// Creates a triangle instance with the given edges
    /// # Parameters
    /// a: Edge a
    ///
    /// b: Edge b
    ///
    /// c: Edge c
    /// # Return values
    /// Returns the triangle instance in case of success
    ///
    /// Returns NegativeValueError if at least one of the given edges has a negative value
    ///
    /// Returns LengthError if no triangle can be constructed with the given edges.
    /// # Examples
    /// ```
    /// let triangle = Triangle::build_triangle_with_edges(3, 4, 5).expect("error");
    /// ```
    pub fn build_triangle_with_edges<T>(a: T, b: T, c: T) -> Result<Triangle, MatholError>
        where T: Num + Convert + Add<Output=T> + PartialOrd + Copy
    {
        if a <= T::zero() || b <= T::zero() || c <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Length of triangle edges must be positive".to_string(),
            }))
        }
        if a + b <= c || a + c <= b || b + c <= a {
            return Err(MatholError::LengthCause(LengthError {
                message: "Cannot create a triangle with the given edges".to_string(),
            }));
        }

        let triangle = Triangle {
            a: a.to_f64(),
            b: b.to_f64(),
            c: c.to_f64(),
        };

        Ok(triangle)
    }

    /// Creates a triangle instance with the given vertices
    /// # Parameters
    /// a: Vertice a
    ///
    /// b: Vertice b
    ///
    /// c: Vertice c
    /// # Return values
    /// Returns the triangle instance
    /// # Examples
    /// ```
    /// use mathol::vectoroperations::vector2d::Vector2D;
    ///
    /// let triangle = Triangle::build_triangle_with_points(Vector2D{x: 1, y:2}, Vector2D{x: 6, y:2}, Vector2D{x: 3, y:4}).expect("error");
    /// ```
    #[allow(non_snake_case)]
    pub fn build_triangle_with_points<T>(A: Vector2D<T>, B: Vector2D<T>, C: Vector2D<T>) -> Triangle
        where T: Num + Convert + Add<Output=T> + PartialOrd + Copy + Amount<T> + Debug + FromPrimitive
    {
        let a = B.get_distance(C);
        let b = A.get_distance(C);
        let c = A.get_distance(B);

        Triangle::build_triangle_with_edges(a, b, c).unwrap()
    }

    /// Calculates the value of each of the three angles in a triangle
    /// # Return values
    /// Returns a tuple with the values of angles alpha, beta and gamma as degrees.
    /// # Examples
    /// ```
    /// let triangle = Triangle::build_triangle_with_edges(3, 4, 5).unwrap();
    /// let (alpha, beta, gamma) = triangle.get_angles();
    /// assert_eq!(36.86989764584401, alpha);
    /// assert_eq!(53.13010235415599, beta);
    /// assert_eq!(90.0, gamma);
    /// ```
    pub fn get_angles(self) -> (f64, f64, f64) {
        let alpha = ((pow(self.a, 2) - pow(self.b, 2) - pow(self.c, 2)) / (-2.0 * self.b * self.c)).acos();
        let beta = ((pow(self.b, 2) - pow(self.a, 2) - pow(self.c, 2)) / (-2.0 * self.a * self.c)).acos();
        let gamma = ((pow(self.c, 2) - pow(self.a, 2) - pow(self.b, 2)) / (-2.0 * self.a * self.b)).acos();

        (alpha.to_degrees(), beta.to_degrees(), gamma.to_degrees())
    }

    /// Calculates the inner circle of a triangle
    /// # Return values
    /// Returns a circle instance in case of success
    /// # Examples
    /// ```
    /// use mathol::geometrics::planimetry::Circle;
    ///
    /// let triangle = Triangle::build_triangle_with_edges(3, 4, 5).expect("error");
    /// let circle = triangle.get_inner_circle();
    /// assert_eq!(1.0, circle.r);
    /// ```
    pub fn get_inner_circle(self) -> Circle {
        let s = self.get_perimeter() / 2.0;
        let r = ((s - self.a) * (s - self.b) * (s - self.c) / s).sqrt();
        Circle::build_circle(r).unwrap()
    }

    /// Calculates the outer circle of a triangle
    /// # Return values
    /// Returns a circle instance in case of success
    /// # Examples
    /// ```
    /// use mathol::geometrics::planimetry::Circle;
    ///
    /// let triangle = Triangle::build_triangle_with_edges(3, 4, 5).expect("error");
    /// let circle = triangle.get_outer_circle();
    /// assert_eq!(2.5, circle.r);
    /// ```
    pub fn get_outer_circle(self) -> Circle {
        let s = self.get_perimeter() / 2.0;
        let r = (self.a * self.b * self.c) / (4.0 * (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt());
        Circle::build_circle(r).unwrap()
    }
}

impl Area for Triangle {
    /// Calculates the area of a triangle
    /// # Remarks
    /// For every triangle, the area can be calculated with A = s * (s - a) * (s - b) * (s - c)
    ///
    /// With s = P / 2
    /// # Return values
    /// Returns the calculated area as f64 value
    /// # Examples
    /// ```
    /// let triangle = Triangle::build_triangle_with_edges(3, 4, 5).expect("error");
    /// assert_eq!(6.0 , triangle.get_area());
    /// ```
    fn get_area(self) -> f64 {
        let s = self.get_perimeter() / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
}

impl Perimeter for Triangle {
    /// Calculates the perimeter of a triangle
    /// # Remarks
    /// The formula for a triangle's perimeter is P = a + b + c
    /// # Return values
    /// Returns the perimeter as f64 values
    /// # Examples
    /// ```
    /// let triangle = Triangle::build_triangle_with_edges(3, 4, 5).unwrap();
    /// assert_eq!(12.0 , triangle.get_perimeter());
    /// ```
    fn get_perimeter(self) -> f64 {
        self.a + self.b + self.c
    }
}

impl Height for Triangle {
    /// Calculates the height of a triangle
    /// # Remarks
    /// The formula for a triangle's height is h = a * sin(beta)
    /// # Return values
    /// Returns the height as f64 values
    /// # Examples
    /// ```
    /// let triangle = Triangle::build_triangle_with_edges(3, 4, 5).unwrap();
    /// assert_eq!(2.4, triangle.get_height());
    /// ```
    fn get_height(self) -> f64 {
        self.a * self.get_angles().1.to_radians().sin()
    }
}


/// Struct representing a rectangle (and square) in euclidean space
/// # Usage
/// ```
/// pub use mathol::geometrics::planimetry::Rectangle;
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    /// Edge a
    pub a: f64,
    /// Edge b
    pub b: f64,
}

impl Rectangle {
    /// Creates a rectangle instance with the given edges
    /// # Parameters
    /// a: Edge a
    ///
    /// b: Edge b
    /// # Return values
    /// Returns the rectangle instance in case of success
    ///
    /// Returns NegativeValueError if at least one of the given edges has a negative value
    /// # Examples
    /// ```
    /// let rectangle = Rectangle::build_rectangle(4, 9).expect("error");
    /// ```
    pub fn build_rectangle<T>(a: T, b: T) -> Result<Rectangle, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if a <= T::zero() || b <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Rectangle must have a positive length or width".to_string(),
            }));
        }

        Ok(Rectangle {
            a: a.to_f64(),
            b: b.to_f64(),
        })
    }
}

impl Area for Rectangle {
    /// Calculates the area of a rectangle
    /// # Remarks
    /// Formula for a rectangle is A = a * b
    ///
    /// If a = b, the rectangle is a square
    /// # Return values
    /// Returns the calculated area as f64 value
    /// # Examples
    /// ```
    /// let rectangle = Rectangle::build_rectangle(4, 9).expect("error");
    /// assert_eq!(36.0, rectangle.get_area());
    /// ```
    fn get_area(self) -> f64 {
        self.a * self.b
    }
}

impl Perimeter for Rectangle {
    /// Calculates the perimeter of a rectangle
    /// # Remarks
    /// The formula for a rectangle's perimeter is P = 2*a + 2*b
    /// # Return values
    /// Returns the perimeter as f64 value
    /// # Examples
    /// ```
    /// let rectangle = Rectangle::build_rectangle(4, 9).expect("error");
    /// assert_eq!(26.0, rectangle.get_perimeter());
    /// ```
    fn get_perimeter(self) -> f64 {
        2.0 * self.a + 2.0 * self.b
    }
}

impl Diagonal for Rectangle {
    /// Calculates the length of the diagonal of a rectangle
    /// # Remarks
    /// The formula for a rectangle's diagonal is d = sqrt(a² + b²)
    /// # Return values
    /// Returns the diagonal length as f64
    /// # Examples
    /// ```
    /// let rectangle = Rectangle::build_rectangle(4, 9).expect("error");
    /// assert_eq!(9.848857801796104, rectangle.get_diagonal());
    /// ```
    fn get_diagonal(self) -> f64 {
        pythagoras2d(self.a, self.b)
    }
}


/// Struct representing a parallelogram in euclidean space
/// # Usage
/// ```
/// pub use mathol::geometrics::planimetry::Parallelogram;
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Parallelogram {
    /// Edge a
    pub a: f64,
    /// Edge b
    pub b: f64,
    /// Height h
    pub h: f64,
}

impl Parallelogram {
    /// Creates a parallelogram instance with the given edges and height
    /// # Parameters
    /// a: Edge a
    ///
    /// b: Edge b
    ///
    /// h: Height h
    /// # Return values
    /// Returns the parallelogram instance in case of success
    ///
    /// Returns NegativeValueError if at least one of the given edges or the height has a negative value
    /// # Examples
    /// ```
    /// let parallelogram = Parallelogram::build_parallelogram(9,5, 4).expect("error");
    /// ```
    pub fn build_parallelogram<T>(a: T, b: T, h: T) -> Result<Parallelogram, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if a <= T::zero() || b <= T::zero() || h <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Parallelogram must have a positive length, width or height".to_string(),
            }));
        }

        Ok(Parallelogram {
            a: a.to_f64(),
            b: b.to_f64(),
            h: h.to_f64(),
        })
    }
}

impl Area for Parallelogram {
    /// Calculates the area of a parallelogram
    /// # Remarks
    /// Formula for a parallelogram is A = a * h
    /// # Return values
    /// Returns the calculated area as f64 value
    /// # Examples
    /// ```
    /// let parallelogram = Parallelogram::build_parallelogram(9,5, 4).expect("error");
    /// assert_eq!(36.0, parallelogram.get_area());
    /// ```
    fn get_area(self) -> f64 {
        self.a * self.h
    }
}

impl Perimeter for Parallelogram {
    /// Calculates the perimeter of a parallelogram
    /// # Remarks
    /// The formula for a parallelogram's perimeter is P = 2*a + 2*b
    /// # Return values
    /// Returns the perimeter as f64 value
    /// # Examples
    /// ```
    /// let parallelogram = Parallelogram::build_parallelogram(9,5, 4).expect("error");
    /// assert_eq!(28.0, parallelogram.get_perimeter());
    /// ```
    fn get_perimeter(self) -> f64 {
        2.0 * self.a + 2.0 * self.b
    }
}


/// Struct representing a trapeze in euclidean space
/// # Usage
/// ```
/// pub use mathol::geometrics::planimetry::Trapeze;
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Trapeze {
    /// Edge a
    pub a: f64,
    /// Edge b
    pub b: f64,
    /// Edge c
    pub c: f64,
    /// Edge d
    pub d: f64,
}

impl Trapeze {
    /// Creates a trapeze instance with the given edges
    /// # Parameters
    /// a: Length of Edge a
    ///
    /// b: Length of Edge b
    ///
    /// c: Length of Edge c
    ///
    /// d: Edge d
    /// # Return values
    /// Returns the trapeze instance in case of success
    ///
    /// Returns NegativeValueError if at least one of the given edges has a negative value
    /// # Examples
    /// ```
    /// let trapeze = Trapeze::build_trapeze(9.0, 6.0, 4.2, 4.5).expect("error");
    /// ```
    pub fn build_trapeze<T>(a: T, b: T, c: T, d: T) -> Result<Trapeze, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if a <= T::zero() || b <= T::zero() || c <= T::zero() || d <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Trapeze edges must have a positive length".to_string(),
            }));
        }

        Ok(Trapeze {
            a: a.to_f64(),
            b: b.to_f64(),
            c: c.to_f64(),
            d: d.to_f64(),
        })
    }
}

impl Area for Trapeze {
    /// Calculates the area of a trapeze
    /// # Remarks
    /// Formula for a trapeze is 0.5 * (a + b) * h
    /// # Return values
    /// Returns the calculated area as f64 value
    /// # Examples
    /// ```
    /// let trapeze = Trapeze::build_trapeze(9.0, 6.0, 4.2, 4.5).expect("error");
    /// assert_eq!(30.470474951171994, trapeze.get_area());
    /// ```
    fn get_area(self) -> f64 {
        0.5 * (self.a + self.b) * self.get_height()
    }
}

impl Perimeter for Trapeze {
    /// Calculates the perimeter of a trapeze
    /// # Remarks
    /// The formula for a trapeze's perimeter is P = a + b + c + d
    /// # Return values
    /// Returns the perimeter as f64 value
    /// # Examples
    /// ```
    /// let trapeze = Trapeze::build_trapeze(9.0, 6.0, 4.2, 4.5).expect("error");
    /// assert_eq!(23.7, trapeze.get_perimeter());
    /// ```
    fn get_perimeter(self) ->f64 {
        self.a + self.b + self.c + self.d
    }
}

impl Height for Trapeze {
    /// Calculates the height of a trapeze
    /// # Return values
    /// Returns the height as f64 values
    /// # Examples
    /// ```
    /// let trapeze = Trapeze::build_trapeze(9.0, 6.0, 4.2, 4.5).expect("error");
    /// assert_eq!(4.062729993489599, trapeze.get_height());
    /// ```
    fn get_height(self) -> f64 {
        ((self.a + self.d - self.b + self.c) * (-self.a + self.d + self.b + self.c) *
            (-self.a - self.d + self.b + self.c) * (-self.a + self.d + self.b - self.c)).sqrt() /
            (2.0 * (self.a - self.b))
    }
}


/// Struct representing a polygon in euclidean space
/// # Usage
/// ```
/// pub use mathol::geometrics::planimetry::Polygon;
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Polygon {
    /// Length of an edge
    pub a: f64,
    /// Number of vertices aka edges
    pub n: f64,
}

impl Polygon {
    /// Creates a polygon instance with the given edges
    /// # Parameters
    /// a: Length of Edge a
    ///
    /// n: Number of edges
    /// # Return values
    /// Returns the polygon instance in case of success
    ///
    /// Returns NegativeValueError if the length of the edge or the number of edges is a negative value
    /// # Examples
    /// ```
    /// let trapeze = Trapeze::build_trapeze(9.0, 6.0, 4.2, 4.5).expect("error");
    /// ```
    pub fn build_polygon<T>(a: T, n: T) -> Result<Polygon, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if a <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Polygon edges must have a positive length".to_string(),
            }));
        }
        if n <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Polygon must have a positive number of edges".to_string(),
            }));
        }

        Ok(Polygon {
            a: a.to_f64(),
            n: n.to_f64(),
        })
    }

    /// Calculates the distance of the polygon's centre to one of its vertices
    /// # Return values
    /// Returns the distance as a f64 value
    /// # Examples
    /// ```
    /// let polygon = Polygon::build_polygon(2, 8).expect("error");
    /// assert_eq!(2.613125929752753, polygon.get_radius());
    /// ```
    pub fn get_radius(&self) -> f64 {
        let basis_area = self.get_area();
        let m = 0.5 * (16.0 * pow(basis_area / self.n.to_f64(), 2) / pow(self.a, 2) + pow(self.a, 2)).sqrt();
        m
    }
}

impl Area for Polygon {
    /// Calculates the area of a polygon
    /// # Remarks
    /// Formula for a polygon is A = 0.25 * n * a² * cot(PI / n)
    /// # Return values
    /// Returns the calculated area as f64 value
    /// # Examples
    /// ```
    /// let polygon = Polygon::build_polygon(2, 8).expect("error");
    /// assert_eq!(19.31370849898476, polygon.get_area());
    /// ```
    fn get_area(self) -> f64 {
        0.25 * self.n * pow(self.a, 2) * (PI / self.n).cot()
    }
}

impl Perimeter for Polygon {
    /// Calculates the perimeter of a polygon
    /// # Remarks
    /// The formula for a polygon's perimeter is P = a * n
    /// # Return values
    /// Returns the perimeter as f64 value
    /// # Examples
    /// ```
    /// let polygon = Polygon::build_polygon(2, 8).expect("error");
    /// assert_eq!(16.0, polygon.get_perimeter());
    /// ```
    fn get_perimeter(self) -> f64 {
        self.a * self.n
    }
}


/// Struct representing a circle in euclidean space
/// # Usage
/// ```
/// pub use mathol::geometrics::planimetry::Circle;
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Circle {
    /// Radius r
    pub r: f64,
}

impl Circle {
    /// Creates a circle instance with the given radius
    /// # Parameters
    /// r: Radius r
    /// # Return values
    /// Returns the circle instance in case of success
    ///
    /// Returns NegativeValueError if the radius is a negative value
    /// # Examples
    /// ```
    /// let circle = Circle::build_circle(2).expect("error");
    /// ```
    pub fn build_circle<T>(r: T) -> Result<Circle, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if r <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Circle radius must be positive".to_string(),
            }))
        }

        Ok(Circle {
            r: r.to_f64(),
        })
    }
}

impl Area for Circle {
    /// Calculates the area of a circle
    /// # Remarks
    /// Formula for a circle is A = PI * r²
    /// # Return values
    /// Returns the calculated area as f64 value
    /// # Examples
    /// ```
    /// let circle = Circle::build_circle(2).expect("error");
    /// assert_eq!(12.566370614359172, circle.get_area());
    /// ```
    fn get_area(self) -> f64 {
        PI * self.r * self.r
    }
}

impl Perimeter for Circle {
    /// Calculates the perimeter of a circle
    /// # Remarks
    /// The formula for a circle's perimeter is P = 2 * PI * r
    /// # Return values
    /// Returns the perimeter as f64 value
    /// # Examples
    /// ```
    /// let circle = Circle::build_circle(2).expect("error");
    /// assert_eq!(12.566370614359172, circle.get_perimeter());
    /// ```
    fn get_perimeter(self) -> f64 {
        2.0 * PI * self.r
    }
}


/// Struct representing an ellipsis in euclidean space
/// # Usage
/// ```
/// pub use mathol::geometrics::planimetry::Ellipsis;
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Ellipsis {
    /// Length a
    pub a: f64,
    /// Width b
    pub b: f64,
}

impl Ellipsis {
    /// Creates an ellipsis instance with the given length and width
    /// # Parameters
    /// a: Length a
    ///
    /// b: Width b
    /// # Return values
    /// Returns the ellipsis instance in case of success
    ///
    /// Returns NegativeValueError if the length or width is a negative value
    /// # Examples
    /// ```
    /// let ellipsis = Ellipsis::build_ellipsis(2, 3).expect("error");
    /// ```
    pub fn build_ellipsis<T>(a: T, b: T) -> Result<Ellipsis, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if a <= T::zero() || b <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Ellipsis must have a positive length or width".to_string(),
            }))
        }

        Ok(Ellipsis {
            a: a.to_f64(),
            b: b.to_f64(),
        })
    }
}

impl Area for Ellipsis {
    /// Calculates the area of an ellipsis
    /// # Remarks
    /// Formula for an ellipsis is A = PI * a * b
    /// # Return values
    /// Returns the calculated area as f64 value
    /// # Examples
    /// ```
    /// let ellipsis = Ellipsis::build_ellipsis(2, 3).expect("error");
    /// assert_eq!(18.84955592153876, ellipsis.get_area());
    /// ```
    fn get_area(self) -> f64 {
        PI * self.a * self.b
    }
}

impl Perimeter for Ellipsis {
    /// Calculates the estimated perimeter of an ellipsis
    /// # Remarks
    /// The formula for an ellipsis's perimeter is P = PI * (1.5 * (a + b) - sqrt(a*b))
    /// # Return values
    /// Returns the perimeter as f64 value
    /// # Examples
    /// ```
    /// let ellipsis = Ellipsis::build_ellipsis(2, 3).expect("error");
    /// assert_eq!(15.866645920952264, ellipsis.get_perimeter());
    /// ```
    fn get_perimeter(self) -> f64 {
        PI * (1.5 * (self.a + self.b) - (self.a * self.b).sqrt())
    }
}
use std::f64::consts::PI;
use basics::{pow, pythagoras3d};
use basics::convert_trait::Convert;
use num::Num;
use geometrics::traits::*;
use std::cmp::PartialOrd;
use error::*;


/// Struct representing a cuboid in euclidean space
/// # Usage
/// ```
/// use mathol::geometrics::stereometry::Cuboid;
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Cuboid {
    /// Length a
    pub a: f64,
    /// Width b
    pub b: f64,
    /// Height c
    pub c: f64,
}

impl Cuboid {
    /// Creates a cuboid instance with the given length, width and height
    /// # Parameters
    /// a: Length a
    ///
    /// b: Width b
    ///
    /// c: Height c
    /// # Return values
    /// Returns the cuboid instance in case of success
    ///
    /// Returns NegativeValueError if at least one of the given edges has a negative value
    /// # Examples
    /// ```
    /// let cuboid = Cuboid::build_cuboid(4, 4, 4).expect("error");
    /// ```
    pub fn build_cuboid<T>(a: T, b: T, c: T) -> Result<Cuboid, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if a <= T::zero() || b <= T::zero() || c <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Cuboid must have a positive length, width or height".to_string(),
            }));
        }

        Ok(Cuboid {
            a: a.to_f64(),
            b: b.to_f64(),
            c: c.to_f64(),
        })
    }
}

impl Volume for Cuboid {
    /// Calculates the volume of a cuboid
    /// # Remarks
    /// Formula for a cuboid is A = a * b * c
    ///
    /// If a = b = c, the cuboid is a cube
    /// # Return values
    /// Returns the calculated volume as f64 value
    /// # Examples
    /// ```
    /// let cuboid = Cuboid::build_cuboid(1, 4, 9).expect("error");
    /// assert_eq!(36.0, cuboid.get_volume());
    /// ```
    fn get_volume(self) -> f64 {
        self.a * self.b * self.c
    }
}

impl Surface for Cuboid {
    /// Calculates the surface of a cuboid
    /// # Remarks
    /// Formula for a cuboid is S = 2 * (a*b + a*c + b*c)
    ///
    /// If a = b = c, the cuboid is a cube
    /// # Return values
    /// Returns the calculated surface as f64 value
    /// # Examples
    /// ```
    /// let cuboid = Cuboid::build_cuboid(1, 4, 9).expect("error");
    /// assert_eq!(98.0, cuboid.get_surface());
    /// ```
    fn get_surface(self) -> f64 {
        2.0 * (self.a * self.b + self.a * self.c + self.b * self.c)
    }
}

impl Diagonal for Cuboid {
    /// Calculates the length of the diagonal of a cuboid
    /// # Remarks
    /// The formula for a cuboid's diagonal is d = sqrt(a² + b² + c²)
    /// # Return values
    /// Returns the diagonal length as f64
    /// # Examples
    /// ```
    /// let cuboid = Cuboid::build_cuboid(1, 4, 9).expect("error");
    /// assert_eq!(9.899494936611665, cuboid.get_diagonal());
    /// ```
    fn get_diagonal(self) -> f64 {
        pythagoras3d(self.a, self.b, self.c)
    }
}


/// Struct representing a pyramid in euclidean space
/// # Usage
/// ```
/// use mathol::geometrics::stereometry::Pyramid;
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Pyramid {
    /// Basis area
    pub area: f64,
    /// Height
    pub h: f64,
}

impl Pyramid {
    /// Creates a pyramid instance with the given area and height
    /// # Parameters
    /// area: Basis area
    ///
    /// h: height
    /// # Return values
    /// Returns the pyramid instance in case of success
    ///
    /// Returns NegativeValueError if the area or the height has a negative value
    /// # Examples
    /// ```
    /// let pyramid = Pyramid::build_pyramid(5, 7).expect("error");
    /// ```
    pub fn build_pyramid<T>(area: T, h: T) -> Result<Pyramid, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if area <= T::zero() || h <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Pyramid must have a positive area or height".to_string()
            }));
        }

        Ok(Pyramid {
            area: area.to_f64(),
            h: h.to_f64(),
        })
    }
}

impl Volume for Pyramid {
    /// Calculates the volume of a pyramid
    /// # Remarks
    /// Formula for a pyramid is A = (1/3) * area * h
    /// # Return values
    /// Returns the calculated volume as f64 value
    /// # Examples
    /// ```
    /// let pyramid = Pyramid::build_pyramid(5, 7).expect("error");
    /// assert_eq!(11.666666666666666, pyramid.get_volume());
    /// ```
    fn get_volume(self) -> f64 {
        (1.0 / 3.0) * self.area * self.h
    }
}


/// Struct representing a wedge in euclidean space
/// # Usage
/// ```
/// use mathol::geometrics::stereometry::Wedge;
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Wedge {
    /// Length of basis rectangle
    pub a: f64,
    /// Width of basis rectangle
    pub b: f64,
    /// Length of edge c
    pub c: f64,
    /// Height between basis rectangle and edge c
    pub h: f64,
}

impl Wedge {
    /// Creates a wedge instance with the given edges and the height
    /// # Parameters
    /// a: Length a
    ///
    /// b: Width b
    ///
    /// c: Edge c
    ///
    /// h: Height h
    /// # Return values
    /// Returns the wedge instance in case of success
    ///
    /// Returns NegativeValueError if at least one of the given edges has a negative value
    /// # Examples
    /// ```
    /// let wedge = Wedge::build_wedge(5, 2, 3, 7).expect("error");
    /// ```
    pub fn build_wedge<T>(a: T, b: T, c: T, h: T) -> Result<Wedge, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if a <= T::zero() || b <= T::zero() || c <= T::zero() || h <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Wedge must have a positive length, width or height".to_string(),
            }))
        }

        Ok(Wedge {
            a: a.to_f64(),
            b: b.to_f64(),
            c: c.to_f64(),
            h: h.to_f64(),
        })
    }
}


impl Volume for Wedge {
    /// Calculates the volume of a wedge
    /// # Remarks
    /// Formula for a wedge is A = (1/6) * b * h * (2*a + c)
    /// # Return values
    /// Returns the calculated volume as f64 value
    /// # Examples
    /// ```
    /// let wedge = Wedge::build_wedge(5, 2, 3, 7).expect("error");
    /// assert_eq!(30.33333333333333, wedge.get_volume());
    /// ```
    fn get_volume(self) -> f64 {
        (1.0 / 6.0) * self.b * self.h * (2.0 * self.a + self.c)
    }
}


/// Struct representing a cylinder in euclidean space
/// # Usage
/// ```
/// use mathol::geometrics::stereometry::Cylinder;
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Cylinder {
    /// Radius of basic circle
    pub r: f64,
    /// Height
    pub h: f64,
}

impl Cylinder {
    /// Creates a cylinder instance with the given radius and height
    /// # Parameters
    /// r: Radius r
    ///
    /// c: Height c
    /// # Return values
    /// Returns the cylinder instance in case of success
    ///
    /// Returns NegativeValueError if the radius or height has a negative value
    /// # Examples
    /// ```
    /// let cylinder = Cylinder::build_cylinder(2, 8).expect("error");
    /// ```
    pub fn build_cylinder<T>(r: T, h: T) -> Result<Cylinder, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if r <= T::zero() || h <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Cylinder must have a positive radius or height".to_string(),
            }));
        }

        Ok(Cylinder {
            r: r.to_f64(),
            h: h.to_f64(),
        })
    }
}

impl Volume for Cylinder {
    /// Calculates the volume of a cylinder
    /// # Remarks
    /// Formula for a cylinder is A = PI * r² * h
    /// # Return values
    /// Returns the calculated volume as f64 value
    /// # Examples
    /// ```
    /// let cylinder = Cylinder::build_cylinder(2, 8).expect("error");
    /// assert_eq!(100.53096491487338, cylinder.get_volume());
    /// ```
    fn get_volume(self) -> f64 {
        PI * pow(self.r, 2) * self.h
    }
}

impl Surface for Cylinder {
    /// Calculates the surface of a cylinder
    /// # Remarks
    /// Formula for a cylinder is S = 2 * PI * r * (r + h)
    /// # Return values
    /// Returns the calculated surface as f64 value
    /// # Examples
    /// ```
    /// let cylinder = Cylinder::build_cylinder(2, 8).expect("error");
    /// assert_eq!(125.66370614359172, cylinder.get_surface());
    /// ```
    fn get_surface(self) -> f64 {
        2.0 * PI * self.r * (self.r + self.h)
    }
}

impl Lateral for Cylinder {
    /// Calculates the lateral surface of a cylinder
    /// # Remarks
    /// Formula for a cylinder is L = 2 * PI * r * h
    /// # Return values
    /// Returns the calculated lateral surface as f64 value
    /// # Examples
    /// ```
    /// let cylinder = Cylinder::build_cylinder(2, 8).expect("error");
    /// assert_eq!(100.53096491487338, cylinder.get_lateral());
    /// ```
    fn get_lateral(self) -> f64 {
        2.0 * PI * self.r * self.h
    }
}


/// Struct representing a cone in euclidean space
/// # Usage
/// ```
/// use mathol::geometrics::stereometry::Cone;
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Cone {
    /// Radius of basic circle
    pub r: f64,
    /// Height
    pub h: f64,
}

impl Cone {
    /// Creates a cone instance with the given radius and height
    /// # Parameters
    /// r: Radius r
    ///
    /// c: Height c
    /// # Return values
    /// Returns the cone instance in case of success
    ///
    /// Returns NegativeValueError if the radius or height has a negative value
    /// # Examples
    /// ```
    /// let cone = Cone::build_cone(3, 7).expect("error");
    /// ```
    pub fn build_cone<T>(r: T, h: T) -> Result<Cone, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if r <= T::zero() || h <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Cone must have a positive radius or height".to_string(),
            }));
        }

        Ok(Cone {
            r: r.to_f64(),
            h: h.to_f64(),
        })
    }
}

impl Volume for Cone {
    /// Calculates the volume of a cone
    /// # Remarks
    /// Formula for a cone is A = (1/3) * PI * r² * h
    /// # Return values
    /// Returns the calculated volume as f64 value
    /// # Examples
    /// ```
    /// let cone = Cone::build_cone(3, 7).expect("error");
    /// assert_eq!(65.97344572538566, cone.get_volume());
    /// ```
    fn get_volume(self) -> f64 {
        (1.0 / 3.0) * PI * pow(self.r, 2) * self.h
    }
}

impl Surface for Cone {
    /// Calculates the surface of a cone
    /// # Remarks
    /// Formula for a cone is S = PI * r * (r + s)
    /// # Return values
    /// Returns the calculated surface as f64 value
    /// # Examples
    /// ```
    /// let cone = Cone::build_cone(3, 7).expect("error");
    /// assert_eq!(100.05130440467447, cone.get_surface());
    /// ```
    fn get_surface(self) -> f64 {
        let s = (pow(self.r, 2) + (pow(self.h, 2))).sqrt();
        PI * self.r * (self.r + s)
    }
}

impl Lateral for Cone {
    /// Calculates the lateral surface of a cone
    /// # Remarks
    /// Formula for a cone is L = PI * r * s
    /// # Return values
    /// Returns the calculated lateral surface as f64 value
    /// # Examples
    /// ```
    /// let cone = Cone::build_cone(3, 7).expect("error");
    /// assert_eq!(71.77697052236633, cone.get_lateral());
    /// ```
    fn get_lateral(self) -> f64 {
        let s = (pow(self.r, 2) + (pow(self.h, 2))).sqrt();
        PI * self.r * s
    }
}


/// Struct representing a sphere in euclidean space
/// # Usage
/// ```
/// pub use mathol::geometrics::stereometry::Sphere;
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    /// Radius
    pub r: f64,
}

impl Sphere {
    /// Creates a sphere instance with the given radius
    /// # Parameters
    /// r: Radius
    /// # Return values
    /// Returns the sphere instance in case of success
    ///
    /// Returns NegativeValueError if at least one of the given edges has a negative value
    /// # Examples
    /// ```
    /// let sphere = Sphere::build_sphere(4).expect("error");
    /// ```
    pub fn build_sphere<T>(r: T) -> Result<Sphere, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if r <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Sphere must have a positive radius".to_string(),
            }));
        }

        Ok(Sphere {
            r: r.to_f64(),
        })
    }
}

impl Volume for Sphere {
    /// Calculates the volume of a sphere
    /// # Remarks
    /// Formula for a sphere is A = (4/3) * PI * r³
    /// # Return values
    /// Returns the calculated volume as f64 value
    /// # Examples
    /// ```
    /// let sphere = Sphere::build_sphere(4).expect("error");
    /// assert_eq!(268.082573106329, sphere.get_volume());
    /// ```
    fn get_volume(self) -> f64 {
        (4.0 / 3.0) * PI * pow(self.r, 3)
    }
}

impl Surface for Sphere {
    /// Calculates the surface of a sphere
    /// # Remarks
    /// Formula for a sphere is S = 4 * PI * r²
    /// # Return values
    /// Returns the calculated surface as f64 value
    /// # Examples
    /// ```
    /// let sphere = Sphere::build_sphere(4).expect("error");
    /// assert_eq!(201.06192982974676, sphere.get_surface());
    /// ```
    fn get_surface(self) -> f64 {
        4.0 * PI * pow(self.r, 2)
    }
}


/// Struct representing an ellipsoid in euclidean space
/// # Usage
/// ```
/// pub use mathol::geometrics::stereometry::Ellipsoid;
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Ellipsoid {
    /// Length
    pub a: f64,
    /// Width
    pub b: f64,
    /// Height
    pub c: f64,
}

impl Ellipsoid {
    /// Creates an ellipsoid instance with the given length, width and height
    /// # Parameters
    /// a: Length
    ///
    /// b: Width
    ///
    /// c: Height
    /// # Return values
    /// Returns the ellipsoid instance in case of success
    ///
    /// Returns NegativeValueError if at least one of the given radiuses has a negative value
    /// # Examples
    /// ```
    /// let ellipsoid = Ellipsoid::build_ellipsoid(2, 3, 4).expect("error");
    /// ```
    pub fn build_ellipsoid<T>(a: T, b: T, c: T) -> Result<Ellipsoid, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if a <= T::zero() || b <= T::zero() || c <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Ellipsoid must have a positive length, width or height".to_string(),
            }));
        }

        Ok(Ellipsoid {
            a: a.to_f64(),
            b: b.to_f64(),
            c: c.to_f64(),
        })
    }
}

impl Volume for Ellipsoid {
    /// Calculates the volume of an ellipsoid
    /// # Remarks
    /// Formula for an ellipsoid is A = (4/3) * PI * a * b * c
    /// # Return values
    /// Returns the calculated volume as f64 value
    /// # Examples
    /// ```
    /// let ellipsoid = Ellipsoid::build_ellipsoid(2, 3, 4).expect("error");
    /// assert_eq!(100.53096491487338, ellipsoid.get_volume());
    /// ```
    fn get_volume(self) -> f64 {
        (4.0 / 3.0) * PI * self.a * self.b * self.c
    }
}


/// Struct representing a barrel with spheric curvature
/// # Usage
/// ```
/// pub use mathol::geometrics::stereometry::SphericBarrel;
/// ```
#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone)]
pub struct SphericBarrel {
    /// Middle radius
    pub R: f64,
    /// Top and bottom radius
    pub r: f64,
    /// Height
    pub h: f64,
}

#[allow(non_snake_case)]
impl SphericBarrel {
    /// Creates a SphericBarrel instance with the given radiuses and height
    /// # Parameters
    /// R: Middle radius
    ///
    /// r: Top and bottom radius
    ///
    /// h: height
    /// # Return values
    /// Returns the SphericBarrel instance in case of success
    ///
    /// Returns NegativeValueError if at least one of the given radiuses has a negative value
    /// # Examples
    /// ```
    /// let barrel = SphericBarrel::build_barrel(4, 2, 6).expect("error");
    /// ```
    pub fn build_barrel<T>(R: T, r: T, h: T) -> Result<SphericBarrel, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if R <= T::zero() || r <= T::zero() || h <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Barrel must have a positive radius or height".to_string(),
            }));
        }

       Ok(SphericBarrel {
           R: R.to_f64(),
           r: r.to_f64(),
           h: h.to_f64(),
       })
    }
}

impl Volume for SphericBarrel {
    /// Calculates the volume of a SphericBarrel
    /// # Remarks
    /// Formula for a SphericBarrel is A = (1/3) * PI * h * (2 * (0.5*R)² + (0.5*r)²)
    /// # Return values
    /// Returns the calculated volume as f64 value
    /// # Examples
    /// ```
    /// let barrel = SphericBarrel::build_barrel(4, 2, 6).expect("error");
    /// assert_eq!(56.548667764616276, barrel.get_volume());
    /// ```
    fn get_volume(self) -> f64 {
        (1.0 / 3.0) * PI * self.h * (2.0 * pow(self.R / 2.0, 2) + pow(self.r / 2.0, 2))
    }
}


/// Struct representing a barrel with parabolic curvature
/// # Usage
/// ```
/// pub use mathol::geometrics::stereometry::ParabolicBarrel;
/// ```
#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone)]
pub struct ParabolicBarrel {
    /// Middle radius
    pub R: f64,
    /// Top and bottom radius
    pub r: f64,
    /// Height
    pub h: f64,
}

#[allow(non_snake_case)]
impl ParabolicBarrel {
    /// Creates a ParabolicBarrel instance with the given radiuses and height
    /// # Parameters
    /// R: Middle radius
    ///
    /// r: Top and bottom radius
    ///
    /// h: height
    /// # Return values
    /// Returns the ParabolicBarrel instance in case of success
    ///
    /// Returns NegativeValueError if at least one of the given radiuses has a negative value
    /// # Examples
    /// ```
    /// let barrel = ParabolicBarrel::build_barrel(4, 2, 6).expect("error");
    /// ```
    pub fn build_barrel<T>(R: T, r: T, h: T) -> Result<ParabolicBarrel, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if R <= T::zero() || r <= T::zero() || h <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Barrel must have a positive radius or height".to_string(),
            }));
        }

        Ok(ParabolicBarrel {
            R: R.to_f64(),
            r: r.to_f64(),
            h: h.to_f64(),
        })
    }
}

impl Volume for ParabolicBarrel {
    /// Calculates the volume of a ParabolicBarrel
    /// # Remarks
    /// Formula for a ParabolicBarrel is A = (1/15) * PI * h * (8*(0.5R)² + R * r + (3/4)*r²)
    /// # Return values
    /// Returns the calculated volume as f64 value
    /// # Examples
    /// ```
    /// let barrel = ParabolicBarrel::build_barrel(4, 2, 6).expect("error");
    /// assert_eq!(54.03539364174444, barrel.get_volume());
    /// ```
    fn get_volume(self) -> f64 {
        (1.0 / 15.0) * PI * self.h * (8.0 * pow(self.R / 2.0, 2) + 4.0 * (self.R / 2.0) * (self.r / 2.0) + 3.0 * pow(self.r / 2.0, 2))
    }
}


/// Struct representing a torus in euclidean space
/// # Usage
/// ```
/// pub use mathol::geometrics::stereometry::Torus;
/// ```
#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone)]
pub struct Torus {
    /// Distance between torus centre and cirle centre
    pub R: f64,
    /// Radius of the circle
    pub r: f64,
}

impl Torus {
    #[allow(non_snake_case)]
    /// Creates a torus instance with the given radiuses
    /// # Parameters
    /// R: Distance between torus centre and cirle centre
    ///
    /// r: Radius of the circle
    /// # Return values
    /// Returns the torus instance in case of success
    ///
    /// Returns NegativeValueError if at least one of the given radiuses has a negative value
    ///
    /// Returns ComparisonError if R is smaller than r
    /// # Examples
    /// ```
    /// let torus = Torus::build_torus(8, 2).expect("error");
    /// ```
    pub fn build_torus<T>(R: T, r: T) -> Result<Torus, MatholError>
        where T: Num + Convert + PartialOrd
    {
        if R <= T::zero() || r <= T::zero() {
            return Err(MatholError::NegativeValueCause(NegativeValueError {
                message: "Torus must have a positive radius".to_string(),
            }));
        }
        if r >= R {
            return Err(MatholError::ComparisonCause(ComparisonError {
                message: "R must be bigger than r".to_string(),
            }));
        }

        Ok(Torus {
            R: R.to_f64(),
            r: r.to_f64(),
        })
    }
}

impl Volume for Torus {
    /// Calculates the volume of a torus
    /// # Remarks
    /// Formula for a torus is A = 2 * PI² * r² * R
    /// # Return values
    /// Returns the calculated volume as f64 value
    /// # Examples
    /// ```
    /// let torus = Torus::build_torus(8, 2).expect("error");
    /// assert_eq!(157.91367041742973, torus.get_volume());
    /// ```
    fn get_volume(self) -> f64 {
        2.0 * pow(PI, 2) * pow(self.r / 2.0, 2) * self.R
    }
}

impl Surface for Torus {
    /// Calculates the surface of a torus
    /// # Remarks
    /// Formula for a torus is S = 4 * PI² * r * R
    /// # Return values
    /// Returns the calculated surface as f64 value
    /// # Examples
    /// ```
    /// let torus = Torus::build_torus(8, 2).expect("error");
    /// assert_eq!(315.82734083485946, torus.get_surface());
    /// ```
    fn get_surface(self) -> f64 {
        4.0 * pow(PI, 2) * self.r / 2.0 * self.R
    }
}
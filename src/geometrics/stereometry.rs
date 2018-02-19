use std::f64::consts::PI;
//use basic::{Convert, pow};
use basics::pow::pow;
use basics::convert_trait::Convert;
use num::Num;
use geometrics::traits::*;
use std::cmp::PartialOrd;
use error::*;


#[derive(Debug, Copy, Clone)]
pub struct Cuboid {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Cuboid {
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
    fn get_volume(self) -> f64 {
        self.a * self.b * self.c
    }
}

impl Surface for Cuboid {
    fn get_surface(self) -> f64 {
        2.0 * (self.a * self.b + self.a * self.c + self.b * self.c)
    }
}

impl Diagonal for Cuboid {
    fn get_diagonal(self) -> f64 {
        (pow(self.a, 2) + pow(self.b, 2) + pow(self.c, 2)).sqrt()
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Pyramid {
    pub area: f64,
    pub h: f64,
}

impl Pyramid {
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
    fn get_volume(self) -> f64 {
        (1.0 / 3.0) * self.area * self.h
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Wedge {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub h: f64,
}

impl Wedge {
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
    fn get_volume(self) -> f64 {
        (1.0 / 6.0) * self.b * self.h * (2.0 * self.a + self.c)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Cylinder {
    pub r: f64,
    pub h: f64,
}

impl Cylinder {
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
    fn get_volume(self) -> f64 {
        PI * pow(self.r, 2) * self.h
    }
}

impl Surface for Cylinder {
    fn get_surface(self) -> f64 {
        2.0 * PI * self.r * (self.r + self.h)
    }
}

impl Lateral for Cylinder {
    fn get_lateral(self) -> f64 {
        2.0 * PI * self.r * self.h
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Cone {
    pub r: f64,
    pub h: f64,
}

impl Cone {
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
    fn get_volume(self) -> f64 {
        (1.0 / 3.0) * PI * pow(self.r, 2) * self.h
    }
}

impl Surface for Cone {
    fn get_surface(self) -> f64 {
        let s = (pow(self.r, 2) + (pow(self.h, 2))).sqrt();
        PI * self.r * (self.r + s)
    }
}

impl Lateral for Cone {
    fn get_lateral(self) -> f64 {
        let s = (pow(self.r, 2) + (pow(self.h, 2))).sqrt();
        PI * self.r * s
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub r: f64,
}

impl Sphere {
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
    fn get_volume(self) -> f64 {
        (4.0 / 3.0) * PI * pow(self.r, 3)
    }
}

impl Surface for Sphere {
    fn get_surface(self) -> f64 {
        4.0 * PI * pow(self.r, 2)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Ellipsoid {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Ellipsoid {
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
    fn get_volume(self) -> f64 {
        (4.0 / 3.0) * PI * self.a * self.b * self.c
    }
}


#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone)]
pub struct SphericBarrel {
    pub R: f64,
    pub r: f64,
    pub h: f64,
}

#[allow(non_snake_case)]
impl SphericBarrel {
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
    fn get_volume(self) -> f64 {
        (1.0 / 3.0) * PI * self.h * (2.0 * pow(self.R / 2.0, 2) + pow(self.r / 2.0, 2))
    }
}


#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone)]
pub struct ParabolicBarrel {
    pub R: f64,
    pub r: f64,
    pub h: f64,
}

#[allow(non_snake_case)]
impl ParabolicBarrel {
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
    fn get_volume(self) -> f64 {
        (1.0 / 15.0) * PI * self.h * (8.0 * pow(self.R / 2.0, 2) + 4.0 * (self.R / 2.0) * (self.r / 2.0) + 3.0 * pow(self.r / 2.0, 2))
    }
}


#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone)]
pub struct Torus {
    pub R: f64,
    pub r: f64,
}

impl Torus {
    #[allow(non_snake_case)]
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
    fn get_volume(self) -> f64 {
        2.0 * pow(PI, 2) * pow(self.r / 2.0, 2) * self.R
    }
}

impl Surface for Torus {
    fn get_surface(self) -> f64 {
        4.0 * pow(PI, 2) * self.r / 2.0 * self.R
    }
}
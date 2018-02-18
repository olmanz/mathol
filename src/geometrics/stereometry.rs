use std::f64::consts::PI;
use basic::{Convert, pow};
use num::Num;
use geometrics::traits::*;


pub struct Cuboid {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Cuboid {
    pub fn build_cuboid<T>(a: T, b: T, c: T) -> Cuboid
        where T: Num + Convert
    {
        Cuboid {
            a: a.to_f64(),
            b: b.to_f64(),
            c: c.to_f64(),
        }
    }
}

impl Volume for Cuboid {
    fn get_volume(&self) -> f64 {
        self.a * self.b * self.c
    }
}

impl Surface for Cuboid {
    fn get_surface(&self) -> f64 {
        2.0 * (self.a * self.b + self.a * self.c + self.b * self.c)
    }
}

impl Diagonal for Cuboid {
    fn get_diagonal(&self) -> f64 {
        (pow(self.a, 2) + pow(self.b, 2) + pow(self.c, 2)).sqrt()
    }
}


pub struct Pyramid {
    pub area: f64,
    pub h: f64,
}

impl Pyramid {
    pub fn build_pyramid<T>(area: T, h: T) -> Pyramid
        where T: Num + Convert
    {
        Pyramid {
            area: area.to_f64(),
            h: h.to_f64(),
        }
    }
}

impl Volume for Pyramid {
    fn get_volume(&self) -> f64 {
        (1.0 / 3.0) * self.area * self.h
    }
}


pub struct Wedge {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub h: f64,
}

impl Wedge {
    pub fn build_wedge<T>(a: T, b: T, c: T, h: T) -> Wedge
        where T: Num + Convert
    {
        Wedge {
            a: a.to_f64(),
            b: b.to_f64(),
            c: c.to_f64(),
            h: h.to_f64(),
        }
    }
}

impl Volume for Wedge {
    fn get_volume(&self) -> f64 {
        (1.0 / 6.0) * self.b * self.h * (2.0 * self.a + self.c)
    }
}


pub struct Cylinder {
    pub r: f64,
    pub h: f64,
}

impl Cylinder {
    pub fn build_cylinder<T>(r: T, h: T) -> Cylinder
        where T: Num + Convert
    {
        Cylinder {
            r: r.to_f64(),
            h: h.to_f64(),
        }
    }
}

impl Volume for Cylinder {
    fn get_volume(&self) -> f64 {
        PI * pow(self.r, 2) * self.h
    }
}

impl Surface for Cylinder {
    fn get_surface(&self) -> f64 {
        2.0 * PI * self.r * (self.r + self.h)
    }
}

impl Lateral for Cylinder {
    fn get_lateral(&self) -> f64 {
        2.0 * PI * self.r * self.h
    }
}


pub struct Cone {
    pub r: f64,
    pub h: f64,
}

impl Cone {
    pub fn build_cone<T>(r: T, h: T) -> Cone
        where T: Num + Convert
    {
        Cone {
            r: r.to_f64(),
            h: h.to_f64(),
        }
    }
}

impl Volume for Cone {
    fn get_volume(&self) -> f64 {
        (1.0 / 3.0) * PI * pow(self.r, 2) * self.h
    }
}

impl Surface for Cone {
    fn get_surface(&self) -> f64 {
        let s = (pow(self.r, 2) + (pow(self.h, 2))).sqrt();
        PI * self.r * (self.r + s)
    }
}

impl Lateral for Cone {
    fn get_lateral(&self) -> f64 {
        let s = (pow(self.r, 2) + (pow(self.h, 2))).sqrt();
        PI * self.r * s
    }
}


pub struct Sphere {
    pub r: f64,
}

impl Sphere {
    pub fn build_sphere<T>(r: T) -> Sphere
        where T: Num + Convert
    {
        Sphere {
            r: r.to_f64(),
        }
    }
}

impl Volume for Sphere {
    fn get_volume(&self) -> f64 {
        (4.0 / 3.0) * PI * pow(self.r, 3)
    }
}

impl Surface for Sphere {
    fn get_surface(&self) -> f64 {
        4.0 * PI * pow(self.r, 2)
    }
}


pub struct Ellipsoid {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Ellipsoid {
    pub fn build_ellipsoid<T>(a: T, b: T, c: T) -> Ellipsoid
        where T: Num + Convert
    {
        Ellipsoid {
            a: a.to_f64(),
            b: b.to_f64(),
            c: c.to_f64(),
        }
    }
}

impl Volume for Ellipsoid {
    fn get_volume(&self) -> f64 {
        (4.0 / 3.0) * PI * self.a * self.b * self.c
    }
}


#[allow(non_snake_case)]
pub struct SphericBarrel {
    pub R: f64,
    pub r: f64,
    pub h: f64,
}

#[allow(non_snake_case)]
impl SphericBarrel {
    pub fn build_barrel<T>(R: T, r: T, h: T) -> SphericBarrel
        where T: Num + Convert
    {
       SphericBarrel {
           R: R.to_f64(),
           r: r.to_f64(),
           h: h.to_f64(),
       }
    }
}

impl Volume for SphericBarrel {
    fn get_volume(&self) -> f64 {
        (1.0 / 3.0) * PI * self.h * (2.0 * pow(self.R / 2.0, 2) + pow(self.r / 2.0, 2))
    }
}


#[allow(non_snake_case)]
pub struct ParabolicBarrel {
    pub R: f64,
    pub r: f64,
    pub h: f64,
}

#[allow(non_snake_case)]
impl ParabolicBarrel {
    pub fn build_barrel<T>(R: T, r: T, h: T) -> ParabolicBarrel
        where T: Num + Convert
    {
        ParabolicBarrel {
            R: R.to_f64(),
            r: r.to_f64(),
            h: h.to_f64(),
        }
    }
}

impl Volume for ParabolicBarrel {
    fn get_volume(&self) -> f64 {
        (1.0 / 15.0) * PI * self.h * (8.0 * pow(self.R / 2.0, 2) + 4.0 * (self.R / 2.0) * (self.r / 2.0) + 3.0 * pow(self.r / 2.0, 2))
    }
}


#[allow(non_snake_case)]
pub struct Torus {
    pub R: f64,
    pub r: f64,
}

impl Torus {
    #[allow(non_snake_case)]
    pub fn build_torus<T>(R: T, r: T) -> Torus
        where T: Num + Convert
    {
        Torus {
            R: R.to_f64(),
            r: r.to_f64(),
        }
    }
}

impl Volume for Torus {
    fn get_volume(&self) -> f64 {
        2.0 * pow(PI, 2) * pow(self.r / 2.0, 2) * self.R
    }
}

impl Surface for Torus {
    fn get_surface(&self) -> f64 {
        4.0 * pow(PI, 2) * self.r / 2.0 * self.R
    }
}
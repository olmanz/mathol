use std::f64::consts::PI;
use basics::pow::pow;
use basics::convert_trait::Convert;
use basics::amount_trait::Amount;
use basics::cotangent::Cotangent;
use std::ops::Add;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use vectoroperations::vector2d::Vector2D;
use num::{Num, FromPrimitive};
use geometrics::traits::{Area, Perimeter, Height, Diagonal};

#[allow(non_snake_case)]
pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Triangle
{
    pub fn build_triangle_with_edges<T>(a: T, b: T, c: T) -> Result<Triangle, &'static str>
        where T: Num + Convert + Add<Output=T> + PartialOrd + Copy
    {
        if a + b <= c || a + c <= b || b + c <= a {
            return Err("Cannot create triangle with the given edges");
        }

        let triangle = Triangle {
            a: a.to_f64(),
            b: b.to_f64(),
            c: c.to_f64(),
        };

        Ok(triangle)
    }

    #[allow(non_snake_case)]
    pub fn build_triangle_with_points<T>(A: &Vector2D<T>, B: &Vector2D<T>, C: &Vector2D<T>) -> Result<Triangle, &'static str>
        where T: Num + Convert + Add<Output=T> + PartialOrd + Copy + Amount<T> + Debug + FromPrimitive
    {
        let a = B.get_distance(C);
        let b = A.get_distance(C);
        let c = A.get_distance(B);

        Triangle::build_triangle_with_edges(a, b, c)
    }

    pub fn get_angles(&self) -> (f64, f64, f64) {
        let alpha = ((pow(self.a, 2) - pow(self.b, 2) - pow(self.c, 2)) / (-2.0 * self.b * self.c)).acos();
        let beta = ((pow(self.b, 2) - pow(self.a, 2) - pow(self.c, 2)) / (-2.0 * self.a * self.c)).acos();
        let gamma = ((pow(self.c, 2) - pow(self.a, 2) - pow(self.b, 2)) / (-2.0 * self.a * self.b)).acos();

        (alpha.to_degrees(), beta.to_degrees(), gamma.to_degrees())
    }

    pub fn get_inner_circle(&self) -> Circle {
        let s = self.get_perimeter() / 2.0;
        let r = ((s - self.a) * (s - self.b) * (s - self.c) / s).sqrt();
        Circle::build_circle(r)
    }

    pub fn get_outer_circle(&self) -> Circle {
        let s = self.get_perimeter() / 2.0;
        let r = (self.a * self.b * self.c) / (4.0 * (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt());
        Circle::build_circle(r)
    }
}

impl Area for Triangle {
    fn get_area(&self) -> f64 {
        let s = self.get_perimeter() / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
}

impl Perimeter for Triangle {
    fn get_perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

impl Height for Triangle {
    fn get_height(&self) -> f64 {
        self.a * self.get_angles().1.to_radians().sin()
    }
}


pub struct Rectangle {
    pub a: f64,
    pub b: f64,
}

impl Rectangle {
    pub fn build_rectangle<T>(a: T, b: T) -> Rectangle
        where T: Num + Convert
    {
        Rectangle {
            a: a.to_f64(),
            b: b.to_f64(),
        }
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f64 {
        self.a * self.b
    }
}

impl Perimeter for Rectangle {
    fn get_perimeter(&self) -> f64 {
        2.0 * self.a + 2.0 * self.b
    }
}

impl Diagonal for Rectangle {
    fn get_diagonal(&self) -> f64 {
        ((pow(self.a, 2)) + pow(self.b, 2)).sqrt()
    }
}


pub struct Parallelogram {
    pub a: f64,
    pub b: f64,
    pub h: f64,
}

impl Parallelogram {
    pub fn build_parallelogram<T>(a: T, b: T, h: T) -> Parallelogram
        where T: Num + Convert
    {
        Parallelogram {
            a: a.to_f64(),
            b: b.to_f64(),
            h: h.to_f64(),
        }
    }
}

impl Area for Parallelogram {
    fn get_area(&self) -> f64 {
        self.a * self.h
    }
}

impl Perimeter for Parallelogram {
    fn get_perimeter(&self) -> f64 {
        2.0 * self.a + 2.0 * self.b
    }
}


pub struct Trapeze {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

impl Trapeze {
    pub fn build_trapeze<T>(a: T, b: T, c: T, d: T) -> Trapeze
        where T: Num + Convert
    {
        Trapeze {
            a: a.to_f64(),
            b: b.to_f64(),
            c: c.to_f64(),
            d: d.to_f64(),
        }
    }
}

impl Area for Trapeze {
    fn get_area(&self) -> f64 {
        0.5 * (self.a + self.b) * self.get_height()
    }
}

impl Perimeter for Trapeze {
    fn get_perimeter(&self) ->f64 {
        self.a + self.b + self.c + self.d
    }
}

impl Height for Trapeze {
    fn get_height(&self) -> f64 {
        ((self.a + self.d - self.b + self.c) * (-self.a + self.d + self.b + self.c) *
            (-self.a - self.d + self.b + self.c) * (-self.a + self.d + self.b - self.c)).sqrt() /
            (2.0 * (self.a - self.b))
    }
}


pub struct Polygon {
    pub a: f64,
    pub n: f64,
}

impl Polygon {
    pub fn build_polygon<T>(a: T, n: T) -> Polygon
        where T: Num + Convert
    {
        Polygon {
            a: a.to_f64(),
            n: n.to_f64(),
        }
    }

    pub fn get_radius(&self) -> f64 {
        let basis_area = self.get_area();
        let m = 0.5 * (16.0 * pow(basis_area / self.n.to_f64(), 2) / pow(self.a, 2) + pow(self.a, 2)).sqrt();
        m
    }
}

impl Area for Polygon {
    fn get_area(&self) -> f64 {
        0.25 * self.n * pow(self.a, 2) * (PI / self.n).cot()
    }
}

impl Perimeter for Polygon {
    fn get_perimeter(&self) -> f64 {
        self.a * self.n
    }
}


pub struct Circle {
    pub r: f64,
}

impl Circle {
    pub fn build_circle<T>(r: T) -> Circle
        where T: Num + Convert
    {
        Circle {
            r: r.to_f64(),
        }
    }
}

impl Area for Circle {
    fn get_area(&self) -> f64 {
        PI * self.r * self.r
    }
}

impl Perimeter for Circle {
    fn get_perimeter(&self) -> f64 {
        2.0 * PI * self.r
    }
}


pub struct Ellipsis {
    pub a: f64,
    pub b: f64,
}

impl Ellipsis {
    pub fn build_ellipsis<T>(a: T, b: T) -> Ellipsis
        where T: Num + Convert
    {
        Ellipsis {
            a: a.to_f64(),
            b: b.to_f64(),
        }
    }
}

impl Area for Ellipsis {
    fn get_area(&self) -> f64 {
        PI * self.a * self.b
    }
}

impl Perimeter for Ellipsis {
    fn get_perimeter(&self) -> f64 {
        PI * (1.5 * (self.a + self.b) - (self.a * self.b).sqrt())
    }
}